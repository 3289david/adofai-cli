use crossterm::style::Color;
use super::framebuffer::FrameBuffer;

pub const BLOCK_FULL: char = '█';
pub const BLOCK_DARK: char = '▓';
pub const BLOCK_MEDIUM: char = '▒';
pub const BLOCK_LIGHT: char = '░';
pub const HALF_UP: char = '▀';
pub const HALF_DOWN: char = '▄';

pub const BRAILLE_BASE: u32 = 0x2800;

pub struct UnicodePixelEngine;

impl UnicodePixelEngine {
    pub fn block_char(intensity: f64) -> char {
        if intensity > 0.9 { BLOCK_FULL }
        else if intensity > 0.7 { BLOCK_DARK }
        else if intensity > 0.4 { BLOCK_MEDIUM }
        else if intensity > 0.1 { BLOCK_LIGHT }
        else { ' ' }
    }

    pub fn draw_braille_dot(fb: &mut FrameBuffer, px: i32, py: i32, fg: Color) {
        let cell_x = px / 2;
        let cell_y = py / 4;
        let dot_x = (px % 2) as u32;
        let dot_y = (py % 4) as u32;

        let bit = match (dot_x, dot_y) {
            (0, 0) => 0x01,
            (0, 1) => 0x02,
            (0, 2) => 0x04,
            (1, 0) => 0x08,
            (1, 1) => 0x10,
            (1, 2) => 0x20,
            (0, 3) => 0x40,
            (1, 3) => 0x80,
            _ => 0,
        };

        if cell_x >= 0 && cell_x < fb.width as i32 && cell_y >= 0 && cell_y < fb.height as i32 {
            let idx = (cell_y as usize) * (fb.width as usize) + (cell_x as usize);
            let current = fb.cells[idx].ch as u32;
            let base = if current >= BRAILLE_BASE && current <= BRAILLE_BASE + 0xFF {
                current
            } else {
                BRAILLE_BASE
            };
            fb.cells[idx].ch = char::from_u32(base | bit).unwrap_or('⣿');
            fb.cells[idx].fg = fg;
        }
    }

    pub fn draw_braille_line(fb: &mut FrameBuffer, x0: i32, y0: i32, x1: i32, y1: i32, fg: Color) {
        let dx = (x1 - x0).abs();
        let dy = -(y1 - y0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx + dy;
        let mut x = x0;
        let mut y = y0;

        loop {
            Self::draw_braille_dot(fb, x, y, fg);
            if x == x1 && y == y1 { break; }
            let e2 = 2 * err;
            if e2 >= dy { err += dy; x += sx; }
            if e2 <= dx { err += dx; y += sy; }
        }
    }

    pub fn draw_braille_circle(fb: &mut FrameBuffer, cx: i32, cy: i32, r: i32, fg: Color) {
        let steps = (r as f64 * std::f64::consts::TAU).ceil() as i32;
        for i in 0..steps {
            let angle = (i as f64 / steps as f64) * std::f64::consts::TAU;
            let x = cx + (angle.cos() * r as f64) as i32;
            let y = cy + (angle.sin() * r as f64) as i32;
            Self::draw_braille_dot(fb, x, y, fg);
        }
    }

    pub fn subpixel_fill(fb: &mut FrameBuffer, x: i32, y: i32, top_color: Color, bottom_color: Color) {
        if x >= 0 && x < fb.width as i32 && y >= 0 && y < fb.height as i32 {
            let idx = (y as usize) * (fb.width as usize) + (x as usize);
            fb.cells[idx].ch = HALF_UP;
            fb.cells[idx].fg = top_color;
            fb.cells[idx].bg = bottom_color;
        }
    }

    pub fn draw_gradient_bar(fb: &mut FrameBuffer, x: i32, y: i32, width: i32, start: Color, end: Color) {
        if let (Color::Rgb { r: r1, g: g1, b: b1 }, Color::Rgb { r: r2, g: g2, b: b2 }) = (start, end) {
            for i in 0..width {
                let t = i as f64 / width as f64;
                let r = (r1 as f64 * (1.0 - t) + r2 as f64 * t) as u8;
                let g = (g1 as f64 * (1.0 - t) + g2 as f64 * t) as u8;
                let b = (b1 as f64 * (1.0 - t) + b2 as f64 * t) as u8;
                fb.set_char(x + i, y, BLOCK_FULL, Color::Rgb { r, g, b }, Color::Reset);
            }
        }
    }

    pub fn draw_progress_bar(fb: &mut FrameBuffer, x: i32, y: i32, width: i32, progress: f64, fg: Color, bg: Color) {
        let filled = (width as f64 * progress.clamp(0.0, 1.0)) as i32;
        for i in 0..width {
            if i < filled {
                fb.set_char(x + i, y, BLOCK_FULL, fg, Color::Reset);
            } else if i == filled {
                let frac = (width as f64 * progress) - filled as f64;
                let ch = if frac > 0.75 { '▓' }
                    else if frac > 0.5 { '▒' }
                    else if frac > 0.25 { '░' }
                    else { ' ' };
                fb.set_char(x + i, y, ch, fg, bg);
            } else {
                fb.set_char(x + i, y, ' ', fg, bg);
            }
        }
    }
}

pub fn angle_to_line_char(angle: f64) -> char {
    let normalized = ((angle % 360.0) + 360.0) % 360.0;
    match normalized as u32 {
        0 | 180 => '─',
        90 | 270 => '│',
        45 => '╲',
        135 => '╱',
        225 => '╲',
        315 => '╱',
        _ => {
            if (normalized > 337.5 || normalized <= 22.5) || (normalized > 157.5 && normalized <= 202.5) {
                '─'
            } else if (normalized > 67.5 && normalized <= 112.5) || (normalized > 247.5 && normalized <= 292.5) {
                '│'
            } else if (normalized > 22.5 && normalized <= 67.5) || (normalized > 202.5 && normalized <= 247.5) {
                '╲'
            } else {
                '╱'
            }
        }
    }
}

pub fn angle_to_connector(from_angle: f64, to_angle: f64) -> char {
    let diff = ((to_angle - from_angle) % 360.0 + 360.0) % 360.0;
    match diff as u32 {
        0 => '─',
        45 => '╮',
        90 => '│',
        135 => '╯',
        180 => '─',
        225 => '╰',
        270 => '│',
        315 => '╭',
        _ => '●',
    }
}
