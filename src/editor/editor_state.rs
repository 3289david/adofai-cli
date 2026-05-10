use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyModifiers, MouseEvent, MouseEventKind};
use crossterm::style::Color;
use crossterm::terminal;
use std::io::{self, Write};
use std::path::PathBuf;
use std::time::{Duration, Instant};

use crate::camera::Camera;
use crate::effects::{VisualState, hex_to_color};
use crate::engine::tile::Tile;
use crate::parser::AdofaiLevel;
use crate::renderer::framebuffer::FrameBuffer;
use crate::renderer::unicode_pixel::UnicodePixelEngine;
use super::undo::UndoTree;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EditorMode {
    Normal,
    Insert,
    Command,
    EventEdit,
    Timeline,
    Preview,
}

pub struct EditorState {
    pub level: AdofaiLevel,
    pub file_path: PathBuf,
    pub mode: EditorMode,
    pub cursor_tile: usize,
    pub camera: Camera,
    pub visual: VisualState,
    pub fb: FrameBuffer,
    pub undo_tree: UndoTree,
    pub command_buffer: String,
    pub status_message: String,
    pub status_timer: f64,
    pub tiles: Vec<Tile>,
    pub running: bool,
    pub dirty: bool,
    pub scroll_offset: f64,
    pub timeline_scroll: usize,
    pub show_grid: bool,
    pub show_events: bool,
    pub selected_event: Option<usize>,
}

impl EditorState {
    pub fn new(level: AdofaiLevel, file_path: PathBuf) -> Result<Self> {
        let (width, height) = terminal::size()?;
        let angles = level.get_angles();
        let track_color = hex_to_color(&level.settings.track_color);
        let secondary = hex_to_color(&level.settings.secondary_track_color);
        let tiles = Tile::build_tiles(&angles, track_color, secondary);
        let undo_tree = UndoTree::new(&level);

        let mut visual = VisualState::default();
        visual.bg_color = hex_to_color(&level.settings.bg_color);

        Ok(Self {
            level,
            file_path,
            mode: EditorMode::Normal,
            cursor_tile: 0,
            camera: Camera::default(),
            visual,
            fb: FrameBuffer::new(width, height),
            undo_tree,
            command_buffer: String::new(),
            status_message: String::new(),
            status_timer: 0.0,
            tiles,
            running: true,
            dirty: false,
            scroll_offset: 0.0,
            timeline_scroll: 0,
            show_grid: true,
            show_events: true,
            selected_event: None,
        })
    }

    pub fn run(&mut self) -> Result<()> {
        use crossterm::{execute, cursor};

        let mut stdout = io::stdout();
        crossterm::terminal::enable_raw_mode()?;
        execute!(stdout, crossterm::terminal::EnterAlternateScreen, cursor::Hide, crossterm::event::EnableMouseCapture)?;

        if !self.tiles.is_empty() {
            self.camera.look_at(self.tiles[0].x, self.tiles[0].y);
        }
        self.camera.set_zoom(150.0);

        let mut last_frame = Instant::now();

        while self.running {
            let now = Instant::now();
            let dt = now.duration_since(last_frame).as_secs_f64();
            last_frame = now;

            self.handle_input()?;
            self.update(dt);
            self.render(&mut stdout)?;

            let frame_time = now.elapsed();
            let target = Duration::from_secs_f64(1.0 / 60.0);
            if frame_time < target {
                std::thread::sleep(target - frame_time);
            }
        }

        execute!(stdout, crossterm::event::DisableMouseCapture, cursor::Show, crossterm::terminal::LeaveAlternateScreen)?;
        crossterm::terminal::disable_raw_mode()?;

        Ok(())
    }

    fn handle_input(&mut self) -> Result<()> {
        if !event::poll(Duration::from_millis(5))? {
            return Ok(());
        }

        let event = event::read()?;

        match self.mode {
            EditorMode::Command => self.handle_command_input(event),
            EditorMode::Normal => self.handle_normal_input(event),
            EditorMode::Insert => self.handle_insert_input(event),
            EditorMode::Timeline => self.handle_timeline_input(event),
            EditorMode::Preview => self.handle_preview_input(event),
            EditorMode::EventEdit => self.handle_event_input(event),
        }
    }

