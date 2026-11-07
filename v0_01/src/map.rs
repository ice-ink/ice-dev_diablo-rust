//! The tile map: a 2D grid of tiles plus the collision logic that goes with it.
//!
//! Tiles are stored in a single flat `Vec<Tile>` in row-major order, meaning
//! the tile at column `x`, row `y` lives at index `y * width + x`. Storing a
//! 2D grid in one contiguous array keeps neighboring tiles adjacent in memory
//! and is a common idiom in games.

use macroquad::prelude::{Vec2, draw_rectangle, screen_height, screen_width};

use crate::camera::Camera;
use crate::config::{FLOOR_COLOR, TILE_SIZE, WALL_COLOR};

/// The two kinds of tile in v0_01. Later versions will add more (doors, etc.).
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Tile {
    Floor,
    Wall,
}

/// A fixed-size, rectangular tile map.
pub struct Map {
    width: i32,
    height: i32,
    tiles: Vec<Tile>,
}

impl Map {
    /// An empty map (all floor, no border). Useful for tests.
    pub fn new(width: i32, height: i32) -> Self {
        let tiles = vec![Tile::Floor; (width * height) as usize];
        Self {
            width,
            height,
            tiles,
        }
    }

    /// A hand-built demo level: a wall border plus a few interior walls and
    /// pillars so collision is easy to see.
    pub fn demo(width: i32, height: i32) -> Self {
        let mut map = Self::new(width, height);

        // Border walls.
        for x in 0..width {
            map.set_tile(x, 0, Tile::Wall);
            map.set_tile(x, height - 1, Tile::Wall);
        }
        for y in 0..height {
            map.set_tile(0, y, Tile::Wall);
            map.set_tile(width - 1, y, Tile::Wall);
        }

        // A vertical wall down the middle with a door-sized gap.
        let mid = width / 2;
        for y in 2..(height - 2) {
            map.set_tile(mid, y, Tile::Wall);
        }
        map.set_tile(mid, height / 2, Tile::Floor);

        // A few standalone pillars in the open areas.
        map.set_tile(3, 3, Tile::Wall);
        map.set_tile(4, 3, Tile::Wall);
        map.set_tile(width - 4, height - 4, Tile::Wall);
        map.set_tile(width - 4, height - 5, Tile::Wall);

        map
    }

    /// Set the tile at `(x, y)`, silently ignoring out-of-bounds coordinates.
    pub fn set_tile(&mut self, x: i32, y: i32, tile: Tile) {
        if let Some(i) = self.index(x, y) {
            self.tiles[i] = tile;
        }
    }

    /// The tile at `(x, y)`, or `None` if the coordinate is outside the map.
    pub fn tile_at(&self, x: i32, y: i32) -> Option<Tile> {
        self.index(x, y).map(|i| self.tiles[i])
    }

    /// True if the tile is solid — a wall or outside the map entirely.
    /// Treating out-of-bounds as solid keeps entities from leaving the map.
    pub fn is_wall(&self, x: i32, y: i32) -> bool {
        self.tile_at(x, y) != Some(Tile::Floor)
    }

    /// True if any tile overlapped by the rectangle (top-left `x,y` + `w,h`)
    /// is solid. This is the core collision test used by the player.
    pub fn collides(&self, x: f32, y: f32, w: f32, h: f32) -> bool {
        let min_x = (x / TILE_SIZE).floor() as i32;
        let max_x = ((x + w) / TILE_SIZE).floor() as i32;
        let min_y = (y / TILE_SIZE).floor() as i32;
        let max_y = ((y + h) / TILE_SIZE).floor() as i32;

        for ty in min_y..=max_y {
            for tx in min_x..=max_x {
                if self.is_wall(tx, ty) {
                    return true;
                }
            }
        }
        false
    }

    /// The size of the whole map in world pixels.
    pub fn world_size(&self) -> Vec2 {
        Vec2::new(
            self.width as f32 * TILE_SIZE,
            self.height as f32 * TILE_SIZE,
        )
    }

    /// Draw only the tiles visible on screen (view culling).
    pub fn draw(&self, camera: &Camera) {
        let screen = Vec2::new(screen_width(), screen_height());

        // Convert the visible screen area into a range of tile coordinates.
        let min_x = (camera.offset.x / TILE_SIZE).floor() as i32;
        let min_y = (camera.offset.y / TILE_SIZE).floor() as i32;
        let max_x = ((camera.offset.x + screen.x) / TILE_SIZE).ceil() as i32;
        let max_y = ((camera.offset.y + screen.y) / TILE_SIZE).ceil() as i32;

        for ty in min_y..=max_y {
            for tx in min_x..=max_x {
                let color = match self.tile_at(tx, ty) {
                    Some(Tile::Floor) => FLOOR_COLOR,
                    Some(Tile::Wall) => WALL_COLOR,
                    None => continue, // off-map: nothing to draw
                };
                let sx = tx as f32 * TILE_SIZE - camera.offset.x;
                let sy = ty as f32 * TILE_SIZE - camera.offset.y;
                draw_rectangle(sx, sy, TILE_SIZE, TILE_SIZE, color);
            }
        }
    }

    /// Convert tile coordinates into a flat array index, or `None` if the
    /// coordinates are out of bounds.
    fn index(&self, x: i32, y: i32) -> Option<usize> {
        if x < 0 || y < 0 || x >= self.width || y >= self.height {
            return None;
        }
        Some((y * self.width + x) as usize)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_map_is_all_floor() {
        let map = Map::new(4, 4);
        // `new` adds no border, so every tile including corners is floor.
        assert!(!map.is_wall(2, 2));
        assert!(!map.is_wall(0, 0));
    }

    #[test]
    fn out_of_bounds_is_solid() {
        let map = Map::new(4, 4);
        assert!(map.is_wall(-1, 0));
        assert!(map.is_wall(4, 0));
    }

    #[test]
    fn demo_has_border_walls() {
        let map = Map::demo(10, 10);
        assert!(map.is_wall(0, 0));
        assert!(map.is_wall(9, 9));
        assert!(!map.is_wall(1, 1));
    }

    #[test]
    fn collides_detects_wall_overlap() {
        let map = Map::demo(10, 10);
        // A rectangle overlapping the top-left border wall.
        assert!(map.collides(0.0, 0.0, TILE_SIZE, TILE_SIZE));
        // A rectangle sitting in open floor (tile 1,1).
        assert!(!map.collides(TILE_SIZE, TILE_SIZE, TILE_SIZE, TILE_SIZE));
    }
}
