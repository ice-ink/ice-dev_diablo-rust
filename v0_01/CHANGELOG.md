# v0_01 — Changelog

First release.

## Added

- Cargo package `diablo-rust-v0_01` (edition 2024, dependency: `macroquad 0.4`).
- `src/config.rs` — centralized tunables (window, tile, map, player, colors).
- `src/map.rs` — tile grid (`Floor`/`Wall`), bounds-safe queries, rectangle
  collision test, view-culled drawing, and a hand-built demo level.
- `src/player.rs` — WASD/arrow input, normalized movement, axis-separated wall
  collision, and drawing.
- `src/camera.rs` — smooth follow camera with world-edge clamping and
  world↔screen transforms.
- `src/main.rs` — macroquad window config and the main game loop.
- Unit tests for map collision, movement normalization, movement blocking, and
  camera clamping.
- Documentation: `README.md`, `USAGE.md`, `LEARN.md`, `CHANGELOG.md`,
  `ROADMAP.md`.

## Decisions

- Rendering/loop via **macroquad** (simple 2D, minimal boilerplate).
- Perspective is **top-down 2D** for now (isometric deferred).
- v0_01 is a minimal runnable slice: window + map + movement + camera.
