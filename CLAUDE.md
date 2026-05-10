# CLAUDE.md — ADOFAI CLI

## Project Overview

ADOFAI CLI is a cinematic terminal-based A Dance of Fire and Ice engine written in Rust. It plays, edits, analyzes, and renders real `.adofai` level files inside the terminal with shader-like post-processing effects, a custom framebuffer renderer, particle systems, and a full vim-style map editor.

## Build & Run

```bash
cargo build                     # Debug build
cargo build --release           # Release build (optimized, LTO, stripped)
cargo run -- play level.adofai  # Run with arguments
cargo run -- --help             # Show help
```

Binary name is `adofai` (not `adofai-cli`). The release binary outputs to `target/release/adofai`.

## Project Structure

```
src/
├── main.rs           CLI entry (clap derive API)
├── renderer/         Custom terminal render engine
│   ├── framebuffer   Cell grid with diff rendering
│   ├── pipeline      Orchestrates render passes
│   ├── post_process  Bloom, CRT, scanlines, glitch, flash, vignette
│   ├── particles     6 styles: Spark, Fire, Ice, Hit, Trail, Rainbow
│   └── unicode_pixel Braille dots, block chars, subpixel, gradients
├── engine/           Game runtime
│   ├── game          Main loop: input → update → render at 120 FPS
│   ├── tile          Tile positions from angles, visual chars, connectors
│   ├── planet        Orbit physics, multi-planet, twirl direction
│   ├── timing        Beat sync, tile time computation, hit accuracy
│   └── input         Crossterm key/mouse → GameInput enum
├── editor/           Map editor
│   ├── editor_state  Modes: Normal/Insert/Command/EventEdit/Timeline/Preview
│   └── undo          Branch-based undo tree (not linear stack)
├── parser/           .adofai format
│   ├── adofai        JSON parser with trailing comma/comment cleanup
│   └── events        All ADOFAI event type definitions
├── camera/           Smooth follow, zoom, rotation, shake, pulse
├── effects/          VisualState + theme system (7 themes)
├── audio/            Rodio playback + hitsounds
├── replay/           Input recording + playback
└── cli/              One module per subcommand
```

## Key Design Decisions

- **Diff rendering**: FrameBuffer tracks previous frame, only redraws changed cells → high FPS
- **No ratatui widgets for gameplay**: Direct framebuffer manipulation for maximum control
- **Branch undo tree**: Editor stores snapshots as tree nodes, not a stack — supports branching history
- **pathData and angleData**: Parser handles both ADOFAI path encoding formats
- **JSON cleanup**: ADOFAI files often have trailing commas and comments — `clean_json()` strips them before serde parsing

## Conventions

- All coordinates use f64 world space; camera transforms to screen i32
- Colors use `crossterm::style::Color::Rgb` for 24-bit support
- Angles follow ADOFAI convention: 0° = right, 90° = up, counterclockwise
- Terminal Y axis is inverted (positive = down)

## Testing

```bash
cargo test                # Run all tests
cargo run -- new          # Quick smoke test: creates a valid .adofai file
cargo run -- analyze <f>  # Verify parser on a real level
```

## npm / Homebrew

- `npm/` contains the npm package wrapper (`adofai-terminal`)
- `homebrew/` contains the Homebrew formula
- `.github/workflows/release.yml` builds cross-platform binaries and publishes

## Website

`website/index.html` is a single-page site deployed to GitHub Pages via the release workflow.
