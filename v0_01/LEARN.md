# v0_01 — How the code works (a guided tour)

This is the "learn by reading" companion to the source. It assumes you know
programming fundamentals (variables, functions, loops, types) and want to see
how they map to Rust and to a game loop. Read the files in the suggested order;
each section points at the real code.

## 1. The big picture: `src/main.rs`

The entry point is `main`, but not the `fn main()` you may expect. macroquad
uses an *async* main plus an attribute macro:

```rust
#[macroquad::main(window_conf)]
async fn main() { ... }
```

- `#[macroquad::main(...)]` tells macroquad to set up the window and run our
  function as the game. The `window_conf` argument is a function returning
  window settings.
- `async fn main()` and `next_frame().await` let macroquad drive the loop: each
  `next_frame().await` pauses our function until it is time to draw the next
  frame.

Everything else in `main` is simply: build the world, then
`loop { read time, update, draw, wait }`.

Rust concept: **attributes** (`#[...]`) are metadata that trigger code
generation; here the attribute turns a plain function into an app.

Primary source: macroquad introduction — https://macroquad.rs/articles/

## 2. Constants: `src/config.rs`

Every "magic number" is a named `const` here (tile size, window size, speeds,
colors). `const` values are computed at compile time. Keeping them in one file
is a discipline that makes the game easy to tune.

Rust concept: `pub const NAME: f32 = 64.0;` — a compile-time constant with an
explicit type, exported with `pub` so other modules can use it.

## 3. Modules and visibility

At the top of `src/main.rs`:

```rust
mod camera;
mod config;
mod map;
mod player;
```

Each `mod` pulls in a same-named file (`map` → `map.rs`). Rust items are
private by default; `pub` exposes them. `use crate::map::Map;` brings a name
into scope.

Rust concept: **modules** and **visibility**. See the Rust Book chapter
"Managing Growing Projects":
https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html

## 4. The map: `src/map.rs`

Tiles are an enum:

```rust
pub enum Tile { Floor, Wall }
```

An **enum** models "one of a fixed set of values".
`#[derive(Clone, Copy, PartialEq, Debug)]` auto-implements common traits:
`Copy` makes tiles cheap to copy, `PartialEq` lets us compare with `==`, and
`Debug` lets us print them in tests.

The map stores tiles in a flat `Vec<Tile>` (row-major), so tile `(x, y)` lives
at index `y * width + x`. `Vec` is Rust's growable array. `tile_at` returns
`Option<Tile>` — `Some(tile)` in-bounds, `None` out-of-bounds — Rust's way of
saying "maybe nothing" without a null value.

`is_wall` treats out-of-bounds as solid:

```rust
pub fn is_wall(&self, x: i32, y: i32) -> bool {
    self.tile_at(x, y) != Some(Tile::Floor)
}
```

`collides` is the collision test: it converts a rectangle in pixel coordinates
into a range of tiles and checks each one. This "check every overlapped tile"
loop is the simplest form of tile collision.

Rust concepts: `enum`, `Vec`, `Option`, `match`, and `impl` blocks (methods).
See the Rust Book chapters 5, 6, and 8.

## 5. The player: `src/player.rs`

Two ideas worth understanding:

**Normalization** (`normalized`): a direction like `(1, 1)` has length √2, so
raw movement would be ~41% faster diagonally. Dividing the vector by its length
makes it length 1. `Vec2` is macroquad's 2D vector type (an `x` and a `y`).

**Axis-separated collision** (`apply_movement`): move along X, test, and roll
back if blocked; then move along Y, test, roll back. Doing both axes at once
would leave the player glued to a wall; doing them separately lets the player
slide along it. This is a classic game-feel detail.

Rust concepts: **methods** (`&mut self` mutates the receiver), the **borrow
checker** (a `&Map` borrow lets us read the map without taking ownership), and
**references** (`&` = shared borrow, `&mut` = mutable borrow).

## 6. The camera: `src/camera.rs`

The camera holds a world-space `offset` for the top-left of the screen.
`world_to_screen` subtracts it. `follow` eases the offset toward
`target - screen/2` (centering) and clamps it to `[0, world - screen]` so we
never scroll past the map edge. Exponential easing — `offset += (desired -
offset) * t` — moves a fixed fraction of the remaining distance each frame,
which looks smooth and is frame-rate independent when `t` scales with `dt`.

Rust concept: `f32::max`, `f32::clamp`, and small structs with methods.

## 7. Delta time and frame-rate independence

`get_frame_time()` returns the seconds since the last frame (`dt`). Movement
uses `velocity * dt`, so a player moving 200 px/s moves the same distance per
second whether the game runs at 30 or 144 FPS. This is why `update` takes `dt`.

Primary source: "Fix Your Timestep" —
https://gafferongames.com/post/fix_your_timestep/

## 8. Tests

Each file has a `#[cfg(test)] mod tests` block. Run them with:

```sh
cargo test
```

`#[test]` marks a function as a test; `assert!` / `assert_eq!` check
conditions. The movement and collision code is factored to be testable without
a window or keyboard, which is why `apply_movement` is separate from
`read_input`.

Rust concept: the built-in test framework. See the Rust Book chapter 11:
https://doc.rust-lang.org/book/ch11-00-testing.html

## Suggested reading order

1. `src/config.rs`
2. `src/map.rs`
3. `src/camera.rs`
4. `src/player.rs`
5. `src/main.rs` (ties it all together)
