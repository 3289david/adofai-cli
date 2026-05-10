use anyhow::Result;
use std::path::PathBuf;
use crate::parser::AdofaiLevel;

pub fn run(output: PathBuf, bpm: f64) -> Result<()> {
    let level = AdofaiLevel::new_empty(bpm);
    level.save(&output)?;

    println!("Created new ADOFAI level:");
    println!("  File: {}", output.display());
    println!("  BPM: {}", bpm);
    println!();
    println!("  Open with: adofai edit {}", output.display());

    Ok(())
}
