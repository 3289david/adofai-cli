pub mod themes;

use crossterm::style::Color;

#[derive(Debug, Clone)]
pub struct VisualState {
    pub track_color: Color,
    pub secondary_track_color: Color,
    pub bg_color: Color,
    pub planet_color_1: Color,
    pub planet_color_2: Color,
    pub flash_active: bool,
    pub flash_timer: f64,
    pub flash_duration: f64,
    pub flash_start_color: (u8, u8, u8),
    pub flash_end_color: (u8, u8, u8),
    pub bloom_active: bool,
    pub bloom_intensity: f64,
    pub filter_active: bool,
    pub filter_type: String,
    pub filter_intensity: f64,
    pub beat_pulse: f64,
    pub bpm_tunnel_factor: f64,
}

impl Default for VisualState {
    fn default() -> Self {
        Self {
            track_color: Color::Rgb { r: 222, g: 187, b: 123 },
            secondary_track_color: Color::Rgb { r: 79, g: 79, b: 79 },
            bg_color: Color::Black,
            planet_color_1: Color::Rgb { r: 255, g: 80, b: 50 },
            planet_color_2: Color::Rgb { r: 50, g: 150, b: 255 },
            flash_active: false,
            flash_timer: 0.0,
            flash_duration: 0.0,
            flash_start_color: (255, 255, 255),
            flash_end_color: (0, 0, 0),
            bloom_active: false,
            bloom_intensity: 0.5,
            filter_active: false,
            filter_type: String::new(),
            filter_intensity: 0.0,
            beat_pulse: 0.0,
            bpm_tunnel_factor: 0.0,
        }
    }
}

impl VisualState {
    pub fn update(&mut self, dt: f64, bpm: f64) {
        if self.flash_active {
            self.flash_timer += dt;
            if self.flash_timer >= self.flash_duration {
                self.flash_active = false;
            }
        }

        // Beat pulse based on BPM
        let beat_period = 60.0 / bpm;
        self.beat_pulse = (self.beat_pulse + dt / beat_period) % 1.0;

        // BPM tunnel factor
        self.bpm_tunnel_factor = (bpm / 200.0).clamp(0.0, 1.0);
    }

    pub fn trigger_flash(&mut self, duration: f64, start: (u8, u8, u8), end: (u8, u8, u8)) {
        self.flash_active = true;
        self.flash_timer = 0.0;
        self.flash_duration = duration;
        self.flash_start_color = start;
        self.flash_end_color = end;
    }

    pub fn get_flash_color(&self) -> Option<((u8, u8, u8), f64)> {
        if !self.flash_active { return None; }
        let t = (self.flash_timer / self.flash_duration).clamp(0.0, 1.0);
        let r = lerp_u8(self.flash_start_color.0, self.flash_end_color.0, t);
        let g = lerp_u8(self.flash_start_color.1, self.flash_end_color.1, t);
        let b = lerp_u8(self.flash_start_color.2, self.flash_end_color.2, t);
        let opacity = 1.0 - t;
        Some(((r, g, b), opacity))
    }

    pub fn get_beat_intensity(&self) -> f64 {
        let x = self.beat_pulse;
        // Sharp peak at beat, exponential decay
        (-x * 8.0).exp()
    }
}

fn lerp_u8(a: u8, b: u8, t: f64) -> u8 {
    ((a as f64) * (1.0 - t) + (b as f64) * t) as u8
}

pub fn hex_to_rgb(hex: &str) -> (u8, u8, u8) {
    let hex = hex.trim_start_matches('#');
    if hex.len() < 6 { return (255, 255, 255); }
    let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(255);
    let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(255);
    let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(255);
    (r, g, b)
}

pub fn hex_to_color(hex: &str) -> Color {
    let (r, g, b) = hex_to_rgb(hex);
    Color::Rgb { r, g, b }
}

pub fn hsl_to_color(h: f64, s: f64, l: f64) -> Color {
    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = l - c / 2.0;

    let (r, g, b) = match h as u32 {
        0..=59 => (c, x, 0.0),
        60..=119 => (x, c, 0.0),
        120..=179 => (0.0, c, x),
        180..=239 => (0.0, x, c),
        240..=299 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };

    Color::Rgb {
        r: ((r + m) * 255.0) as u8,
        g: ((g + m) * 255.0) as u8,
        b: ((b + m) * 255.0) as u8,
    }
}
