use anyhow::Result;
use crossterm::style::Color;
use crossterm::terminal;
use std::io::{self, Write};
use std::time::{Duration, Instant};

use crate::camera::Camera;
use crate::effects::{VisualState, hex_to_color};
use crate::effects::themes::Theme;
use crate::parser::AdofaiLevel;
use crate::renderer::pipeline::RenderPipeline;
use crate::renderer::particles::ParticleStyle;
use crate::renderer::unicode_pixel::UnicodePixelEngine;
use crate::replay::ReplayRecorder;
use super::tile::Tile;
use super::planet::PlanetSystem;
use super::timing::{TimingEngine, HitAccuracy};
use super::input::{InputHandler, GameInput};

pub struct GameState {
    pub level: AdofaiLevel,
    pub tiles: Vec<Tile>,
    pub planets: PlanetSystem,
    pub timing: TimingEngine,
    pub camera: Camera,
    pub visual: VisualState,
    pub pipeline: RenderPipeline,
    pub input: InputHandler,
    pub recorder: ReplayRecorder,
    pub current_tile: usize,
    pub score: u64,
    pub combo: u32,
    pub max_combo: u32,
    pub hits: Vec<HitAccuracy>,
    pub auto_play: bool,
    pub running: bool,
    pub paused: bool,
    pub last_hit_label: String,
    pub last_hit_color: Color,
    pub last_hit_timer: f64,
    pub show_hud: bool,
    pub theme: Option<Theme>,
    pub start_tile: usize,
}

impl GameState {
    pub fn new(level: AdofaiLevel, start: usize, auto_play: bool, theme: Option<String>) -> Result<Self> {
        let (width, height) = terminal::size()?;
        let angles = level.get_angles();

        let track_color = hex_to_color(&level.settings.track_color);
        let secondary = hex_to_color(&level.settings.secondary_track_color);
        let tiles = Tile::build_tiles(&angles, track_color, secondary);

        let mut timing = TimingEngine::new(level.settings.bpm, level.settings.offset);
        timing.compute_tile_times(&angles);

        let mut visual = VisualState::default();
        visual.bg_color = hex_to_color(&level.settings.bg_color);
        visual.track_color = track_color;
        visual.secondary_track_color = secondary;

        let theme_obj = theme.and_then(|name| Theme::get(&name));
        if let Some(ref t) = theme_obj {
            t.apply_to_visual_state(&mut visual);
        }

        let mut pipeline = RenderPipeline::new(width, height);
        if let Some(ref t) = theme_obj {
            pipeline.post_processor.scanlines_enabled = t.scanlines;
            pipeline.post_processor.bloom_enabled = t.bloom;
            pipeline.post_processor.crt_enabled = t.crt;
            pipeline.post_processor.vignette_enabled = t.vignette;
        }

        Ok(Self {
            level,
            tiles,
            planets: PlanetSystem::new(),
            timing,
            camera: Camera::default(),
            visual,
            pipeline,
            input: InputHandler::new(true),
            recorder: ReplayRecorder::new(),
            current_tile: start,
            score: 0,
            combo: 0,
            max_combo: 0,
            hits: Vec::new(),
            auto_play,
            running: true,
            paused: false,
            last_hit_label: String::new(),
            last_hit_color: Color::White,
            last_hit_timer: 0.0,
            show_hud: true,
            theme: theme_obj,
            start_tile: start,
        })
    }

    pub fn run(&mut self) -> Result<()> {
        use crossterm::{execute, cursor};

        let mut stdout = io::stdout();
        crossterm::terminal::enable_raw_mode()?;
        execute!(stdout, crossterm::terminal::EnterAlternateScreen, cursor::Hide)?;
        if self.input.mouse_enabled {
            execute!(stdout, crossterm::event::EnableMouseCapture)?;
        }

        self.timing.start();
        self.camera.look_at(self.tiles[0].x, self.tiles[0].y);
        self.camera.set_zoom(self.level.settings.zoom);

        let target_fps = 120.0;
        let frame_duration = Duration::from_secs_f64(1.0 / target_fps);
        let mut last_frame = Instant::now();

        while self.running {
            let now = Instant::now();
            let dt = now.duration_since(last_frame).as_secs_f64();
            last_frame = now;

            self.handle_input()?;
            if !self.paused {
                self.update(dt);
                self.render(&mut stdout)?;
            }

            let elapsed = now.elapsed();
            if elapsed < frame_duration {
                std::thread::sleep(frame_duration - elapsed);
            }
        }

        if self.input.mouse_enabled {
            execute!(stdout, crossterm::event::DisableMouseCapture)?;
        }
        execute!(stdout, cursor::Show, crossterm::terminal::LeaveAlternateScreen)?;
        crossterm::terminal::disable_raw_mode()?;

        self.print_results();
        Ok(())
    }

