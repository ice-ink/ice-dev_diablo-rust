//! Central place for all game constants and tunables.
//!
//! Keeping every "magic number" in one file makes the game easy to adjust
//! without hunting through the code. Most of these are `const` values, which
//! are computed at compile time.

use macroquad::prelude::Color;

/// Width/height in pixels of one square tile in the world.
pub const TILE_SIZE: f32 = 64.0;

/// Logical window size (the game is drawn into this area).
pub const WINDOW_WIDTH: i32 = 1280;
pub const WINDOW_HEIGHT: i32 = 720;

/// Map size, in tiles.
pub const MAP_WIDTH: i32 = 25;
pub const MAP_HEIGHT: i32 = 20;

/// Player size (a square) and movement speed in pixels per second.
pub const PLAYER_SIZE: f32 = 40.0;
pub const PLAYER_SPEED: f32 = 200.0;

/// Tile and player colors. `Color::new` takes red/green/blue/alpha in the
/// range 0.0..=1.0.
pub const FLOOR_COLOR: Color = Color::new(0.14, 0.16, 0.20, 1.0);
pub const WALL_COLOR: Color = Color::new(0.38, 0.38, 0.44, 1.0);
pub const PLAYER_COLOR: Color = Color::new(0.85, 0.65, 0.15, 1.0);
