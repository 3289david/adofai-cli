use anyhow::{Context, Result};
use std::path::PathBuf;
use crate::parser::AdofaiLevel;
use crate::editor::EditorState;

pub fn run(file: PathBuf) -> Result<()> {
    let level = if file.exists() {
        AdofaiLevel::load(&file)
            .with_context(|| format!("Failed to load level: {}", file.display()))?
    } else {
        let level = AdofaiLevel::new_empty(120.0);
        level.save(&file)?;
        level
    };

    let mut editor = EditorState::new(level, file)?;
    editor.run()?;

    Ok(())
}