    fn handle_input(&mut self) -> Result<()> {
        if let Some(input) = self.input.poll(Duration::from_millis(1)) {
            match input {
                GameInput::Hit => {
                    if !self.auto_play {
                        self.process_hit();
                    }
                }
                GameInput::Pause => {
                    self.paused = !self.paused;
                }
                GameInput::Quit => {
                    self.running = false;
                }
                GameInput::Restart => {
                    self.current_tile = self.start_tile;
                    self.score = 0;
                    self.combo = 0;
                    self.hits.clear();
                    self.timing.start();
                }
                GameInput::ToggleAutoPlay => {
                    self.auto_play = !self.auto_play;
                }
                _ => {}
            }
        }
        Ok(())
    }

    fn process_hit(&mut self) {
        if self.current_tile >= self.tiles.len() { return; }

        self.recorder.record_hit(self.timing.elapsed, self.current_tile);

        if let Some(accuracy) = self.timing.get_hit_accuracy(self.current_tile) {
            self.last_hit_label = accuracy.label().to_string();
            self.last_hit_color = accuracy.color();
            self.last_hit_timer = 0.5;

            let score = accuracy.score();
            self.score += score as u64;

            if accuracy != HitAccuracy::Miss {
                self.combo += 1;
                if self.combo > self.max_combo {
                    self.max_combo = self.combo;
                }

                // Particles on hit
                let tile = &self.tiles[self.current_tile];
                let (sx, sy) = self.pipeline.world_to_screen(tile.x, tile.y, &self.camera);
                self.pipeline.particles.emit(sx as f64, sy as f64, 5, ParticleStyle::Hit);

                if accuracy == HitAccuracy::Perfect {
                    self.pipeline.particles.emit(sx as f64, sy as f64, 8, ParticleStyle::Spark);
                }
            } else {
                self.combo = 0;
            }

            self.hits.push(accuracy);
            self.advance_tile();
        }
    }

    fn advance_tile(&mut self) {
        self.current_tile += 1;

        if self.current_tile >= self.tiles.len() {
            self.running = false;
            return;
        }

        let tile = &self.tiles[self.current_tile];

        // Process events
        for event in &tile.events {
            match event {
                super::tile::TileEvent::SetSpeed { bpm, multiplier } => {
                    if let Some(b) = bpm { self.timing.set_bpm(*b); }
                    if let Some(m) = multiplier { self.timing.set_bpm(self.timing.bpm * m); }
                }
                super::tile::TileEvent::Twirl => { self.planets.twirl(); }
                super::tile::TileEvent::Flash { duration, color } => {
                    self.visual.trigger_flash(*duration, *color, (0, 0, 0));
                }
                super::tile::TileEvent::ShakeScreen { strength, .. } => {
                    self.camera.shake(*strength, 5.0);
                }
                super::tile::TileEvent::MoveCamera { duration, position, rotation, zoom, ease } => {
                    self.camera.move_event(*duration, *position, *rotation, *zoom, ease);
                }
                super::tile::TileEvent::Bloom { enabled, intensity } => {
                    self.pipeline.post_processor.bloom_enabled = *enabled;
                    self.pipeline.post_processor.bloom_intensity = *intensity;
                }
                _ => {}
            }
        }

        // Camera follow
        self.camera.look_at(tile.x, tile.y);

        // Planet advance
        if self.current_tile < self.tiles.len() {
            let next_angle = if self.current_tile + 1 < self.tiles.len() {
                self.tiles[self.current_tile + 1].angle
            } else { 0.0 };
            self.planets.advance_tile(tile.x, tile.y, next_angle);
        }
    }

