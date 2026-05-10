# Contributing to ADOFAI CLI

Thanks for your interest in contributing! This project is a terminal-based ADOFAI engine written in Rust.

## Getting Started

```bash
git clone https://github.com/danwoo/adofai-cli.git
cd adofai-cli
cargo build
cargo run -- --help
```

## Development Workflow

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/my-feature`
3. Make your changes
4. Test: `cargo test` and manual testing with `cargo run -- <command>`
5. Format: `cargo fmt`
6. Lint: `cargo clippy`
7. Commit and push
8. Open a Pull Request

## What to Contribute

### Good first issues

- Add new themes to `src/effects/themes.rs`
- Add new particle styles to `src/renderer/particles.rs`
- Improve angle-to-character mapping in `src/renderer/unicode_pixel.rs`
- Add more editor commands in `src/editor/editor_state.rs`

### Bigger contributions

- Implement additional ADOFAI events
- Add new post-processing effects
- Improve the replay viewer
- Add multiplayer / online leaderboard support
- Improve audio sync accuracy

## Code Style

- Run `cargo fmt` before committing
- Run `cargo clippy` and fix all warnings
- No comments unless explaining a non-obvious "why"
- Prefer clear names over comments
- Keep functions short and focused

## Architecture Rules

- All rendering goes through `FrameBuffer` — never write to stdout directly during gameplay
- State mutations in the editor must be preceded by `undo_tree.push()`
- After modifying level data, call `rebuild_tiles()`
- The render loop must stay at 120 FPS — no allocations in the hot path

## Testing

```bash
cargo test                          # Unit tests
cargo run -- new                    # Smoke test: creates valid .adofai
cargo run -- analyze <level>.adofai # Parser validation
```

Manual testing is required for the editor and player since they need a terminal.

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
