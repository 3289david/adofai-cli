use anyhow::{Context, Result};
use std::path::PathBuf;
use crate::parser::AdofaiLevel;

pub fn run(file: PathBuf) -> Result<()> {
    let level = AdofaiLevel::load(&file)
        .with_context(|| format!("Failed to load level: {}", file.display()))?;

    let angles = level.get_angles();
    let tile_count = angles.len() + 1;

    println!("╭──────────────────────────────────────╮");
    println!("│          LEVEL ANALYSIS              │");
    println!("├──────────────────────────────────────┤");
    println!("│ Song: {:31} │", truncate(&level.settings.song, 31));
    println!("│ Artist: {:29} │", truncate(&level.settings.artist, 29));
    println!("│ Author: {:29} │", truncate(&level.settings.author, 29));
    println!("│ BPM: {:32} │", level.settings.bpm);
    println!("│ Offset: {:27}ms │", level.settings.offset);
    println!("│ Tiles: {:30} │", tile_count);
    println!("│ Difficulty: {:25} │", level.settings.difficulty);
    println!("├──────────────────────────────────────┤");

    // Angle distribution
    let mut angle_counts: std::collections::HashMap<i32, usize> = std::collections::HashMap::new();
    for &a in &angles {
        let bucket = (a as i32 / 45) * 45;
        *angle_counts.entry(bucket).or_insert(0) += 1;
    }

    println!("│ Angle Distribution:                  │");
    let mut sorted: Vec<_> = angle_counts.iter().collect();
    sorted.sort_by_key(|&(k, _)| *k);
    for (angle, count) in &sorted {
        let bar_len = (**count as f64 / tile_count as f64 * 25.0) as usize;
        let bar: String = "█".repeat(bar_len);
        println!("│ {:>4}°: {:25} {:>3} │", angle, bar, count);
    }

    println!("├──────────────────────────────────────┤");

    // Event summary
    let mut event_counts: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
    for action in &level.actions {
        *event_counts.entry(action.event_type.clone()).or_insert(0) += 1;
    }

    println!("│ Events:                              │");
    let mut sorted_events: Vec<_> = event_counts.iter().collect();
    sorted_events.sort_by(|a, b| b.1.cmp(a.1));
    for (event_type, count) in &sorted_events {
        println!("│   {:28} {:>4} │", truncate(event_type, 28), count);
    }

    if level.actions.is_empty() {
        println!("│   (no events)                        │");
    }

    println!("├──────────────────────────────────────┤");

    // BPM changes
    let bpm_changes: Vec<_> = level.actions.iter()
        .filter(|a| a.event_type == "SetSpeed")
        .collect();

    if !bpm_changes.is_empty() {
        println!("│ BPM Changes:                         │");
        let mut min_bpm = level.settings.bpm;
        let mut max_bpm = level.settings.bpm;
        for action in &bpm_changes {
            if let Some(val) = action.properties.get("beatsPerMinute").and_then(|v| v.as_f64()) {
                if val < min_bpm { min_bpm = val; }
                if val > max_bpm { max_bpm = val; }
            }
        }
        println!("│   Range: {:.0} - {:.0} BPM {:>15} │", min_bpm, max_bpm, "");
        println!("│   Changes: {:26} │", bpm_changes.len());
    }

    // Twirl count
    let twirl_count = level.actions.iter().filter(|a| a.event_type == "Twirl").count();
    println!("│ Twirls: {:29} │", twirl_count);

    // Decorations
    println!("│ Decorations: {:24} │", level.decorations.len());

    println!("├──────────────────────────────────────┤");

    // Density visualization
    println!("│ Density Graph (tiles per section):   │");
    let section_size = (tile_count / 20).max(1);
    for i in 0..20 {
        let start = i * section_size;
        let end = ((i + 1) * section_size).min(tile_count);
        if start >= tile_count { break; }

        let density = end - start;
        let bpm = level.get_bpm_at(start);
        let intensity = (bpm / 300.0).clamp(0.0, 1.0);
        let bar_len = (density as f64 / section_size as f64 * 15.0) as usize;
        let bar: String = if intensity > 0.7 { "█" } else if intensity > 0.4 { "▓" } else { "▒" }
            .repeat(bar_len);
        println!("│ {:>4}-{:<4}: {:25} │", start, end, bar);
    }

    println!("╰──────────────────────────────────────╯");

    Ok(())
}

fn truncate(s: &str, max: usize) -> String {
    if s.len() <= max {
        s.to_string()
    } else {
        format!("{}...", &s[..max.saturating_sub(3)])
    }
}
