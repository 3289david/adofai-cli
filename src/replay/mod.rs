use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplayData {
    pub version: u32,
    pub level_file: String,
    pub player: String,
    pub date: String,
    pub hits: Vec<ReplayHit>,
    pub total_score: u64,
    pub max_combo: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplayHit {
    pub time: f64,
    pub tile: usize,
    pub accuracy_ms: f64,
}

pub struct ReplayRecorder {
    hits: Vec<ReplayHit>,
    recording: bool,
}

impl ReplayRecorder {
    pub fn new() -> Self {
        Self {
            hits: Vec::new(),
            recording: true,
        }
    }

    pub fn record_hit(&mut self, time: f64, tile: usize) {
        if self.recording {
            self.hits.push(ReplayHit {
                time,
                tile,
                accuracy_ms: 0.0,
            });
        }
    }

    pub fn save(&self, path: &Path, level_file: &str, score: u64, max_combo: u32) -> Result<()> {
        let data = ReplayData {
            version: 1,
            level_file: level_file.to_string(),
            player: std::env::var("USER").unwrap_or_else(|_| "Player".to_string()),
            date: chrono_lite_now(),
            hits: self.hits.clone(),
            total_score: score,
            max_combo,
        };

        let json = serde_json::to_string_pretty(&data)?;
        std::fs::write(path, json)?;
        Ok(())
    }
}

pub struct ReplayPlayer {
    pub data: ReplayData,
    pub current_hit: usize,
    pub speed: f64,
    pub paused: bool,
    pub elapsed: f64,
}

impl ReplayPlayer {
    pub fn load(path: &Path) -> Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let data: ReplayData = serde_json::from_str(&content)?;
        Ok(Self {
            data,
            current_hit: 0,
            speed: 1.0,
            paused: false,
            elapsed: 0.0,
        })
    }

    pub fn update(&mut self, dt: f64) -> Option<&ReplayHit> {
        if self.paused { return None; }

        self.elapsed += dt * self.speed;

        if self.current_hit < self.data.hits.len() {
            let hit = &self.data.hits[self.current_hit];
            if self.elapsed >= hit.time {
                self.current_hit += 1;
                return Some(hit);
            }
        }
        None
    }

    pub fn is_finished(&self) -> bool {
        self.current_hit >= self.data.hits.len()
    }

    pub fn progress(&self) -> f64 {
        if self.data.hits.is_empty() { return 1.0; }
        self.current_hit as f64 / self.data.hits.len() as f64
    }
}

fn chrono_lite_now() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let secs = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    let days = secs / 86400;
    let year = 1970 + days / 365;
    let day_of_year = days % 365;
    let month = day_of_year / 30 + 1;
    let day = day_of_year % 30 + 1;
    format!("{:04}-{:02}-{:02}", year, month, day)
}
