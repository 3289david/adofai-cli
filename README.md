<p align="center">
  <br>
  <code>
  ╔═══════════════════════════════════════════════════════════╗
  ║                                                           ║
  ║     █████╗ ██████╗  ██████╗ ███████╗ █████╗ ██╗          ║
  ║    ██╔══██╗██╔══██╗██╔═══██╗██╔════╝██╔══██╗██║          ║
  ║    ███████║██║  ██║██║   ██║█████╗  ███████║██║          ║
  ║    ██╔══██║██║  ██║██║   ██║██╔══╝  ██╔══██║██║          ║
  ║    ██║  ██║██████╔╝╚██████╔╝██║     ██║  ██║██║          ║
  ║    ╚═╝  ╚═╝╚═════╝  ╚═════╝ ╚═╝     ╚═╝  ╚═╝╚═╝          ║
  ║               ░█████╗░██╗░░░░░██╗                         ║
  ║               ██╔══██╗██║░░░░░██║                         ║
  ║               ██║░░╚═╝██║░░░░░██║                         ║
  ║               ██║░░██╗██║░░░░░██║                         ║
  ║               ╚█████╔╝███████╗██║                         ║
  ║               ░╚════╝░╚══════╝╚═╝                         ║
  ║                                                           ║
  ╚═══════════════════════════════════════════════════════════╝
  </code>
</p>

<p align="center">
  <strong>Cinematic Terminal-Based A Dance of Fire and Ice Engine</strong>
</p>

<p align="center">
  <a href="https://github.com/3289david/adofai-cli/actions"><img src="https://img.shields.io/github/actions/workflow/status/3289david/adofai-cli/ci.yml?label=build&logo=github" alt="Build"></a>
  <img src="https://img.shields.io/badge/rust-1.70%2B-orange?logo=rust&logoColor=white" alt="Rust">
  <img src="https://img.shields.io/badge/platform-macOS%20%7C%20Linux%20%7C%20Windows-brightgreen" alt="Platform">
  <img src="https://img.shields.io/badge/fps-120-blueviolet" alt="120 FPS">
  <img src="https://img.shields.io/badge/binary-1.1MB-blue" alt="Binary Size">
  <a href="https://github.com/3289david/adofai-cli/blob/master/LICENSE"><img src="https://img.shields.io/badge/license-MIT-green" alt="License"></a>
  <img src="https://img.shields.io/npm/v/adofai-terminal?color=red&logo=npm" alt="npm">
</p>

<p align="center">
  <a href="https://3289david.github.io/adofai-cli">Website</a> &middot;
  <a href="https://3289david.github.io/adofai-cli#docs">Documentation</a> &middot;
  <a href="https://github.com/3289david/adofai-cli/releases">Download</a>
</p>

---

## What is this?