    fn handle_normal_input(&mut self, event: Event) -> Result<()> {
        if let Event::Key(key) = event {
            if key.modifiers.contains(KeyModifiers::CONTROL) {
                match key.code {
                    KeyCode::Char('s') => self.save()?,
                    KeyCode::Char('z') => self.undo(),
                    KeyCode::Char('y') => self.redo(),
                    KeyCode::Char('c') | KeyCode::Char('q') => self.running = false,
                    _ => {}
                }
                return Ok(());
            }

            match key.code {
                KeyCode::Char('q') => self.running = false,
                KeyCode::Char('h') | KeyCode::Left => self.move_cursor(-1),
                KeyCode::Char('l') | KeyCode::Right => self.move_cursor(1),
                KeyCode::Char('j') | KeyCode::Down => self.move_cursor(5),
                KeyCode::Char('k') | KeyCode::Up => self.move_cursor(-5),
                KeyCode::Char('0') | KeyCode::Home => { self.cursor_tile = 0; self.update_camera_to_cursor(); }
                KeyCode::Char('$') | KeyCode::End => {
                    self.cursor_tile = self.tiles.len().saturating_sub(1);
                    self.update_camera_to_cursor();
                }
                KeyCode::Char('i') => self.mode = EditorMode::Insert,
                KeyCode::Char(':') => {
                    self.mode = EditorMode::Command;
                    self.command_buffer.clear();
                }
                KeyCode::Char('a') => self.add_tile(),
                KeyCode::Char('d') | KeyCode::Delete => self.delete_tile(),
                KeyCode::Char('r') => self.rotate_tile(45.0),
                KeyCode::Char('R') => self.rotate_tile(-45.0),
                KeyCode::Char('t') => self.toggle_twirl(),
                KeyCode::Char('e') => self.mode = EditorMode::EventEdit,
                KeyCode::Char('T') => self.mode = EditorMode::Timeline,
                KeyCode::Char(' ') | KeyCode::Tab => self.mode = EditorMode::Preview,
                KeyCode::Char('g') => self.show_grid = !self.show_grid,
                KeyCode::Char('+') | KeyCode::Char('=') => self.camera.set_zoom(self.camera.target_zoom + 10.0),
                KeyCode::Char('-') => self.camera.set_zoom((self.camera.target_zoom - 10.0).max(20.0)),
                KeyCode::Char('u') => self.undo(),
                _ => {}
            }
        }

        if let Event::Mouse(mouse) = event {
            self.handle_mouse(mouse);
        }

        Ok(())
    }

    fn handle_command_input(&mut self, event: Event) -> Result<()> {
        if let Event::Key(key) = event {
            match key.code {
                KeyCode::Enter => {
                    self.execute_command();
                    self.mode = EditorMode::Normal;
                }
                KeyCode::Esc => {
                    self.mode = EditorMode::Normal;
                    self.command_buffer.clear();
                }
                KeyCode::Backspace => { self.command_buffer.pop(); }
                KeyCode::Char(c) => { self.command_buffer.push(c); }
                _ => {}
            }
        }
        Ok(())
    }

    fn handle_insert_input(&mut self, event: Event) -> Result<()> {
        if let Event::Key(key) = event {
            match key.code {
                KeyCode::Esc => self.mode = EditorMode::Normal,
                KeyCode::Char('r') => { self.add_tile_with_angle(0.0); }   // Right
                KeyCode::Char('u') => { self.add_tile_with_angle(90.0); }  // Up
                KeyCode::Char('l') => { self.add_tile_with_angle(180.0); } // Left
                KeyCode::Char('d') => { self.add_tile_with_angle(270.0); } // Down
                KeyCode::Char('e') => { self.add_tile_with_angle(45.0); }  // Up-right
                KeyCode::Char('q') => { self.add_tile_with_angle(135.0); } // Up-left
                KeyCode::Char('z') => { self.add_tile_with_angle(225.0); } // Down-left
                KeyCode::Char('c') => { self.add_tile_with_angle(315.0); } // Down-right
                _ => {}
            }
        }
        Ok(())
    }

