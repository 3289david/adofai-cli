use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdofaiLevel {
    #[serde(default)]
    pub angle_data: Vec<f64>,
    #[serde(default)]
    pub path_data: Option<String>,
    pub settings: LevelSettings,
    #[serde(default)]
    pub actions: Vec<Action>,
    #[serde(default)]
    pub decorations: Vec<Decoration>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LevelSettings {
    #[serde(default = "default_version")]
    pub version: u32,
    #[serde(default)]
    pub artist: String,
    #[serde(default)]
    pub song: String,
    #[serde(default)]
    pub author: String,
    #[serde(default = "default_bpm")]
    pub bpm: f64,
    #[serde(default = "default_offset")]
    pub offset: f64,
    #[serde(default)]
    pub song_filename: String,
    #[serde(default = "default_pitch")]
    pub pitch: f64,
    #[serde(default)]
    pub volume: f64,
    #[serde(default)]
    pub hitsound: String,
    #[serde(default = "default_count_sound")]
    pub count_sound: String,
    #[serde(default)]
    pub countdown_ticks: u32,
    #[serde(default)]
    pub bg_color: String,
    #[serde(default)]
    pub track_color: String,
    #[serde(default)]
    pub secondary_track_color: String,
    #[serde(default)]
    pub track_color_type: String,
    #[serde(default)]
    pub track_style: String,
    #[serde(default)]
    pub track_animation: String,
    #[serde(default = "default_beats_ahead")]
    pub beats_ahead: f64,
    #[serde(default = "default_beats_behind")]
    pub beats_behind: f64,
    #[serde(default)]
    pub bg_image: String,
    #[serde(default)]
    pub bg_video: String,
    #[serde(default)]
    pub relative_to: String,
    #[serde(default)]
    pub position: Vec<f64>,
    #[serde(default)]
    pub rotation: f64,
    #[serde(default = "default_zoom")]
    pub zoom: f64,
    #[serde(default)]
    pub planet_ease: String,
    #[serde(default)]
    pub planet_ease_parts: u32,
    #[serde(default)]
    pub special_artist_type: String,
    #[serde(default)]
    pub difficulty: u32,
    #[serde(default)]
    pub required_mods: Vec<String>,
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Action {
    pub floor: usize,
    pub event_type: String,
    #[serde(flatten)]
    pub properties: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Decoration {
    pub floor: usize,
    pub event_type: String,
    #[serde(flatten)]
    pub properties: HashMap<String, serde_json::Value>,
}

fn default_version() -> u32 { 13 }
fn default_bpm() -> f64 { 120.0 }
fn default_offset() -> f64 { 0.0 }
fn default_pitch() -> f64 { 100.0 }
fn default_count_sound() -> String { "Cowbell".to_string() }
fn default_beats_ahead() -> f64 { 3.0 }
fn default_beats_behind() -> f64 { 4.0 }
fn default_zoom() -> f64 { 100.0 }

impl AdofaiLevel {
    pub fn load(path: &Path) -> Result<Self> {
        let content = std::fs::read_to_string(path)
            .with_context(|| format!("Failed to read file: {}", path.display()))?;

        // ADOFAI files sometimes have trailing commas and comments, clean them
        let cleaned = clean_json(&content);

        let level: AdofaiLevel = serde_json::from_str(&cleaned)
            .with_context(|| "Failed to parse ADOFAI level")?;

        Ok(level)
    }

    pub fn save(&self, path: &Path) -> Result<()> {
        let json = serde_json::to_string_pretty(self)
            .with_context(|| "Failed to serialize level")?;
        std::fs::write(path, json)
            .with_context(|| format!("Failed to write file: {}", path.display()))?;
        Ok(())
    }

    pub fn get_angles(&self) -> Vec<f64> {
        if !self.angle_data.is_empty() {
            return self.angle_data.clone();
        }

        if let Some(ref path_data) = self.path_data {
            return path_to_angles(path_data);
        }

        vec![]
    }

    pub fn tile_count(&self) -> usize {
        self.get_angles().len() + 1
    }

    pub fn get_actions_at(&self, floor: usize) -> Vec<&Action> {
        self.actions.iter().filter(|a| a.floor == floor).collect()
    }

    pub fn get_bpm_at(&self, floor: usize) -> f64 {
        let mut bpm = self.settings.bpm;
        for action in &self.actions {
            if action.floor > floor {
                break;
            }
            if action.event_type == "SetSpeed" {
                if let Some(val) = action.properties.get("beatsPerMinute") {
                    if let Some(b) = val.as_f64() {
                        bpm = b;
                    }
                }
                if let Some(val) = action.properties.get("bpmMultiplier") {
                    if let Some(mult) = val.as_f64() {
                        bpm *= mult;
                    }
                }
            }
        }
        bpm
    }

    pub fn has_twirl_at(&self, floor: usize) -> bool {
        self.actions.iter().any(|a| a.floor == floor && a.event_type == "Twirl")
    }

    pub fn new_empty(bpm: f64) -> Self {
        Self {
            angle_data: vec![0.0],
            path_data: None,
            settings: LevelSettings {
                version: 13,
                artist: String::new(),
                song: String::new(),
                author: String::new(),
                bpm,
                offset: 0.0,
                song_filename: String::new(),
                pitch: 100.0,
                volume: 100.0,
                hitsound: "Kick".to_string(),
                count_sound: "Cowbell".to_string(),
                countdown_ticks: 4,
                bg_color: "000000".to_string(),
                track_color: "debb7b".to_string(),
                secondary_track_color: "4f4f4f".to_string(),
                track_color_type: "Single".to_string(),
                track_style: "Standard".to_string(),
                track_animation: "None".to_string(),
                beats_ahead: 3.0,
                beats_behind: 4.0,
                bg_image: String::new(),
                bg_video: String::new(),
                relative_to: "Player".to_string(),
                position: vec![0.0, 0.0],
                rotation: 0.0,
                zoom: 100.0,
                planet_ease: "Linear".to_string(),
                planet_ease_parts: 1,
                special_artist_type: String::new(),
                difficulty: 1,
                required_mods: vec![],
                extra: HashMap::new(),
            },
            actions: vec![],
            decorations: vec![],
        }
    }
}

fn path_to_angles(path: &str) -> Vec<f64> {
    path.chars().filter_map(|c| match c {
        'R' => Some(0.0),
        'p' => Some(15.0),
        'J' => Some(30.0),
        'E' => Some(45.0),
        'T' => Some(60.0),
        'o' => Some(75.0),
        'U' => Some(90.0),
        'q' => Some(105.0),
        'G' => Some(120.0),
        'Q' => Some(135.0),
        'H' => Some(150.0),
        'W' => Some(165.0),
        'L' => Some(180.0),
        'x' => Some(195.0),
        'N' => Some(210.0),
        'Z' => Some(225.0),
        'F' => Some(240.0),
        'V' => Some(255.0),
        'D' => Some(270.0),
        'Y' => Some(285.0),
        'B' => Some(300.0),
        'C' => Some(315.0),
        'M' => Some(330.0),
        'A' => Some(345.0),
        '5' => Some(108.0),
        '6' => Some(252.0),
        '7' => Some(900.0), // midspin
        '8' => Some(900.0), // midspin
        '!' => Some(999.0), // special
        _ => None,
    }).collect()
}

fn clean_json(input: &str) -> String {
    let mut result = String::with_capacity(input.len());
    let mut in_string = false;
    let mut escape_next = false;
    let chars: Vec<char> = input.chars().collect();
    let len = chars.len();
    let mut i = 0;

    while i < len {
        let c = chars[i];

        if escape_next {
            result.push(c);
            escape_next = false;
            i += 1;
            continue;
        }

        if c == '\\' && in_string {
            result.push(c);
            escape_next = true;
            i += 1;
            continue;
        }

        if c == '"' {
            in_string = !in_string;
            result.push(c);
            i += 1;
            continue;
        }

        if !in_string {
            // Remove single-line comments
            if c == '/' && i + 1 < len && chars[i + 1] == '/' {
                while i < len && chars[i] != '\n' {
                    i += 1;
                }
                continue;
            }
            // Remove trailing commas before } or ]
            if c == ',' {
                let mut j = i + 1;
                while j < len && (chars[j] == ' ' || chars[j] == '\t' || chars[j] == '\n' || chars[j] == '\r') {
                    j += 1;
                }
                if j < len && (chars[j] == '}' || chars[j] == ']') {
                    i += 1;
                    continue;
                }
            }
        }

        result.push(c);
        i += 1;
    }

    result
}