**ADOFAI CLI** is a terminal-native game engine for [A Dance of Fire and Ice](https://store.steampowered.com/app/977950/A_Dance_of_Fire_and_Ice/). It reads and writes real `.adofai` files, renders levels with cinematic post-processing effects, and includes a full vim-style map editor — all inside your terminal at **120 FPS**.

This is **not** a GUI clone. It embraces terminal constraints as a **visual style**:

> *"The terminal is not a limitation — it's an aesthetic."*

```
     ┌─────────────────────────────────────────────────┐
     │                                                 │
     │        ◉                                        │
     │       ╱                                         │
     │      ●────●────●────●                           │
     │                     ╲                           │
     │                      ●────●                     │
     │                           ╲                     │
     │                            ●                    │
     │                           ╱                     │
     │                     ●────●                      │
     │                                                 │
     │  BPM:180  Tile:12/64  ██████░░░░  Score:1200   │
     └─────────────────────────────────────────────────┘
```

---

## Demo

```
$ adofai play examples/demo.adofai --theme neon

╭─────────────────────────────────╮
│     ADOFAI Terminal Engine       │
├─────────────────────────────────┤
│ Song: Terminal Demo              │
│ Artist: ADOFAI CLI              │
│ BPM: 160                        │
│ Tiles: 33                       │
│ Theme: neon                     │
╰─────────────────────────────────╯

  Controls:
  Space/J/K/D/F/Arrows = Hit
  P/Esc = Pause
  R = Restart
  A = Toggle Auto-play
  Q = Quit

  Press any key to start...
```

```
$ adofai analyze examples/demo.adofai

╭──────────────────────────────────────╮
│          LEVEL ANALYSIS              │
├──────────────────────────────────────┤
│ Song: Terminal Demo                   │
│ Artist: ADOFAI CLI                    │
│ BPM: 160                             │
│ Tiles: 33                            │
├──────────────────────────────────────┤
│ Angle Distribution:                  │
│    0°: ██████████████             19 │
│   45°: █                           2 │
│   90°: ██                          3 │
│  180°: ██                          3 │
│  270°: ██                          3 │
│  315°: █                           2 │
├──────────────────────────────────────┤
│ BPM Changes:                         │
│   Range: 160 - 240 BPM              │
│   Changes: 3                         │
│ Twirls: 2                            │
╰──────────────────────────────────────╯
```

---

## Installation

### Homebrew (macOS / Linux) — Recommended

```bash
brew tap 3289david/tap
brew install adofai-cli
```

### npm (Cross-platform)

```bash
npm install -g adofai-terminal
```

### Cargo (From source)

```bash
cargo install adofai-cli
```

### Download Binary

Grab a pre-built binary from [GitHub Releases](https://github.com/3289david/adofai-cli/releases) — zero dependencies, single file.

### Verify Installation

```bash
adofai --version
# adofai 1.0.0
```

---

## Quick Start

```bash
# Create a new level at 180 BPM
adofai new --bpm 180

# Open the vim-style map editor
adofai edit untitled.adofai

# Play it with neon post-processing
adofai play untitled.adofai --theme neon

# Auto-play to watch the level
adofai play level.adofai --auto-play

# Get full level analysis
adofai analyze level.adofai

# List available themes
adofai theme neon
```

---

## Features

### Rendering Engine

The custom terminal renderer is the heart of ADOFAI CLI. It's not built on top of a TUI library — it's a **purpose-built framebuffer engine** with a full post-processing pipeline.

| Feature | Description |
|---------|-------------|
| **Diff Rendering** | Only changed cells are redrawn — 120 FPS with minimal I/O |
| **24-bit Color** | Full RGB color support via ANSI escape sequences |
| **Unicode Pixels** | Block chars (`█▓▒░`), braille dots (`⣿`), subpixel (`▀▄`) |
| **Bloom** | Bright cells emit glow into neighboring empty cells |
| **Scanlines** | Alternating row dimming for retro CRT feel |
| **Chromatic Aberration** | RGB channel separation for CRT distortion |
| **Vignette** | Edge darkening for cinematic focus |
| **Glitch** | Row displacement + random color corruption |
| **Screen Flash** | Full-screen color overlay with fade |
| **Screen Shake** | Framebuffer offset with exponential decay |
| **Particles** | 6 styles with gravity, velocity, lifetime, color interpolation |

### Player

```
┌────────────────────────────────────────────────┐
│  Terminal Demo - ADOFAI CLI        BPM:240     │
│                                                │
│            ◉                                   │
│           ╱                                    │
│     ●────●────●            PERFECT!            │
│                ╲                               │
│                 ●────●                         │
│                                                │
│  ████████████████░░░░░░░░░░░░░░░░░░░░░░░░░░░  │
│  Score:4800   Combo:48               [PLAY]    │
└────────────────────────────────────────────────┘
```

- **Orbit Physics** — accurate planet rotation with multi-planet support
- **Timing System** — Perfect / Great / Good / Early / Late / Miss (25ms windows)
- **Scoring** — combo multiplier, max combo tracking, end-of-level breakdown
- **Auto-Play** — watch the level play itself with particle trails
- **All Events** — BPM changes, twirls, camera moves, flash, shake, bloom

### Editor

```
┌────────────────────────────────────────────────┐
│  ADOFAI Editor - demo.adofai [+]    INSERT     │
│  Tile:12/33 BPM:200 Angle:45°                  │
│                                                │
│     ·  ·  ·  ·  ·  ·  ·  ·  ·  ·  ·  ·     │
│     ·  ·  ●────●────[●]───●  ·  ·  ·  ·     │
│     ·  ·  ·  ·  ·  ·  · ↺╲  ·  ·  ·  ·     │
│     ·  ·  ·  ·  ·  ·  ·  ·●  ·  ·  ·  ·     │
│     ·  ·  ·  ·  ·  ·  ·  ·  ·  ·  ·  ·     │
│                                                │
│ Events                                         │
│   SetSpeed beatsPerMinute:200                  │
│   Twirl                                        │
│                                                │
│ r:right u:up l:left d:down ESC:back            │
└────────────────────────────────────────────────┘
```

- **Vim Modes** — Normal, Insert, Command, Event, Timeline, Preview
- **Directional Insert** — press `r/u/l/d/e/q/z/c` to place tiles in 8 directions
- **Branch Undo Tree** — full history branching, not just linear undo/redo
- **Mouse Support** — click to select tiles, scroll to zoom
- **Instant Preview** — press `Space` to test-play from the editor
- **Full Event Editing** — BPM, twirl, flash, camera, shake via `:` commands

### Particle System

6 built-in styles, each with physics simulation:

| Style | Visual | Use |
|-------|--------|-----|
| **Spark** | `·•*✦✧` in orange/gold | Perfect hits |
| **Fire** | `▓▒░█` rising | Fire theme, intense sections |
| **Ice** | `❄✦·•` floating | Ice theme |
| **Hit** | `●○◎◉` expanding | Every hit feedback |
| **Trail** | `·∙°` fading | Auto-play breadcrumbs |
| **Rainbow** | `✦✧★☆◆` cycling | Special moments |

### Camera System

- **Smooth Follow** — exponential interpolation to active tile
- **Zoom** — supports all ADOFAI zoom events
- **Rotation** — camera rotation with smooth easing
- **Pulse** — beat-synced zoom oscillation
- **Shake** — exponential decay screen shake on events

### Audio Engine

Built on `rodio` for cross-platform audio:

- Supports **MP3, WAV, OGG, FLAC**
- Per-hit **hitsound** playback (sine wave beep)
- Volume control, pause/resume
- Sync with timing engine

### Themes

7 hand-crafted visual themes, each with unique post-processing:

| Theme | Palette | Effects | Vibe |
|-------|---------|---------|------|
| `neon` | Cyan + Magenta | Bloom, Vignette | Cyberpunk nightclub |
| `retro` | Green monochrome | Scanlines, CRT, Vignette | 1980s terminal |
| `minimal` | White on dark | None | Clean and focused |
| `fire` | Orange + Red | Bloom, Vignette | Burning intensity |
| `ice` | Blue + Cyan | Bloom, Vignette | Frozen elegance |
| `synthwave` | Pink + Purple | Scanlines, Bloom, Vignette | Outrun aesthetic |
| `matrix` | Green on black | Scanlines, Bloom, CRT, Vignette | Digital rain |

```bash
# Try them all
adofai theme neon
adofai theme retro
adofai theme synthwave
adofai play level.adofai --theme matrix
```

### Level Analyzer

Full statistical breakdown of any `.adofai` file:

- Angle distribution histogram
- BPM range and change points
- Event type summary with counts
- Tile density graph by section
- Twirl and decoration counts
- Difficulty assessment

---

## Commands Reference

| Command | Description | Example |
|---------|-------------|---------|
| `play <file>` | Play a level | `adofai play level.adofai --theme neon` |
| `edit <file>` | Open map editor | `adofai edit level.adofai` |
| `new` | Create empty level | `adofai new --bpm 200 -o my.adofai` |
| `render <file>` | Auto-play render | `adofai render level.adofai` |
| `replay <file>` | View replay | `adofai replay run.rep --speed 0.5` |
| `theme <name>` | Show theme info | `adofai theme synthwave` |
| `analyze <file>` | Level statistics | `adofai analyze level.adofai` |

### Play Options

```
-s, --start <N>     Start from tile N
-a, --auto-play     Enable auto-play mode
-t, --theme <NAME>  Apply visual theme
```

### New Options

```
-o, --output <PATH>  Output file path (default: untitled.adofai)
-b, --bpm <N>        Starting BPM (default: 120)
```

---

## Editor Keybindings

### Normal Mode

| Key | Action |
|-----|--------|
| `h` `j` `k` `l` / Arrows | Navigate tiles |
| `a` | Add tile (right) |
| `d` / `Delete` | Delete tile |
| `r` / `R` | Rotate tile +45° / -45° |
| `t` | Toggle twirl |
| `i` | Enter insert mode |
| `e` | Enter event mode |
| `:` | Enter command mode |
| `Space` / `Tab` | Preview (test play) |
| `+` / `-` | Zoom in/out |
| `g` | Toggle grid |
| `u` | Undo |
| `Ctrl+S` | Save |
| `Ctrl+Z` / `Ctrl+Y` | Undo / Redo |

### Insert Mode

| Key | Direction | Angle |
|-----|-----------|-------|
| `r` | Right | 0° |
| `e` | Up-Right | 45° |
| `u` | Up | 90° |
| `q` | Up-Left | 135° |
| `l` | Left | 180° |
| `z` | Down-Left | 225° |
| `d` | Down | 270° |
| `c` | Down-Right | 315° |
| `Esc` | Back to normal | — |

### Command Mode (`:`)

```vim
:bpm 240             Set BPM at current tile
:twirl               Toggle twirl
:planet multi 3      Set multi-planet count
:song My Song        Set song title
:artist DJ Name      Set artist name
:offset 100          Set audio offset (ms)
:w                   Save
:wq                  Save and quit
:q                   Quit (warns if unsaved)
:q!                  Force quit without saving
:help                Show available commands
```

---

## .adofai File Support

Full compatibility with the ADOFAI level format:

### Encoding Formats

| Format | Support | Description |
|--------|---------|-------------|
| `angleData` | Full | Array of angle values |
| `pathData` | Full | Character-encoded path (R, U, L, D, E, Q, etc.) |

### Events

| Event | Status | Description |
|-------|--------|-------------|
| SetSpeed | Full | BPM and multiplier changes |
| Twirl | Full | Reverse orbit direction |
| MultiPlanet | Full | 2+ planet support |
| Flash | Full | Screen flash with color/fade |
| MoveCamera | Full | Position, zoom, rotation, easing |
| ShakeScreen | Full | Screen shake with decay |
| Bloom | Full | Glow post-processing |
| SetFilter | Full | Visual filter effects |
| SetTrackColor | Full | Track color changes |
| Pause | Full | Beat pauses |
| Hold | Full | Hold notes |
| Checkpoint | Full | Progress checkpoints |
| PositionTrack | Full | Track positioning |
| ScaleRadius | Full | Orbit radius changes |
| AddDecoration | Parsed | Decoration data preserved |
| SetText | Parsed | Text decoration data |
| CustomBackground | Parsed | Background settings |
| HallOfMirrors | Parsed | Mirror effect data |
| RepeatEvents | Parsed | Event repetition |
| Hitsound | Parsed | Custom hitsound settings |

---

## Architecture

```
                    ┌─────────────────────────────────────┐
                    │           Terminal Output            │
                    └───────────────────┬─────────────────┘
                                        │
                    ┌───────────────────▼─────────────────┐
                    │         FrameBuffer (diff)          │
                    └───────────────────┬─────────────────┘
                                        │
                    ┌───────────────────▼─────────────────┐
                    │       Post-Processing Pipeline       │
                    │  bloom → scanline → CRT → vignette  │
                    │  glitch → flash → shake             │
                    └───────────────────┬─────────────────┘
                                        │
          ┌─────────────────────────────▼────────────────────────────┐
          │                    Render Pipeline                        │
          │  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌────────┐  │
          │  │  Tiles   │  │ Planets  │  │Particles │  │  HUD   │  │
          │  └──────────┘  └──────────┘  └──────────┘  └────────┘  │
          └──────────────────────────────────────────────────────────┘
                                        │
          ┌─────────────────────────────▼────────────────────────────┐
          │                     Game Engine                           │
          │  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌────────┐  │
          │  │ Timing   │  │  Camera  │  │  Input   │  │ Visual │  │
          │  └──────────┘  └──────────┘  └──────────┘  └────────┘  │
          └──────────────────────────────────────────────────────────┘
                                        │
                    ┌───────────────────▼─────────────────┐
                    │        .adofai Parser/Writer         │
                    └─────────────────────────────────────┘
```

### Source Layout

```
src/
├── main.rs              CLI entry point (clap derive)
├── renderer/
│   ├── framebuffer.rs   Cell grid with diff rendering
│   ├── pipeline.rs      Render pipeline orchestration
│   ├── post_process.rs  Bloom, CRT, scanlines, glitch, flash, vignette
│   ├── particles.rs     6 particle styles with physics
│   └── unicode_pixel.rs Braille dots, block chars, subpixel, gradients
├── engine/
│   ├── game.rs          Main game loop (120 FPS, delta timing)
│   ├── tile.rs          Tile positions from angles, visual chars
│   ├── planet.rs        Orbit physics, multi-planet, twirl
│   ├── timing.rs        Beat sync, tile time computation, accuracy
│   └── input.rs         Key + mouse input handling
├── editor/
│   ├── editor_state.rs  Full TUI editor with 6 vim modes
│   └── undo.rs          Branch-based undo tree
├── parser/
│   ├── adofai.rs        .adofai JSON parser (handles quirks)
│   └── events.rs        All ADOFAI event type definitions
├── camera/mod.rs        Smooth follow, zoom, rotation, shake, pulse
├── effects/
│   ├── mod.rs           Visual state, color utilities
│   └── themes.rs        7 named theme presets
├── audio/mod.rs         Rodio audio engine + hitsounds
├── replay/mod.rs        Input recording + playback
└── cli/                 One module per subcommand
```

---

## Building from Source

```bash
git clone https://github.com/3289david/adofai-cli.git
cd adofai-cli
cargo build --release
./target/release/adofai --help
```

The release binary uses LTO and symbol stripping — **1.1 MB** single file, zero runtime dependencies.

### Development

```bash
cargo build              # Debug build
cargo run -- --help      # Run with args
cargo run -- play examples/demo.adofai --auto-play
cargo test               # Run tests
```

---

## Requirements

| Requirement | Details |
|-------------|---------|
| **Terminal** | 24-bit color (truecolor), Unicode support |
| **OS** | macOS, Linux, or Windows |
| **Audio** | Optional — needs output device for music/hitsounds |

**Recommended terminals:** iTerm2, Alacritty, Kitty, WezTerm, Windows Terminal, any modern terminal with truecolor.

---

## Roadmap

- [ ] Online leaderboards
- [ ] Custom theme creation (`.theme` files)
- [ ] Level sharing / import from URL
- [ ] Multiplayer spectate mode
- [ ] Beat-reactive terminal shader (music-driven effects)
- [ ] Frame-by-frame replay viewer
- [ ] Level difficulty auto-rating

---

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

```bash
# Fork, clone, then:
cargo build && cargo run -- play examples/demo.adofai --auto-play
```

---

## License

MIT License. See [LICENSE](LICENSE).

---

<p align="center">
  <strong>Built with Rust.</strong><br>
  <a href="https://github.com/crossterm-rs/crossterm">crossterm</a> &middot;
  <a href="https://github.com/ratatui/ratatui">ratatui</a> &middot;
  <a href="https://github.com/RustAudio/rodio">rodio</a> &middot;
  <a href="https://github.com/clap-rs/clap">clap</a> &middot;
  <a href="https://github.com/serde-rs/serde">serde</a>
</p>

<p align="center">
  Inspired by <a href="https://store.steampowered.com/app/977950/">A Dance of Fire and Ice</a> by 7th Beat Games.
</p>