    fn handle_timeline_input(&mut self, event: Event) -> Result<()> {
        if let Event::Key(key) = event {
            match key.code {
                KeyCode::Esc | KeyCode::Char('T') => self.mode = EditorMode::Normal,
                KeyCode::Left => { if self.timeline_scroll > 0 { self.timeline_scroll -= 1; } }
                KeyCode::Right => { self.timeline_scroll += 1; }
                _ => {}
            }
        }
        Ok(())
    }

    fn handle_preview_input(&mut self, event: Event) -> Result<()> {
        if let Event::Key(key) = event {
            match key.code {
                KeyCode::Esc | KeyCode::Tab => self.mode = EditorMode::Normal,
                _ => {}
            }
        }
        Ok(())
    }

    fn handle_event_input(&mut self, event: Event) -> Result<()> {
        if let Event::Key(key) = event {
            match key.code {
                KeyCode::Esc => self.mode = EditorMode::Normal,
                KeyCode::Char('b') => {
                    self.mode = EditorMode::Command;
                    self.command_buffer = "bpm ".into();
                }
                KeyCode::Char('t') => self.toggle_twirl(),
                KeyCode::Char('f') => self.add_flash_event(),
                KeyCode::Char('c') => {
                    self.mode = EditorMode::Command;
                    self.command_buffer = "camera ".into();
                }
                KeyCode::Char('s') => self.add_shake_event(),
                _ => {}
            }
        }
        Ok(())
    }

    fn handle_mouse(&mut self, mouse: MouseEvent) {
        match mouse.kind {
            MouseEventKind::ScrollUp => self.camera.set_zoom(self.camera.target_zoom + 5.0),
            MouseEventKind::ScrollDown => self.camera.set_zoom((self.camera.target_zoom - 5.0).max(20.0)),
            MouseEventKind::Down(_) => {
                // Find closest tile to click position
                let click_x = mouse.column as i32;
                let click_y = mouse.row as i32;
                let mut best_dist = i32::MAX;
                let mut best_tile = self.cursor_tile;

                for (i, tile) in self.tiles.iter().enumerate() {
                    let zoom = self.camera.zoom / 100.0;
                    let sx = ((tile.x - self.camera.x) * zoom) as i32 + self.fb.width as i32 / 2;
                    let sy = ((tile.y - self.camera.y) * zoom) as i32 + self.fb.height as i32 / 2;
                    let dist = (sx - click_x).abs() + (sy - click_y).abs();
                    if dist < best_dist {
                        best_dist = dist;
                        best_tile = i;
                    }
                }

                if best_dist < 5 {
                    self.cursor_tile = best_tile;
                    self.update_camera_to_cursor();
                }
            }
            _ => {}
        }
    }

