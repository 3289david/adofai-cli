use std::time::Instant;

#[derive(Debug, Clone)]
pub struct TimingEngine {
    pub bpm: f64,
    pub offset: f64,
    pub start_time: Option<Instant>,
    pub elapsed: f64,
    pub beat_count: u64,
    pub tile_times: Vec<f64>,
    pub paused: bool,
    pub pause_time: f64,
}

impl TimingEngine {
    pub fn new(bpm: f64, offset: f64) -> Self {
        Self {
            bpm,
            offset,
            start_time: None,
            elapsed: 0.0,
            beat_count: 0,
            tile_times: Vec::new(),
            paused: false,
            pause_time: 0.0,
        }
    }

    pub fn start(&mut self) {
        self.start_time = Some(Instant::now());
        self.elapsed = 0.0;
    }

    pub fn update(&mut self) {
        if self.paused { return; }
        if let Some(start) = self.start_time {
            self.elapsed = start.elapsed().as_secs_f64() - self.offset / 1000.0 - self.pause_time;
        }
    }

    pub fn beat_duration(&self) -> f64 {
        60.0 / self.bpm
    }

    pub fn current_beat(&self) -> f64 {
        self.elapsed / self.beat_duration()
    }

    pub fn beat_progress(&self) -> f64 {
        self.current_beat() % 1.0
    }

    pub fn set_bpm(&mut self, bpm: f64) {
        self.bpm = bpm;
    }

    pub fn compute_tile_times(&mut self, angles: &[f64]) {
        self.tile_times.clear();
        let mut time = 0.0;
        let mut current_bpm = self.bpm;

        self.tile_times.push(time);

        for &angle in angles {
            if angle >= 900.0 {
                // Midspin - half beat
                time += 60.0 / current_bpm / 2.0;
            } else {
                let angle_ratio = 180.0 / 180.0; // Standard tile = 1 beat
                time += 60.0 / current_bpm * angle_ratio;
            }
            self.tile_times.push(time);
        }
    }

    pub fn get_current_tile(&self) -> usize {
        for (i, &t) in self.tile_times.iter().enumerate().rev() {
            if self.elapsed >= t {
                return i;
            }
        }
        0
    }

    pub fn get_hit_accuracy(&self, tile_index: usize) -> Option<HitAccuracy> {
        if tile_index >= self.tile_times.len() { return None; }

        let target_time = self.tile_times[tile_index];
        let diff = (self.elapsed - target_time).abs() * 1000.0; // ms

        Some(if diff < 25.0 {
            HitAccuracy::Perfect
        } else if diff < 50.0 {
            HitAccuracy::Great
        } else if diff < 100.0 {
            HitAccuracy::Good
        } else if diff < 150.0 {
            HitAccuracy::Early
        } else if diff < 200.0 {
            HitAccuracy::Late
        } else {
            HitAccuracy::Miss
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HitAccuracy {
    Perfect,
    Great,
    Good,
    Early,
    Late,
    Miss,
}

impl HitAccuracy {
    pub fn label(&self) -> &str {
        match self {
            Self::Perfect => "PERFECT!",
            Self::Great => "Great!",
            Self::Good => "Good",
            Self::Early => "Early",
            Self::Late => "Late",
            Self::Miss => "MISS",
        }
    }

    pub fn score(&self) -> u32 {
        match self {
            Self::Perfect => 100,
            Self::Great => 80,
            Self::Good => 60,
            Self::Early => 40,
            Self::Late => 40,
            Self::Miss => 0,
        }
    }

    pub fn color(&self) -> crossterm::style::Color {
        use crossterm::style::Color;
        match self {
            Self::Perfect => Color::Rgb { r: 255, g: 255, b: 50 },
            Self::Great => Color::Rgb { r: 100, g: 255, b: 100 },
            Self::Good => Color::Rgb { r: 50, g: 200, b: 255 },
            Self::Early => Color::Rgb { r: 255, g: 150, b: 50 },
            Self::Late => Color::Rgb { r: 255, g: 100, b: 50 },
            Self::Miss => Color::Rgb { r: 255, g: 50, b: 50 },
        }
    }
}
