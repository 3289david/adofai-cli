use crossterm::style::Color;

#[derive(Debug, Clone)]
pub struct Tile {
    pub index: usize,
    pub angle: f64,
    pub x: f64,
    pub y: f64,
    pub color: Color,
    pub secondary_color: Color,
    pub has_twirl: bool,
    pub is_midspin: bool,
    pub events: Vec<TileEvent>,
}

#[derive(Debug, Clone)]
pub enum TileEvent {
    SetSpeed { bpm: Option<f64>, multiplier: Option<f64> },
    Twirl,
    Pause { duration: f64 },
    Flash { duration: f64, color: (u8, u8, u8) },
    MoveCamera { duration: f64, position: Option<(f64, f64)>, rotation: Option<f64>, zoom: Option<f64>, ease: String },
    ShakeScreen { duration: f64, strength: f64 },
    Bloom { enabled: bool, intensity: f64 },
    SetFilter { filter: String, intensity: f64 },
    Checkpoint,
    MultiPlanet { planets: u32 },
    SetTrackColor { color: Color, secondary: Color },
    Hold { duration: f64 },
}

impl Tile {
    pub fn compute_positions(angles: &[f64], scale: f64) -> Vec<(f64, f64)> {
        let mut positions = Vec::with_capacity(angles.len() + 1);
        positions.push((0.0, 0.0));

        let mut x = 0.0;
        let mut y = 0.0;

        for &angle in angles {
            if angle >= 900.0 {
                // Midspin - same position
                positions.push((x, y));
                continue;
            }

            let rad = angle.to_radians();
            x += rad.cos() * scale;
            y -= rad.sin() * scale; // Y is inverted in terminal
            positions.push((x, y));
        }

        positions
    }

    pub fn build_tiles(angles: &[f64], track_color: Color, secondary_color: Color) -> Vec<Self> {
        let positions = Self::compute_positions(angles, 3.0);
        let mut tiles = Vec::with_capacity(angles.len() + 1);

        // First tile at origin
        tiles.push(Tile {
            index: 0,
            angle: 0.0,
            x: positions[0].0,
            y: positions[0].1,
            color: track_color,
            secondary_color,
            has_twirl: false,
            is_midspin: false,
            events: vec![],
        });

        for (i, &angle) in angles.iter().enumerate() {
            tiles.push(Tile {
                index: i + 1,
                angle,
                x: positions[i + 1].0,
                y: positions[i + 1].1,
                color: if i % 2 == 0 { track_color } else { secondary_color },
                secondary_color,
                has_twirl: false,
                is_midspin: angle >= 900.0,
                events: vec![],
            });
        }

        tiles
    }

    pub fn get_visual_char(&self) -> char {
        if self.is_midspin { return '◇'; }
        '●'
    }

    pub fn get_connector_to(&self, next: &Tile) -> char {
        let dx = next.x - self.x;
        let dy = next.y - self.y;
        let angle = dy.atan2(dx).to_degrees();
        let normalized = ((angle % 360.0) + 360.0) % 360.0;

        match normalized as u32 {
            0..=22 | 338..=360 => '─',
            23..=67 => '╲',
            68..=112 => '│',
            113..=157 => '╱',
            158..=202 => '─',
            203..=247 => '╲',
            248..=292 => '│',
            293..=337 => '╱',
            _ => '─',
        }
    }
}