    fn execute_command(&mut self) {
        let cmd = self.command_buffer.clone();
        let parts: Vec<&str> = cmd.trim().split_whitespace().collect();

        match parts.first().copied() {
            Some("w") | Some("save") => {
                if let Err(e) = self.save() {
                    self.set_status(&format!("Error saving: {}", e));
                }
            }
            Some("q") | Some("quit") => {
                if self.dirty {
                    self.set_status("Unsaved changes! Use :q! to force quit or :wq to save and quit");
                } else {
                    self.running = false;
                }
            }
            Some("q!") => self.running = false,
            Some("wq") => {
                let _ = self.save();
                self.running = false;
            }
            Some("bpm") => {
                if let Some(val) = parts.get(1).and_then(|v| v.parse::<f64>().ok()) {
                    self.set_bpm_at_cursor(val);
                } else {
                    self.set_status(&format!("Current BPM: {:.1}", self.level.get_bpm_at(self.cursor_tile)));
                }
            }
            Some("twirl") => self.toggle_twirl(),
            Some("planet") => {
                if let Some("multi") = parts.get(1).copied() {
                    if let Some(n) = parts.get(2).and_then(|v| v.parse::<u32>().ok()) {
                        self.add_multi_planet(n);
                    }
                }
            }
            Some("title") | Some("song") => {
                if parts.len() > 1 {
                    self.level.settings.song = parts[1..].join(" ");
                    self.dirty = true;
                    self.set_status(&format!("Song: {}", self.level.settings.song));
                }
            }
            Some("artist") => {
                if parts.len() > 1 {
                    self.level.settings.artist = parts[1..].join(" ");
                    self.dirty = true;
                    self.set_status(&format!("Artist: {}", self.level.settings.artist));
                }
            }
            Some("offset") => {
                if let Some(val) = parts.get(1).and_then(|v| v.parse::<f64>().ok()) {
                    self.level.settings.offset = val;
                    self.dirty = true;
                    self.set_status(&format!("Offset: {:.0}ms", val));
                }
            }
            Some("help") => {
                self.set_status("Commands: bpm, twirl, planet multi N, song, artist, offset, w, q, wq, q!");
            }
            _ => {
                self.set_status(&format!("Unknown command: {}", cmd));
            }
        }

        self.command_buffer.clear();
    }

    fn update(&mut self, dt: f64) {
        self.camera.update(dt);
        if self.status_timer > 0.0 {
            self.status_timer -= dt;
        }

        if let Ok((w, h)) = terminal::size() {
            if w != self.fb.width || h != self.fb.height {
                self.fb.resize(w, h);
            }
        }
    }

