# v0_01 — User Guide

## Requirements

- Rust (stable). Install via https://rustup.rs
- Windows, macOS, or Linux. Developed and tested on Windows (MSVC).

## Build & run

```sh
cd v0_01
cargo run --release   # build optimized, then run
```

Useful variants:

```sh
cargo build           # compile without running
cargo run             # debug build (faster to compile, slower to run)
cargo test            # run the unit tests
cargo clippy          # extra lint checks
```

## Controls

| Key            | Action   |
|----------------|----------|
| `W` / `Up`     | Move up  |
| `S` / `Down`   | Move down |
| `A` / `Left`   | Move left |
| `D` / `Right`  | Move right |

Movement is normalized, so diagonal movement is not faster than cardinal
movement.

## What you should see

- A dark floor made of tiles, with lighter wall tiles around the border, a wall
  running down the middle (with a gap), and a few standalone pillars.
- A yellow square (the player) near the left-center.
- The camera smoothly follows the player and stops at the map edges.
- The player stops when it hits a wall, and slides along walls when moving
  diagonally into them.

## Tweaking

All tunables live in [`src/config.rs`](src/config.rs): tile size, window size,
map dimensions, player size/speed, and colors. Edit them and re-run to see the
effect.