    fn update(&mut self, dt: f64) {
        self.timing.update();
        self.planets.update(dt, self.timing.bpm);
        self.camera.update(dt);
        self.visual.update(dt, self.timing.bpm);
        self.pipeline.update(dt);

        // Auto-play
        if self.auto_play && self.current_tile < self.tiles.len() {
            if self.current_tile < self.timing.tile_times.len() {
                let target_time = self.timing.tile_times[self.current_tile];
                if self.timing.elapsed >= target_time {
                    self.process_hit_auto();
                }
            }
        }

        // Update post-processing from visual state
        if let Some((color, opacity)) = self.visual.get_flash_color() {
            self.pipeline.post_processor.flash_color = Some(color);
            self.pipeline.post_processor.flash_opacity = opacity;
        } else {
            self.pipeline.post_processor.flash_opacity = 0.0;
        }

        let (sx, sy) = self.camera.get_shake_offset();
        self.pipeline.post_processor.shake_x = sx;
        self.pipeline.post_processor.shake_y = sy;

        if self.last_hit_timer > 0.0 {
            self.last_hit_timer -= dt;
        }

        // Resize check
        if let Ok((w, h)) = terminal::size() {
            if w != self.pipeline.fb.width || h != self.pipeline.fb.height {
                self.pipeline.fb.resize(w, h);
            }
        }
    }

    fn process_hit_auto(&mut self) {
        self.last_hit_label = "AUTO".to_string();
        self.last_hit_color = Color::Rgb { r: 150, g: 150, b: 150 };
        self.last_hit_timer = 0.3;
        self.hits.push(HitAccuracy::Perfect);
        self.score += 100;
        self.combo += 1;
        if self.combo > self.max_combo { self.max_combo = self.combo; }

        let tile = &self.tiles[self.current_tile];
        let (sx, sy) = self.pipeline.world_to_screen(tile.x, tile.y, &self.camera);
        self.pipeline.particles.emit(sx as f64, sy as f64, 3, ParticleStyle::Trail);

        self.advance_tile();
    }

    fn render(&mut self, stdout: &mut io::Stdout) -> Result<()> {
        self.pipeline.begin_frame(self.visual.bg_color);

        let camera = self.camera.clone();
        let beat_intensity = self.visual.get_beat_intensity();

        // Draw tracks
        let visible_start = self.current_tile.saturating_sub(10);
        let visible_end = (self.current_tile + 30).min(self.tiles.len());

        for i in visible_start..visible_end {
            let tile = &self.tiles[i];
            let (sx, sy) = self.pipeline.world_to_screen(tile.x, tile.y, &camera);

            // Draw connector to next tile
            if i + 1 < self.tiles.len() {
                let next = &self.tiles[i + 1];
                let (nx, ny) = self.pipeline.world_to_screen(next.x, next.y, &camera);
                let connector = tile.get_connector_to(next);

                let mid_x = (sx + nx) / 2;
                let mid_y = (sy + ny) / 2;
                let color = if i < self.current_tile {
                    dim_color(tile.color, 0.3)
                } else {
                    tile.color
                };
                self.pipeline.fb.set_char(mid_x, mid_y, connector, color, Color::Reset);
            }

            // Draw tile
            let ch = tile.get_visual_char();
            let active = i == self.current_tile;
            let color = if i < self.current_tile {
                dim_color(tile.color, 0.4)
            } else if active {
                brighten_color(tile.color, beat_intensity)
            } else {
                tile.color
            };

            self.pipeline.draw_tile_at_screen(sx, sy, ch, color, Color::Reset, active);

            // Draw twirl indicator
            if tile.has_twirl {
                self.pipeline.fb.set_char(sx, sy - 1, '↺', Color::Cyan, Color::Reset);
            }
        }

        // Draw planets
        let positions = self.planets.get_planet_positions();
        for (i, (px, py)) in positions.iter().enumerate() {
            let (sx, sy) = self.pipeline.world_to_screen(*px, *py, &camera);
            let color = if i == 0 {
                self.visual.planet_color_1
            } else {
                self.visual.planet_color_2
            };

            let planet_char = if i == self.planets.active_planet { '◉' } else { '○' };
            self.pipeline.fb.set_char(sx, sy, planet_char, color, Color::Reset);
        }

        // Draw particles
        self.pipeline.draw_particles();

        // Apply post-processing
        self.pipeline.apply_post_processing();

        // Draw HUD
        if self.show_hud {
            self.draw_hud();
        }

        self.pipeline.fb.render_diff(stdout)?;
        Ok(())
    }

