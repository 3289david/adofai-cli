use crossterm::style::Color;
use super::framebuffer::{FrameBuffer, Cell};

pub struct PostProcessor {
    pub bloom_enabled: bool,
    pub bloom_intensity: f64,
    pub vignette_enabled: bool,
    pub vignette_strength: f64,
    pub scanlines_enabled: bool,
    pub scanline_opacity: f64,
    pub crt_enabled: bool,
    pub chromatic_aberration: f64,
    pub glitch_enabled: bool,
    pub glitch_intensity: f64,
    pub flash_color: Option<(u8, u8, u8)>,
    pub flash_opacity: f64,
    pub motion_blur: f64,
    pub shake_x: f64,
    pub shake_y: f64,
}

impl Default for PostProcessor {
    fn default() -> Self {
        Self {
            bloom_enabled: false,
            bloom_intensity: 0.5,
            vignette_enabled: false,
            vignette_strength: 0.3,
            scanlines_enabled: false,
            scanline_opacity: 0.3,
            crt_enabled: false,
            chromatic_aberration: 0.0,
            glitch_enabled: false,
            glitch_intensity: 0.0,
            flash_color: None,
            flash_opacity: 0.0,
            motion_blur: 0.0,
            shake_x: 0.0,
            shake_y: 0.0,
        }
    }
}

impl PostProcessor {
    pub fn apply(&self, fb: &mut FrameBuffer) {
        if self.scanlines_enabled {
            self.apply_scanlines(fb);
        }
        if self.vignette_enabled {
            self.apply_vignette(fb);
        }
        if self.bloom_enabled {
            self.apply_bloom(fb);
        }
        if self.crt_enabled {
            self.apply_crt(fb);
        }
        if self.glitch_enabled && self.glitch_intensity > 0.0 {
            self.apply_glitch(fb);
        }
        if let Some(color) = self.flash_color {
            if self.flash_opacity > 0.0 {
                self.apply_flash(fb, color);
            }
        }
        if self.shake_x.abs() > 0.1 || self.shake_y.abs() > 0.1 {
            fb.offset_x = self.shake_x as i16;
            fb.offset_y = self.shake_y as i16;
        } else {
            fb.offset_x = 0;
            fb.offset_y = 0;
        }
    }

    fn apply_scanlines(&self, fb: &mut FrameBuffer) {
        for y in (0..fb.height).step_by(2) {
            for x in 0..fb.width {
                let idx = (y as usize) * (fb.width as usize) + (x as usize);
                fb.cells[idx].dim = true;
            }
        }
    }

    fn apply_vignette(&self, fb: &mut FrameBuffer) {
        let cx = fb.width as f64 / 2.0;
        let cy = fb.height as f64 / 2.0;
        let max_dist = (cx * cx + cy * cy).sqrt();

        for y in 0..fb.height {
            for x in 0..fb.width {
                let dx = x as f64 - cx;
                let dy = y as f64 - cy;
                let dist = (dx * dx + dy * dy).sqrt() / max_dist;
                let factor = dist * self.vignette_strength;

                if factor > 0.5 {
                    let idx = (y as usize) * (fb.width as usize) + (x as usize);
                    fb.cells[idx].dim = true;
                }
            }
        }
    }

    fn apply_bloom(&self, fb: &mut FrameBuffer) {
        let w = fb.width as usize;
        let h = fb.height as usize;
        let mut bright_cells: Vec<(usize, usize)> = Vec::new();

        for y in 0..h {
            for x in 0..w {
                let idx = y * w + x;
                let cell = &fb.cells[idx];
                if cell.bold || matches!(cell.fg, Color::White | Color::Yellow | Color::Cyan) {
                    bright_cells.push((x, y));
                }
            }
        }

        for (bx, by) in bright_cells {
            for dy in -1i32..=1 {
                for dx in -1i32..=1 {
                    if dx == 0 && dy == 0 { continue; }
                    let nx = bx as i32 + dx;
                    let ny = by as i32 + dy;
                    if nx >= 0 && nx < w as i32 && ny >= 0 && ny < h as i32 {
                        let idx = ny as usize * w + nx as usize;
                        if fb.cells[idx].ch == ' ' {
                            fb.cells[idx].ch = '░';
                            fb.cells[idx].fg = Color::Rgb { r: 255, g: 255, b: 200 };
                        }
                    }
                }
            }
        }
    }

    fn apply_crt(&self, fb: &mut FrameBuffer) {
        let w = fb.width as usize;

        // Chromatic aberration - shift red channel left, blue right
        if self.chromatic_aberration > 0.0 {
            let shift = self.chromatic_aberration.ceil() as usize;
            for y in 0..fb.height as usize {
                for x in shift..w.saturating_sub(shift) {
                    let idx = y * w + x;
                    if fb.cells[idx].ch != ' ' {
                        if let Color::Rgb { r, g, b } = fb.cells[idx].fg {
                            // Shift color channels
                            let left_idx = y * w + (x - shift);
                            let right_idx = y * w + (x + shift);
                            if fb.cells[left_idx].ch == ' ' {
                                fb.cells[left_idx].fg = Color::Rgb { r, g: 0, b: 0 };
                                fb.cells[left_idx].ch = '░';
                            }
                            if fb.cells[right_idx].ch == ' ' {
                                fb.cells[right_idx].fg = Color::Rgb { r: 0, g: 0, b };
                                fb.cells[right_idx].ch = '░';
                            }
                            fb.cells[idx].fg = Color::Rgb { r, g, b };
                        }
                    }
                }
            }
        }
    }

    fn apply_glitch(&self, fb: &mut FrameBuffer) {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let w = fb.width as usize;
        let h = fb.height as usize;

        let num_glitch_lines = (self.glitch_intensity * 5.0) as usize;
        for _ in 0..num_glitch_lines {
            let y = rng.gen_range(0..h);
            let offset = rng.gen_range(-3i32..4);

            if offset != 0 {
                let row_start = y * w;
                let row: Vec<Cell> = fb.cells[row_start..row_start + w].to_vec();

                for x in 0..w {
                    let src_x = (x as i32 - offset).clamp(0, w as i32 - 1) as usize;
                    fb.cells[row_start + x] = row[src_x];
                }
            }

            // Random color corruption
            if rng.gen_bool(0.3) {
                let x = rng.gen_range(0..w);
                let idx = y * w + x;
                fb.cells[idx].fg = Color::Rgb {
                    r: rng.gen(),
                    g: rng.gen(),
                    b: rng.gen(),
                };
                let glitch_chars = ['▒', '▓', '█', '░', '▀', '▄'];
                fb.cells[idx].ch = glitch_chars[rng.gen_range(0..glitch_chars.len())];
            }
        }
    }

    fn apply_flash(&self, fb: &mut FrameBuffer, color: (u8, u8, u8)) {
        let opacity = self.flash_opacity.clamp(0.0, 1.0);
        if opacity < 0.1 { return; }

        for cell in &mut fb.cells {
            if let Color::Rgb { r, g, b } = cell.bg {
                cell.bg = Color::Rgb {
                    r: lerp_u8(r, color.0, opacity),
                    g: lerp_u8(g, color.1, opacity),
                    b: lerp_u8(b, color.2, opacity),
                };
            } else {
                cell.bg = Color::Rgb {
                    r: (color.0 as f64 * opacity) as u8,
                    g: (color.1 as f64 * opacity) as u8,
                    b: (color.2 as f64 * opacity) as u8,
                };
            }
        }
    }
}

fn lerp_u8(a: u8, b: u8, t: f64) -> u8 {
    ((a as f64) * (1.0 - t) + (b as f64) * t) as u8
}