    fn render(&mut self, stdout: &mut io::Stdout) -> Result<()> {
        self.fb.clear_with_color(self.visual.bg_color);

        let w = self.fb.width as i32;
        let h = self.fb.height as i32;
        let editor_top = 2;
        let editor_bottom = h - 6;

        // Draw grid
        if self.show_grid {
            let grid_color = Color::Rgb { r: 25, g: 25, b: 30 };
            let zoom = self.camera.zoom / 100.0;
            let grid_size = (3.0 * zoom) as i32;
            if grid_size > 1 {
                let cx = w / 2 - ((self.camera.x * zoom) as i32 % grid_size);
                let cy = h / 2 - ((self.camera.y * zoom) as i32 % grid_size);
                for x in (cx % grid_size..w).step_by(grid_size.max(1) as usize) {
                    for y in editor_top..editor_bottom {
                        self.fb.set_char(x, y, '·', grid_color, Color::Reset);
                    }
                }
            }
        }

        // Draw tiles
        for (i, tile) in self.tiles.iter().enumerate() {
            let zoom = self.camera.zoom / 100.0;
            let sx = ((tile.x - self.camera.x) * zoom) as i32 + w / 2;
            let sy = ((tile.y - self.camera.y) * zoom) as i32 + h / 2;

            if sx < -2 || sx >= w + 2 || sy < editor_top || sy >= editor_bottom {
                continue;
            }

            // Draw connector
            if i + 1 < self.tiles.len() {
                let next = &self.tiles[i + 1];
                let nx = ((next.x - self.camera.x) * zoom) as i32 + w / 2;
                let ny = ((next.y - self.camera.y) * zoom) as i32 + h / 2;
                let connector = tile.get_connector_to(next);
                let mid_x = (sx + nx) / 2;
                let mid_y = (sy + ny) / 2;
                self.fb.set_char(mid_x, mid_y, connector, tile.color, Color::Reset);
            }

            // Draw tile
            let is_selected = i == self.cursor_tile;
            let ch = tile.get_visual_char();
            let color = if is_selected {
                Color::White
            } else {
                tile.color
            };

            self.fb.set_char(sx, sy, ch, color, Color::Reset);

            if is_selected {
                self.fb.set_char(sx - 1, sy, '[', Color::Yellow, Color::Reset);
                self.fb.set_char(sx + 1, sy, ']', Color::Yellow, Color::Reset);
            }

            // Event indicators
            if self.show_events {
                let actions = self.level.get_actions_at(i);
                if !actions.is_empty() {
                    self.fb.set_char(sx, sy - 1, '▾', Color::Cyan, Color::Reset);
                }
                if tile.has_twirl {
                    self.fb.set_char(sx + 1, sy - 1, '↺', Color::Magenta, Color::Reset);
                }
            }

            // Tile index (every 10)
            if i % 10 == 0 {
                let label = format!("{}", i);
                self.fb.set_str(sx - label.len() as i32 / 2, sy + 1, &label,
                    Color::Rgb { r: 80, g: 80, b: 100 }, Color::Reset);
            }
        }

        // Title bar
        let title_bg = Color::Rgb { r: 30, g: 30, b: 45 };
        self.fb.fill_rect(0, 0, w, 2, ' ', Color::White, title_bg);

        let filename = self.file_path.file_name()
            .map(|f| f.to_string_lossy().to_string())
            .unwrap_or_default();
        let dirty_mark = if self.dirty { " [+]" } else { "" };
        let title = format!(" ADOFAI Editor - {}{} ", filename, dirty_mark);
        self.fb.set_str_bold(0, 0, &title, Color::White, title_bg);

        let mode_str = match self.mode {
            EditorMode::Normal => " NORMAL ",
            EditorMode::Insert => " INSERT ",
            EditorMode::Command => " COMMAND ",
            EditorMode::EventEdit => " EVENTS ",
            EditorMode::Timeline => " TIMELINE ",
            EditorMode::Preview => " PREVIEW ",
        };
        let mode_color = match self.mode {
            EditorMode::Normal => Color::Rgb { r: 100, g: 150, b: 255 },
            EditorMode::Insert => Color::Rgb { r: 100, g: 255, b: 100 },
            EditorMode::Command => Color::Rgb { r: 255, g: 200, b: 50 },
            EditorMode::EventEdit => Color::Rgb { r: 255, g: 100, b: 255 },
            EditorMode::Timeline => Color::Rgb { r: 255, g: 150, b: 50 },
            EditorMode::Preview => Color::Rgb { r: 50, g: 255, b: 200 },
        };
        self.fb.set_str_bold(w - mode_str.len() as i32 - 1, 0, mode_str, Color::Black, mode_color);

        let info = format!(" Tile:{}/{} BPM:{:.0} Angle:{:.0}° ",
            self.cursor_tile, self.tiles.len(),
            self.level.get_bpm_at(self.cursor_tile),
            if self.cursor_tile < self.tiles.len() { self.tiles[self.cursor_tile].angle } else { 0.0 }
        );
        self.fb.set_str(0, 1, &info, Color::Rgb { r: 180, g: 180, b: 200 }, title_bg);

        // Event panel
        let panel_top = editor_bottom;
        let panel_bg = Color::Rgb { r: 20, g: 20, b: 30 };
        self.fb.fill_rect(0, panel_top, w, h - panel_top, ' ', Color::White, panel_bg);

        self.fb.set_str(1, panel_top, " Events ", Color::Yellow, panel_bg);
        let events = self.level.get_actions_at(self.cursor_tile);
        for (ei, event) in events.iter().enumerate() {
            let y = panel_top + 1 + ei as i32;
            if y >= h - 1 { break; }
            let text = format!("  {} {}", event.event_type,
                serde_json::to_string(&event.properties).unwrap_or_default().chars().take(50).collect::<String>()
            );
            let selected = self.selected_event == Some(ei);
            let fg = if selected { Color::Yellow } else { Color::Rgb { r: 180, g: 180, b: 200 } };
            self.fb.set_str(0, y, &text, fg, panel_bg);
        }
        if events.is_empty() {
            self.fb.set_str(2, panel_top + 1, "(no events)", Color::Rgb { r: 80, g: 80, b: 100 }, panel_bg);
        }

        // Status bar
        let status_bg = Color::Rgb { r: 25, g: 25, b: 35 };
        self.fb.fill_rect(0, h - 1, w, 1, ' ', Color::White, status_bg);

        if self.mode == EditorMode::Command {
            let cmd_str = format!(":{}", self.command_buffer);
            self.fb.set_str(0, h - 1, &cmd_str, Color::White, status_bg);
        } else if self.status_timer > 0.0 {
            self.fb.set_str(1, h - 1, &self.status_message, Color::Yellow, status_bg);
        } else {
            let help = match self.mode {
                EditorMode::Normal => "a:add d:del r:rot t:twirl e:events i:insert ::cmd space:test q:quit",
                EditorMode::Insert => "r:right u:up l:left d:down e:↗ q:↖ z:↙ c:↘ ESC:back",
                EditorMode::EventEdit => "b:bpm t:twirl f:flash c:camera s:shake ESC:back",
                EditorMode::Timeline => "←→:scroll ESC:back",
                EditorMode::Preview => "ESC/TAB:back to editor",
                _ => "",
            };
            self.fb.set_str(1, h - 1, help, Color::Rgb { r: 100, g: 100, b: 120 }, status_bg);
        }

        self.fb.render_diff(stdout)?;
        Ok(())
    }

