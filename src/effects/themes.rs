use crossterm::style::Color;
use super::VisualState;

#[derive(Debug, Clone)]
pub struct Theme {
    pub name: String,
    pub track_color: Color,
    pub secondary_track_color: Color,
    pub bg_color: Color,
    pub planet_1: Color,
    pub planet_2: Color,
    pub accent: Color,
    pub scanlines: bool,
    pub bloom: bool,
    pub crt: bool,
    pub vignette: bool,
}

impl Theme {
    pub fn get(name: &str) -> Option<Self> {
        match name.to_lowercase().as_str() {
            "neon" => Some(Self {
                name: "neon".into(),
                track_color: Color::Rgb { r: 0, g: 255, b: 200 },
                secondary_track_color: Color::Rgb { r: 255, g: 0, b: 255 },
                bg_color: Color::Rgb { r: 10, g: 0, b: 20 },
                planet_1: Color::Rgb { r: 255, g: 50, b: 255 },
                planet_2: Color::Rgb { r: 0, g: 255, b: 255 },
                accent: Color::Rgb { r: 255, g: 255, b: 0 },
                scanlines: false,
                bloom: true,
                crt: false,
                vignette: true,
            }),
            "retro" => Some(Self {
                name: "retro".into(),
                track_color: Color::Rgb { r: 0, g: 200, b: 0 },
                secondary_track_color: Color::Rgb { r: 0, g: 100, b: 0 },
                bg_color: Color::Rgb { r: 0, g: 10, b: 0 },
                planet_1: Color::Rgb { r: 0, g: 255, b: 0 },
                planet_2: Color::Rgb { r: 100, g: 255, b: 100 },
                accent: Color::Rgb { r: 0, g: 255, b: 0 },
                scanlines: true,
                bloom: false,
                crt: true,
                vignette: true,
            }),
            "minimal" => Some(Self {
                name: "minimal".into(),
                track_color: Color::Rgb { r: 200, g: 200, b: 200 },
                secondary_track_color: Color::Rgb { r: 100, g: 100, b: 100 },
                bg_color: Color::Rgb { r: 20, g: 20, b: 20 },
                planet_1: Color::White,
                planet_2: Color::Rgb { r: 180, g: 180, b: 180 },
                accent: Color::White,
                scanlines: false,
                bloom: false,
                crt: false,
                vignette: false,
            }),
            "fire" => Some(Self {
                name: "fire".into(),
                track_color: Color::Rgb { r: 255, g: 120, b: 0 },
                secondary_track_color: Color::Rgb { r: 200, g: 50, b: 0 },
                bg_color: Color::Rgb { r: 20, g: 5, b: 0 },
                planet_1: Color::Rgb { r: 255, g: 80, b: 0 },
                planet_2: Color::Rgb { r: 255, g: 200, b: 50 },
                accent: Color::Rgb { r: 255, g: 255, b: 100 },
                scanlines: false,
                bloom: true,
                crt: false,
                vignette: true,
            }),
            "ice" => Some(Self {
                name: "ice".into(),
                track_color: Color::Rgb { r: 100, g: 200, b: 255 },
                secondary_track_color: Color::Rgb { r: 50, g: 100, b: 200 },
                bg_color: Color::Rgb { r: 5, g: 10, b: 25 },
                planet_1: Color::Rgb { r: 150, g: 220, b: 255 },
                planet_2: Color::Rgb { r: 50, g: 150, b: 255 },
                accent: Color::Rgb { r: 200, g: 240, b: 255 },
                scanlines: false,
                bloom: true,
                crt: false,
                vignette: true,
            }),
            "synthwave" => Some(Self {
                name: "synthwave".into(),
                track_color: Color::Rgb { r: 255, g: 0, b: 128 },
                secondary_track_color: Color::Rgb { r: 128, g: 0, b: 255 },
                bg_color: Color::Rgb { r: 15, g: 0, b: 30 },
                planet_1: Color::Rgb { r: 255, g: 50, b: 150 },
                planet_2: Color::Rgb { r: 100, g: 50, b: 255 },
                accent: Color::Rgb { r: 255, g: 200, b: 50 },
                scanlines: true,
                bloom: true,
                crt: false,
                vignette: true,
            }),
            "matrix" => Some(Self {
                name: "matrix".into(),
                track_color: Color::Rgb { r: 0, g: 255, b: 65 },
                secondary_track_color: Color::Rgb { r: 0, g: 180, b: 40 },
                bg_color: Color::Rgb { r: 0, g: 5, b: 0 },
                planet_1: Color::Rgb { r: 0, g: 255, b: 0 },
                planet_2: Color::Rgb { r: 100, g: 255, b: 50 },
                accent: Color::Rgb { r: 200, g: 255, b: 200 },
                scanlines: true,
                bloom: true,
                crt: true,
                vignette: true,
            }),
            _ => None,
        }
    }

    pub fn apply_to_visual_state(&self, state: &mut VisualState) {
        state.track_color = self.track_color;
        state.secondary_track_color = self.secondary_track_color;
        state.bg_color = self.bg_color;
        state.planet_color_1 = self.planet_1;
        state.planet_color_2 = self.planet_2;
        state.bloom_active = self.bloom;
        if self.bloom {
            state.bloom_intensity = 0.7;
        }
    }

    pub fn list() -> Vec<&'static str> {
        vec!["neon", "retro", "minimal", "fire", "ice", "synthwave", "matrix"]
    }
}