    fn draw_hud(&mut self) {
        let w = self.pipeline.fb.width as i32;
        let h = self.pipeline.fb.height as i32;
        let hud_bg = Color::Rgb { r: 10, g: 10, b: 15 };
        let hud_fg = Color::Rgb { r: 180, g: 180, b: 200 };

        // Top bar
        let title = format!(" {} - {} ", self.level.settings.song, self.level.settings.artist);
        let bpm_str = format!(" BPM:{:.0} ", self.timing.bpm);
        let tile_str = format!(" Tile:{}/{} ", self.current_tile, self.tiles.len());

        self.pipeline.fb.set_str(0, 0, &title, Color::White, hud_bg);
        self.pipeline.fb.set_str(w - bpm_str.len() as i32, 0, &bpm_str, Color::Yellow, hud_bg);
        self.pipeline.fb.set_str(w - bpm_str.len() as i32 - tile_str.len() as i32, 0, &tile_str, hud_fg, hud_bg);

        // Bottom bar
        let score_str = format!(" Score:{} ", self.score);
        let combo_str = format!(" Combo:{} ", self.combo);
        let mode_str = if self.auto_play { " [AUTO] " } else { " [PLAY] " };

        self.pipeline.fb.set_str(0, h - 1, &score_str, Color::White, hud_bg);
        self.pipeline.fb.set_str(score_str.len() as i32, h - 1, &combo_str, Color::Cyan, hud_bg);
        self.pipeline.fb.set_str(w - mode_str.len() as i32, h - 1, mode_str, Color::Green, hud_bg);

        // Progress bar
        let progress = if !self.tiles.is_empty() {
            self.current_tile as f64 / self.tiles.len() as f64
        } else { 0.0 };
        UnicodePixelEngine::draw_progress_bar(
            &mut self.pipeline.fb,
            0, h - 2,
            w, progress,
            Color::Rgb { r: 100, g: 200, b: 255 },
            Color::Rgb { r: 30, g: 30, b: 40 },
        );

        // Hit accuracy label
        if self.last_hit_timer > 0.0 {
            let label = &self.last_hit_label;
            let x = w / 2 - label.len() as i32 / 2;
            let y = h / 2 + 3;
            self.pipeline.fb.set_str_bold(x, y, label, self.last_hit_color, Color::Reset);
        }

        // Pause overlay
        if self.paused {
            let pause_text = "║ PAUSED ║";
            let x = w / 2 - pause_text.len() as i32 / 2;
            let y = h / 2;
            self.pipeline.fb.set_str_bold(x, y, pause_text, Color::White, Color::Rgb { r: 40, g: 40, b: 50 });

            let help = "[P] Resume  [R] Restart  [Q] Quit";
            let hx = w / 2 - help.len() as i32 / 2;
            self.pipeline.fb.set_str(hx, y + 2, help, Color::Rgb { r: 150, g: 150, b: 150 }, Color::Reset);
        }
    }

    fn print_results(&self) {
        println!("\n╭─────────────────────────────────╮");
        println!("│         LEVEL COMPLETE          │");
        println!("├─────────────────────────────────┤");
        println!("│ Song: {:25} │", self.level.settings.song);
        println!("│ Score: {:24} │", self.score);
        println!("│ Max Combo: {:20} │", self.max_combo);

        let total = self.hits.len();
        if total > 0 {
            let perfects = self.hits.iter().filter(|h| **h == HitAccuracy::Perfect).count();
            let greats = self.hits.iter().filter(|h| **h == HitAccuracy::Great).count();
            let goods = self.hits.iter().filter(|h| **h == HitAccuracy::Good).count();
            let misses = self.hits.iter().filter(|h| **h == HitAccuracy::Miss).count();

            println!("│ Perfect: {:22} │", perfects);
            println!("│ Great:   {:22} │", greats);
            println!("│ Good:    {:22} │", goods);
            println!("│ Miss:    {:22} │", misses);

            let accuracy = (perfects * 100 + greats * 80 + goods * 60) as f64 / (total as f64);
            println!("│ Accuracy: {:19.1}% │", accuracy);
        }

        println!("╰─────────────────────────────────╯");
    }
}

fn dim_color(color: Color, factor: f64) -> Color {
    match color {
        Color::Rgb { r, g, b } => Color::Rgb {
            r: (r as f64 * factor) as u8,
            g: (g as f64 * factor) as u8,
            b: (b as f64 * factor) as u8,
        },
        other => other,
    }
}

fn brighten_color(color: Color, intensity: f64) -> Color {
    match color {
        Color::Rgb { r, g, b } => {
            let boost = (intensity * 50.0) as u8;
            Color::Rgb {
                r: r.saturating_add(boost),
                g: g.saturating_add(boost),
                b: b.saturating_add(boost),
            }
        },
        other => other,
    }
}
