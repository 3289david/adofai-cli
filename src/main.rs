mod renderer;
mod engine;
mod editor;
mod parser;
mod audio;
mod effects;
mod camera;
mod replay;
mod cli;

use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "adofai")]
#[command(about = "Cinematic terminal-based A Dance of Fire and Ice engine")]
#[command(version)]
#[command(author)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Play an ADOFAI level
    Play {
        /// Path to .adofai file
        file: PathBuf,
        /// Start from specific tile
        #[arg(short, long, default_value_t = 0)]
        start: usize,
        /// Auto-play mode
        #[arg(short, long)]
        auto_play: bool,
        /// Apply a theme
        #[arg(short, long)]
        theme: Option<String>,
    },
    /// Open the map editor
    Edit {
        /// Path to .adofai file
        file: PathBuf,
    },
    /// Create a new level
    New {
        /// Output path
        #[arg(short, long, default_value = "untitled.adofai")]
        output: PathBuf,
        /// Starting BPM
        #[arg(short, long, default_value_t = 120.0)]
        bpm: f64,
    },
    /// Render level to video-like terminal recording
    Render {
        /// Path to .adofai file
        file: PathBuf,
        /// Output path for recording
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
    /// View a replay
    Replay {
        /// Path to .rep file
        file: PathBuf,
        /// Playback speed multiplier
        #[arg(short, long, default_value_t = 1.0)]
        speed: f64,
    },
    /// Apply a visual theme
    Theme {
        /// Theme name (neon, retro, minimal, fire, ice)
        name: String,
    },
    /// Show level analysis and statistics
    Analyze {
        /// Path to .adofai file
        file: PathBuf,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Play { file, start, auto_play, theme } => {
            cli::play::run(file, start, auto_play, theme)?;
        }
        Commands::Edit { file } => {
            cli::edit::run(file)?;
        }
        Commands::New { output, bpm } => {
            cli::new::run(output, bpm)?;
        }
        Commands::Render { file, output } => {
            cli::render::run(file, output)?;
        }
        Commands::Replay { file, speed } => {
            cli::replay::run(file, speed)?;
        }
        Commands::Theme { name } => {
            cli::theme::run(name)?;
        }
        Commands::Analyze { file } => {
            cli::analyze::run(file)?;
        }
    }

    Ok(())
}
