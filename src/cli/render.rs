use anyhow::{Context, Result};
use std::path::PathBuf;
use crate::parser::AdofaiLevel;
use crate::engine::game::GameState;

pub fn run(file: PathBuf, output: Option<PathBuf>) -> Result<()> {
    let level = AdofaiLevel::load(&file)
        .with_context(|| format!("Failed to load level: {}", file.display()))?;

    let output = output.unwrap_or_else(|| {
        let stem = file.file_stem().unwrap_or_default().to_string_lossy();
        PathBuf::from(format!("{}.render.txt", stem))
    });

    println!("Rendering level: {}", file.display());
    println!("Output: {}", output.display());
    println!("Tiles: {}", level.tile_count());
    println!();

    // Auto-play and capture frames
    let mut game = GameState::new(level, 0, true, None)?;
    game.run()?;

    println!("Render complete: {}", output.display());
    Ok(())
}