    fn move_cursor(&mut self, delta: i32) {
        let new = self.cursor_tile as i32 + delta;
        self.cursor_tile = new.clamp(0, self.tiles.len().saturating_sub(1) as i32) as usize;
        self.update_camera_to_cursor();
    }

    fn update_camera_to_cursor(&mut self) {
        if self.cursor_tile < self.tiles.len() {
            let tile = &self.tiles[self.cursor_tile];
            self.camera.look_at(tile.x, tile.y);
        }
    }

    fn add_tile(&mut self) {
        self.add_tile_with_angle(0.0);
    }

    fn add_tile_with_angle(&mut self, angle: f64) {
        self.undo_tree.push(&self.level, &format!("Add tile at angle {:.0}", angle));

        let insert_pos = if self.cursor_tile < self.level.angle_data.len() {
            self.cursor_tile + 1
        } else {
            self.level.angle_data.len()
        };

        self.level.angle_data.insert(insert_pos, angle);
        self.rebuild_tiles();
        self.cursor_tile = insert_pos;
        self.dirty = true;
        self.update_camera_to_cursor();
        self.set_status(&format!("Added tile at angle {:.0}°", angle));
    }

    fn delete_tile(&mut self) {
        if self.level.angle_data.is_empty() { return; }
        if self.cursor_tile == 0 || self.cursor_tile > self.level.angle_data.len() { return; }

        self.undo_tree.push(&self.level, "Delete tile");
        self.level.angle_data.remove(self.cursor_tile - 1);
        self.rebuild_tiles();
        if self.cursor_tile > 0 { self.cursor_tile -= 1; }
        self.dirty = true;
        self.update_camera_to_cursor();
        self.set_status("Deleted tile");
    }

    fn rotate_tile(&mut self, delta: f64) {
        if self.cursor_tile == 0 || self.cursor_tile > self.level.angle_data.len() { return; }

        self.undo_tree.push(&self.level, &format!("Rotate tile {:.0}°", delta));
        let idx = self.cursor_tile - 1;
        self.level.angle_data[idx] = (self.level.angle_data[idx] + delta) % 360.0;
        self.rebuild_tiles();
        self.dirty = true;
        self.set_status(&format!("Rotated to {:.0}°", self.level.angle_data[idx]));
    }

    fn toggle_twirl(&mut self) {
        self.undo_tree.push(&self.level, "Toggle twirl");
        let has = self.level.has_twirl_at(self.cursor_tile);
        if has {
            self.level.actions.retain(|a| !(a.floor == self.cursor_tile && a.event_type == "Twirl"));
            self.set_status("Removed twirl");
        } else {
            self.level.actions.push(crate::parser::Action {
                floor: self.cursor_tile,
                event_type: "Twirl".into(),
                properties: std::collections::HashMap::new(),
            });
            self.set_status("Added twirl");
        }
        self.rebuild_tiles();
        self.dirty = true;
    }

