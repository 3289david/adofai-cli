use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventType {
    SetSpeed { bpm: Option<f64>, multiplier: Option<f64>, speed_type: SpeedType },
    Twirl,
    SetPlanetRotation { ease: String, ease_parts: u32 },
    MultiPlanet { planets: u32 },
    Pause { duration: f64, count_ticks: u32, angle_correction: f64 },
    AutoPlayTiles { enabled: bool },
    ScaleMargin,
    ScaleRadius { scale: f64 },
    Checkpoint,
    CustomBackground { color: String, image: String, parallax: Vec<f64> },
    Flash { duration: f64, plane: String, start_color: String, end_color: String, start_opacity: f64, end_opacity: f64, angle: f64, ease: String },
    MoveCamera { duration: f64, relative_to: String, position: Vec<f64>, rotation: f64, zoom: f64, angle_offset: f64, ease: String },
    SetFilter { filter: String, intensity: f64, duration: f64, ease: String },
    HallOfMirrors { enabled: bool },
    ShakeScreen { duration: f64, strength: f64, intensity: f64, fade_out: bool },
    Bloom { enabled: bool, threshold: f64, intensity: f64, color: String },
    ScreenTile { x: f64, y: f64 },
    ScreenScroll { x: f64, y: f64 },
    SetTrackColor { color: String, secondary_color: String, color_type: String, animation_duration: f64, ease: String },
    RecolorTrack { start: usize, end: usize, gap_length: f64, color: String },
    MoveTrack { start: usize, end: usize, gap_length: f64, duration: f64, angle_offset: f64, ease: String, position_offset: Vec<f64>, rotation_offset: f64, scale: f64, opacity: f64 },
    SetText { decal_text: String, font: String, position: Vec<f64>, relative_to: String, angle: f64, color: String },
    AddDecoration { decoration_image: String, position: Vec<f64>, relative_to: String, pivot_offset: Vec<f64>, rotation: f64, scale: Vec<f64>, tile: Vec<f64>, color: String },
    PositionTrack { position_offset: Vec<f64>, relative_to: String, rotation: f64, scale: f64, opacity: f64, just_this_tile: bool, editor_only: bool },
    RepeatEvents { repetitions: u32, floor_count: u32, interval: f64 },
    SetConditionalEvents { enabled: bool },
    SetHitsound { hitsound: String, volume: f64 },
    PlaySound { hitsound: String, volume: f64, angle_offset: f64 },
    Hold { duration: f64, distance_multiplier: f64, landing_animation: bool },
    EditorComment { comment: String },
    Bookmark,
    Unknown { event_type: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SpeedType {
    Bpm,
    Multiplier,
}

impl EventType {
    pub fn name(&self) -> &str {
        match self {
            Self::SetSpeed { .. } => "SetSpeed",
            Self::Twirl => "Twirl",
            Self::SetPlanetRotation { .. } => "SetPlanetRotation",
            Self::MultiPlanet { .. } => "MultiPlanet",
            Self::Pause { .. } => "Pause",
            Self::AutoPlayTiles { .. } => "AutoPlayTiles",
            Self::ScaleMargin => "ScaleMargin",
            Self::ScaleRadius { .. } => "ScaleRadius",
            Self::Checkpoint => "Checkpoint",
            Self::CustomBackground { .. } => "CustomBackground",
            Self::Flash { .. } => "Flash",
            Self::MoveCamera { .. } => "MoveCamera",
            Self::SetFilter { .. } => "SetFilter",
            Self::HallOfMirrors { .. } => "HallOfMirrors",
            Self::ShakeScreen { .. } => "ShakeScreen",
            Self::Bloom { .. } => "Bloom",
            Self::ScreenTile { .. } => "ScreenTile",
            Self::ScreenScroll { .. } => "ScreenScroll",
            Self::SetTrackColor { .. } => "SetTrackColor",
            Self::RecolorTrack { .. } => "RecolorTrack",
            Self::MoveTrack { .. } => "MoveTrack",
            Self::SetText { .. } => "SetText",
            Self::AddDecoration { .. } => "AddDecoration",
            Self::PositionTrack { .. } => "PositionTrack",
            Self::RepeatEvents { .. } => "RepeatEvents",
            Self::SetConditionalEvents { .. } => "SetConditionalEvents",
            Self::SetHitsound { .. } => "SetHitsound",
            Self::PlaySound { .. } => "PlaySound",
            Self::Hold { .. } => "Hold",
            Self::EditorComment { .. } => "EditorComment",
            Self::Bookmark => "Bookmark",
            Self::Unknown { .. } => "Unknown",
        }
    }
}
