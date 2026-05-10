<p align="center">
  <img src="https://img.shields.io/badge/rust-1.70%2B-orange?logo=rust" alt="Rust">
  <img src="https://img.shields.io/badge/license-MIT-blue" alt="License">
  <img src="https://img.shields.io/badge/platform-macOS%20%7C%20Linux%20%7C%20Windows-green" alt="Platform">
  <img src="https://img.shields.io/npm/v/adofai-terminal?color=red" alt="npm">
</p>

<h1 align="center">ADOFAI CLI</h1>
<h3 align="center">Cinematic Terminal-Based A Dance of Fire and Ice Engine</h3>

<p align="center">
  Play, edit, and create ADOFAI levels entirely in your terminal.<br>
  Shader-like effects. 120 FPS rendering. Braille graphics. Single binary.
</p>

---

## What is this?

ADOFAI CLI is a terminal-native engine for [A Dance of Fire and Ice](https://store.steampowered.com/app/977950/A_Dance_of_Fire_and_Ice/). It reads and writes real `.adofai` files, renders levels with cinematic post-processing effects, and includes a full vim-style map editor — all running inside your terminal.

This is not a GUI clone. It embraces terminal constraints as a visual style: scanlines, ANSI pulse, unicode glow, monospace aesthetics, and braille graphics create something that looks better *because* it's a terminal.

```
      ●────●────●
                ╲
                 ●
                ╱
      ●────●
```

## Installation

### Homebrew (macOS/Linux)

```bash
brew tap danwoo/tap
brew install adofai-cli
```

### npm

```bash
npm install -g adofai-terminal
```

### Cargo (from source)

```bash
cargo install adofai-cli
```

### Binary download

Grab the latest release from [GitHub Releases](https://github.com/3289david/adofai-cli/releases).

## Quick Start

```bash
# Create a new level
adofai new --bpm 180

# Edit the level
adofai edit untitled.adofai

# Play the level
adofai play untitled.adofai

# Play with a theme
adofai play level.adofai --theme neon --auto-play

# Analyze a level
adofai analyze level.adofai
```

## Features

### Player

- Real-time orbit physics with accurate planet rotation
- Timing system with Perfect/Great/Good/Early/Late/Miss accuracy
- Scoring and combo tracking
- Auto-play mode
- Full .adofai event support (BPM changes, twirls, camera, flash, shake, bloom)

### Editor

- Vim-style keybindings (hjkl navigation, : commands, modes)
- Insert mode with directional tile placement
- Event editing (BPM, twirl, flash, camera, shake)
- Branch-based undo tree (not just linear undo)
- Mouse support (click to select, scroll to zoom)
- Real-time preview (press Space/Tab to test play instantly)
- Full .adofai read/write

### Rendering Engine

- Custom framebuffer with diff rendering (only changed cells redrawn)
- 120 FPS target with delta timing
- Unicode pixel engine: block characters (`█▓▒░`), braille graphics (`⣿`), subpixel rendering (`▀▄`)
- Post-processing pipeline:
  - **Bloom** — bright cells glow into neighbors
  - **Scanlines** — alternating row dimming
  - **CRT distortion** — chromatic aberration
  - **Vignette** — edge darkening
  - **Glitch** — row displacement + color corruption
  - **Flash** — full-screen color overlay
  - **Screen shake** — buffer offset
  - **Motion blur** — temporal blending

### Particle System

6 particle styles: Spark, Fire, Ice, Hit, Trail, Rainbow. Gravity, velocity, lifetime, color interpolation. Hit feedback creates particle bursts.

### Camera System

- Smooth follow with configurable speed
- Zoom, rotation, pulse
- Screen shake with decay
- Supports all ADOFAI camera events

### Audio

Built-in audio engine with rodio. Supports MP3, WAV, OGG, FLAC. Hitsound playback on every input.

### Themes

7 built-in themes, each with unique post-processing:

| Theme | Style | Effects |
|-------|-------|---------|
| `neon` | Cyberpunk | Bloom, Vignette |
| `retro` | Green terminal | Scanlines, CRT, Vignette |
| `minimal` | Clean white | None |
| `fire` | Orange/red | Bloom, Vignette |
| `ice` | Blue/cyan | Bloom, Vignette |
| `synthwave` | Pink/purple | Scanlines, Bloom, Vignette |
| `matrix` | Green rain | Scanlines, Bloom, CRT, Vignette |

```bash
adofai play level.adofai --theme synthwave
```

### Level Analyzer

```bash
adofai analyze level.adofai
```

Shows: BPM range, angle distribution histogram, event summary, density graph, twirl count, decoration count.

## Commands

| Command | Description |
|---------|-------------|
| `adofai play <file>` | Play a level |
| `adofai edit <file>` | Open the editor |
| `adofai new` | Create a new level |
| `adofai render <file>` | Auto-play render |
| `adofai replay <file>` | View a replay |
| `adofai theme <name>` | Theme info |
| `adofai analyze <file>` | Level analysis |

### Play options

```
--start <n>      Start from tile N
--auto-play      Enable auto-play
--theme <name>   Apply a visual theme
```

### New options

```
--output <path>  Output file path
--bpm <n>        Starting BPM (default: 120)
```

## Editor Commands

The editor uses vim-style modes:

**Normal mode:**
- `h/j/k/l` or arrows — Navigate
- `a` — Add tile
- `d` — Delete tile
- `r/R` — Rotate ±45°
- `t` — Toggle twirl
- `i` — Enter insert mode
- `e` — Enter event mode
- `:` — Command mode
- `Space/Tab` — Preview play
- `+/-` — Zoom
- `u` — Undo

**Command mode (`:`):**
- `:bpm 240` — Set BPM
- `:twirl` — Toggle twirl
- `:planet multi 3` — Multi-planet
- `:song My Song` — Set title
- `:artist Name` — Set artist
- `:offset 100` — Set offset
- `:w` / `:wq` / `:q` / `:q!` — Save/quit

## Supported ADOFAI Events

| Event | Status |
|-------|--------|
| SetSpeed (BPM/Multiplier) | Full |
| Twirl | Full |
| MultiPlanet | Full |
| Flash | Full |
| MoveCamera | Full |
| ShakeScreen | Full |
| Bloom | Full |
| SetFilter | Full |
| SetTrackColor | Full |
| Pause | Full |
| Hold | Full |
| Checkpoint | Full |
| PositionTrack | Full |
| ScaleRadius | Full |
| AddDecoration | Parsed |
| SetText | Parsed |
| CustomBackground | Parsed |
| HallOfMirrors | Parsed |
| ScreenTile/Scroll | Parsed |
| RepeatEvents | Parsed |
| Hitsound | Parsed |

## Architecture

```
src/
├── main.rs           CLI entry point (clap)
├── renderer/
│   ├── framebuffer   Cell-based terminal buffer
│   ├── pipeline      Render pipeline orchestration
│   ├── post_process  Bloom, CRT, scanlines, glitch, flash, shake
│   ├── particles     6 particle styles with physics
│   └── unicode_pixel Braille + block + subpixel rendering
├── engine/
│   ├── game          Main game loop (120 FPS)
│   ├── tile          Tile geometry and visual representation
│   ├── planet        Orbit physics and multi-planet support
│   ├── timing        Beat synchronization and hit accuracy
│   └── input         Keyboard and mouse input handling
├── editor/
│   ├── editor_state  Full TUI editor with vim modes
│   └── undo          Branch-based undo tree
├── parser/
│   ├── adofai        .adofai file parser and serializer
│   └── events        Complete event type definitions
├── camera/           Cinematic camera (zoom, rotate, shake, pulse)
├── effects/          Visual state management and theme system
├── audio/            Rodio-based audio engine
├── replay/           Record and playback system
└── cli/              Command handlers for each subcommand
```

## Building from Source

```bash
git clone https://github.com/3289david/adofai-cli.git
cd adofai-cli
cargo build --release
./target/release/adofai --help
```

The release binary is optimized with LTO and stripped — typically under 5MB.

## Requirements

- A terminal that supports:
  - 24-bit (true color) — most modern terminals
  - Unicode — for block and braille rendering
  - Raw mode — for keyboard input
- Optional: audio output device (for music and hitsounds)

## License

MIT License. See [LICENSE](LICENSE).

## Credits

- [A Dance of Fire and Ice](https://store.steampowered.com/app/977950/) by 7th Beat Games
- Built with [Rust](https://www.rust-lang.org/), [crossterm](https://github.com/crossterm-rs/crossterm), [ratatui](https://github.com/ratatui/ratatui), [rodio](https://github.com/RustAudio/rodio)
