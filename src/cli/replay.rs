use anyhow::{Context, Result};
use std::path::PathBuf;
use crate::replay::ReplayPlayer;

pub fn run(file: PathBuf, speed: f64) -> Result<()> {
    let player = ReplayPlayer::load(&file)
        .with_context(|| format!("Failed to load replay: {}", file.display()))?;

    println!("╭─────────────────────────────────╮");
    println!("│         REPLAY VIEWER           │");
    println!("├─────────────────────────────────┤");
    println!("│ Level: {:24} │", player.data.level_file);
    println!("│ Player: {:23} │", player.data.player);
    println!("│ Date: {:25} │", player.data.date);
    println!("│ Score: {:24} │", player.data.total_score);
    println!("│ Max Combo: {:20} │", player.data.max_combo);
    println!("│ Speed: {:24.1}x │", speed);
    println!("│ Hits: {:25} │", player.data.hits.len());
    println!("╰─────────────────────────────────╯");
    println!();
    println!("  Replay playback coming soon!");

    Ok(())
}
