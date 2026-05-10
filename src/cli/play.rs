use anyhow::{Context, Result};
use std::path::PathBuf;
use crate::parser::AdofaiLevel;
use crate::engine::game::GameState;

pub fn run(file: PathBuf, start: usize, auto_play: bool, theme: Option<String>) -> Result<()> {
    let level = AdofaiLevel::load(&file)
        .with_context(|| format!("Failed to load level: {}", file.display()))?;

    println!("╭─────────────────────────────────╮");
    println!("│     ADOFAI Terminal Engine       │");
    println!("├─────────────────────────────────┤");
    println!("│ Song: {:25} │", truncate(&level.settings.song, 25));
    println!("│ Artist: {:23} │", truncate(&level.settings.artist, 23));
    println!("│ BPM: {:26} │", level.settings.bpm);
    println!("│ Tiles: {:24} │", level.tile_count());
    if auto_play {
        println!("│ Mode: AUTO                      │");
    }
    if let Some(ref t) = theme {
        println!("│ Theme: {:24} │", t);
    }
    println!("╰─────────────────────────────────╯");
    println!();
    println!("  Controls:");
    println!("  Space/J/K/D/F/Arrows = Hit");
    println!("  P/Esc = Pause");
    println!("  R = Restart");
    println!("  A = Toggle Auto-play");
    println!("  Q = Quit");
    println!();
    println!("  Press any key to start...");

    // Wait for keypress
    crossterm::terminal::enable_raw_mode()?;
    loop {
        if crossterm::event::poll(std::time::Duration::from_millis(100))? {
            if let crossterm::event::Event::Key(_) = crossterm::event::read()? {
                break;
            }
        }
    }
    crossterm::terminal::disable_raw_mode()?;

    let mut game = GameState::new(level, start, auto_play, theme)?;
    game.run()?;

    Ok(())
}

fn truncate(s: &str, max: usize) -> String {
    if s.len() <= max {
        s.to_string()
    } else {
        format!("{}...", &s[..max - 3])
    }
}
