# v0_01 — First runnable slice

The first alpha version: a top-down 2D map you can walk around in, built with
[macroquad](https://macroquad.rs/).

What it does:

- Opens a window and runs a frame-rate-independent game loop.
- Draws a tiled map (floor + walls) with a border and a few interior walls.
- Lets you move a player square with WASD/arrow keys, with wall collision.
- A camera follows the player smoothly and stays within the map.

## Quickstart

```sh
cargo run --release
```

- For controls and a full user guide, see [`USAGE.md`](USAGE.md).
- To understand how the code works, see [`LEARN.md`](LEARN.md).
- For what changed, see [`CHANGELOG.md`](CHANGELOG.md).
- For where this is going, see [`ROADMAP.md`](ROADMAP.md).