    fn set_bpm_at_cursor(&mut self, bpm: f64) {
        self.undo_tree.push(&self.level, &format!("Set BPM to {:.0}", bpm));

        let mut props = std::collections::HashMap::new();
        props.insert("beatsPerMinute".into(), serde_json::Value::from(bpm));
        props.insert("speedType".into(), serde_json::Value::from("Bpm"));

        self.level.actions.push(crate::parser::Action {
            floor: self.cursor_tile,
            event_type: "SetSpeed".into(),
            properties: props,
        });

        self.dirty = true;
        self.set_status(&format!("Set BPM to {:.0}", bpm));
    }

    fn add_multi_planet(&mut self, count: u32) {
        self.undo_tree.push(&self.level, &format!("Multi-planet: {}", count));

        let mut props = std::collections::HashMap::new();
        props.insert("planets".into(), serde_json::Value::from(count));

        self.level.actions.push(crate::parser::Action {
            floor: self.cursor_tile,
            event_type: "MultiPlanet".into(),
            properties: props,
        });

        self.dirty = true;
        self.set_status(&format!("Set {} planets", count));
    }

    fn add_flash_event(&mut self) {
        self.undo_tree.push(&self.level, "Add flash");

        let mut props = std::collections::HashMap::new();
        props.insert("duration".into(), serde_json::Value::from(1.0));
        props.insert("plane".into(), serde_json::Value::from("Foreground"));
        props.insert("startColor".into(), serde_json::Value::from("ffffff"));
        props.insert("startOpacity".into(), serde_json::Value::from(100));
        props.insert("endColor".into(), serde_json::Value::from("ffffff"));
        props.insert("endOpacity".into(), serde_json::Value::from(0));
        props.insert("ease".into(), serde_json::Value::from("Linear"));

        self.level.actions.push(crate::parser::Action {
            floor: self.cursor_tile,
            event_type: "Flash".into(),
            properties: props,
        });

        self.dirty = true;
        self.set_status("Added flash event");
    }

    fn add_shake_event(&mut self) {
        self.undo_tree.push(&self.level, "Add shake");

        let mut props = std::collections::HashMap::new();
        props.insert("duration".into(), serde_json::Value::from(1.0));
        props.insert("strength".into(), serde_json::Value::from(100));
        props.insert("intensity".into(), serde_json::Value::from(100));
        props.insert("fadeOut".into(), serde_json::Value::from("Enabled"));

        self.level.actions.push(crate::parser::Action {
            floor: self.cursor_tile,
            event_type: "ShakeScreen".into(),
            properties: props,
        });

        self.dirty = true;
        self.set_status("Added screen shake");
    }

    fn rebuild_tiles(&mut self) {
        let angles = self.level.get_angles();
        let track_color = hex_to_color(&self.level.settings.track_color);
        let secondary = hex_to_color(&self.level.settings.secondary_track_color);
        self.tiles = Tile::build_tiles(&angles, track_color, secondary);

        // Apply twirls
        for action in &self.level.actions {
            if action.event_type == "Twirl" && action.floor < self.tiles.len() {
                self.tiles[action.floor].has_twirl = true;
            }
        }
    }

    fn save(&mut self) -> Result<()> {
        self.level.save(&self.file_path)?;
        self.dirty = false;
        self.set_status(&format!("Saved to {}", self.file_path.display()));
        Ok(())
    }

    fn undo(&mut self) {
        if let Some(level) = self.undo_tree.undo() {
            self.level = level;
            self.rebuild_tiles();
            self.dirty = true;
            self.set_status("Undo");
        }
    }

    fn redo(&mut self) {
        if let Some(level) = self.undo_tree.redo() {
            self.level = level;
            self.rebuild_tiles();
            self.dirty = true;
            self.set_status("Redo");
        }
    }

    fn set_status(&mut self, msg: &str) {
        self.status_message = msg.to_string();
        self.status_timer = 3.0;
    }
}
