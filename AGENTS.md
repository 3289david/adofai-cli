# AGENTS.md — AI Agent Guide for ADOFAI CLI

This document provides context for AI coding agents working on the ADOFAI CLI codebase.

## Project Identity

**ADOFAI CLI** is a cinematic terminal-based rhythm game engine. It is not a toy project — it is a fully functional engine with a custom rendering pipeline, particle system, camera system, audio engine, vim-style editor, and replay system, all running inside a terminal at 120 FPS.

The design philosophy is: **"The terminal is not a limitation — it's a style."**

## Architecture Overview

```
User Input → Engine → Camera → Renderer → PostProcessor → FrameBuffer → Terminal
                ↑                               ↑
            TimingEngine                   Particle System
                ↑
           .adofai Parser
```

### Data Flow

1. **Parser** reads `.adofai` files → `AdofaiLevel` struct
2. **Engine** converts angles to tile positions and builds game state
3. **TimingEngine** synchronizes gameplay to BPM
4. **PlanetSystem** handles orbit physics (the core ADOFAI mechanic)
5. **Camera** smoothly tracks the active tile with zoom/rotation/shake
6. **RenderPipeline** draws tiles, planets, particles to `FrameBuffer`
7. **PostProcessor** applies bloom, scanlines, CRT, glitch, flash, vignette
8. **FrameBuffer.render_diff()** outputs only changed cells to terminal

### Key Types

| Type | Location | Purpose |
|------|----------|---------|
| `AdofaiLevel` | `parser/adofai.rs` | Complete level representation |
| `GameState` | `engine/game.rs` | Runtime game state (owns everything) |
| `EditorState` | `editor/editor_state.rs` | Editor runtime state |
| `FrameBuffer` | `renderer/framebuffer.rs` | Cell grid with diff rendering |
| `RenderPipeline` | `renderer/pipeline.rs` | Coordinates all rendering |
| `PostProcessor` | `renderer/post_process.rs` | Post-processing effects |
| `ParticleSystem` | `renderer/particles.rs` | Particle emission and physics |
| `Camera` | `camera/mod.rs` | Cinematic camera with smooth follow |
| `PlanetSystem` | `engine/planet.rs` | Orbit physics |
| `TimingEngine` | `engine/timing.rs` | Beat sync and accuracy |
| `VisualState` | `effects/mod.rs` | Current visual effect state |
| `Theme` | `effects/themes.rs` | Named visual presets |
| `UndoTree` | `editor/undo.rs` | Branch-based undo history |

## Agent Guidelines

### When adding new features

1. **New events**: Add to `parser/events.rs` (type definition), `parser/adofai.rs` (parsing), `engine/game.rs` (runtime handling)
2. **New effects**: Add to `renderer/post_process.rs` and wire up in `effects/mod.rs` via `VisualState`
3. **New themes**: Add to `effects/themes.rs` in `Theme::get()`
4. **New commands**: Add variant to `Commands` enum in `main.rs`, create handler in `src/cli/`
5. **New particle styles**: Add variant to `ParticleStyle` enum in `renderer/particles.rs`

### When modifying the renderer

- All rendering goes through `FrameBuffer` — never write directly to stdout during gameplay
- Use `set_char()` / `set_str()` for single characters/strings
- Use `draw_box()` / `draw_line()` / `fill_rect()` for primitives
- Post-processing reads and modifies `fb.cells` directly
- `render_diff()` compares against `prev_cells` — this is critical for performance

### When modifying the editor

- The editor uses modal editing (like vim): Normal, Insert, Command, EventEdit, Timeline, Preview
- All state mutations should go through `undo_tree.push()` first
- After modifying `level.angle_data` or `level.actions`, call `rebuild_tiles()`
- Commands in `:` mode are dispatched in `execute_command()`

### When modifying the parser

- ADOFAI files are JSON but often have trailing commas and `//` comments
- `clean_json()` handles this — test any parser changes against real ADOFAI files
- Both `angleData` (array of floats) and `pathData` (string of characters) are supported
- The `Action` struct uses `HashMap<String, serde_json::Value>` for flexibility — ADOFAI has many event-specific fields

### Performance considerations

- The rendering loop targets 120 FPS — avoid allocations in the hot path
- `render_diff()` is O(cells changed), not O(total cells) — preserve this invariant
- Particle system caps at `max_particles` (default 500) to prevent slowdown
- Camera interpolation uses exponential smoothing — avoid sudden jumps

### Testing approach

- Parser can be tested against real `.adofai` files (they're JSON)
- Use `adofai new` to generate valid test files
- The editor and player require a terminal — manual testing is expected
- `adofai analyze` is a good sanity check for parser correctness

## File Format Reference

`.adofai` files are JSON with this structure:

```json
{
  "angleData": [0, 180, 0, 90, 270, ...],
  "settings": {
    "bpm": 180,
    "offset": 0,
    "song": "Title",
    "artist": "Artist",
    "trackColor": "debb7b",
    ...
  },
  "actions": [
    { "floor": 5, "eventType": "SetSpeed", "beatsPerMinute": 240 },
    { "floor": 10, "eventType": "Twirl" },
    ...
  ],
  "decorations": [...]
}
```

Alternatively, `pathData` encodes angles as single characters (R=0°, U=90°, L=180°, D=270°, etc.).

## Distribution

- **Cargo**: `cargo install adofai-cli` (binary name: `adofai`)
- **npm**: `npm install -g adofai-terminal` (wrapper that downloads the Rust binary)
- **Homebrew**: `brew install 3289david/tap/adofai-cli`
- **Binary releases**: GitHub Releases (cross-compiled for macOS, Linux, Windows)
- **Website**: GitHub Pages at `3289david.github.io/adofai-cli`
