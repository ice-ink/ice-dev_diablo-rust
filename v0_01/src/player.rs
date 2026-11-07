//! The player: input, movement, and collision resolution.
//!
//! Reading input is kept separate from the movement math so the movement logic
//! can be unit-tested without a live keyboard or window.

use macroquad::prelude::{KeyCode, Vec2, draw_rectangle, is_key_down};

use crate::camera::Camera;
use crate::config::PLAYER_COLOR;
use crate::map::Map;

/// Normalize a direction vector so diagonal movement isn't faster than
/// cardinal movement. Returns the zero vector when the input is zero.
pub fn normalized(dx: f32, dy: f32) -> Vec2 {
    let v = Vec2::new(dx, dy);
    let len = v.length();
    if len > 0.0 { v / len } else { v }
}

pub struct Player {
    /// Center of the player, in world pixels.
    pub pos: Vec2,
    size: f32,
    speed: f32,
}

impl Player {
    pub fn new(pos: Vec2, size: f32, speed: f32) -> Self {
        Self { pos, size, speed }
    }

    /// Read the keyboard and move. Called once per frame.
    pub fn update(&mut self, dt: f32, map: &Map) {
        let dir = self.read_input();
        self.apply_movement(dir, dt, map);
    }

    fn read_input(&self) -> Vec2 {
        let mut dir = Vec2::new(0.0, 0.0);
        if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
            dir.x -= 1.0;
        }
        if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
            dir.x += 1.0;
        }
        if is_key_down(KeyCode::Up) || is_key_down(KeyCode::W) {
            dir.y -= 1.0;
        }
        if is_key_down(KeyCode::Down) || is_key_down(KeyCode::S) {
            dir.y += 1.0;
        }
        dir
    }

    /// Apply movement with axis-separated collision resolution.
    ///
    /// We move along X first and only commit it if there is no collision, then
    /// do the same for Y. Testing the axes separately lets the player slide
    /// along a wall instead of getting stuck on it — a classic game-feel
    /// detail.
    pub fn apply_movement(&mut self, dir: Vec2, dt: f32, map: &Map) {
        let dir = normalized(dir.x, dir.y);
        if dir.x == 0.0 && dir.y == 0.0 {
            return;
        }

        let half = self.size / 2.0;
        // Distance to move this frame = speed (px/s) * elapsed seconds.
        let vel = dir * self.speed * dt;

        // X axis.
        let x0 = self.pos.x + vel.x - half;
        if !map.collides(x0, self.pos.y - half, self.size, self.size) {
            self.pos.x += vel.x;
        }

        // Y axis.
        let y0 = self.pos.y + vel.y - half;
        if !map.collides(self.pos.x - half, y0, self.size, self.size) {
            self.pos.y += vel.y;
        }
    }

    pub fn draw(&self, camera: &Camera) {
        let s = camera.world_to_screen(self.pos);
        let half = self.size / 2.0;
        draw_rectangle(s.x - half, s.y - half, self.size, self.size, PLAYER_COLOR);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalized_zero_stays_zero() {
        let v = normalized(0.0, 0.0);
        assert!(v.x == 0.0 && v.y == 0.0);
    }

    #[test]
    fn normalized_diagonal_has_unit_length() {
        let v = normalized(3.0, 3.0);
        assert!((v.length() - 1.0).abs() < 0.001);
    }

    #[test]
    fn normalized_axis_direction_is_unchanged() {
        let v = normalized(0.0, 5.0);
        assert!(v.x.abs() < 0.001);
        assert!((v.y - 1.0).abs() < 0.001);
    }

    #[test]
    fn movement_stops_at_wall() {
        let map = Map::demo(10, 10);
        // Start in open floor near tile (2, 2), heading right.
        let mut p = Player::new(Vec2::new(2.0 * 64.0, 2.0 * 64.0), 40.0, 100.0);
        let start_x = p.pos.x;
        // Move right in many small steps; it will run into walls.
        for _ in 0..100 {
            p.apply_movement(Vec2::new(1.0, 0.0), 0.1, &map);
        }
        assert!(p.pos.x > start_x);
        // The player must never end up overlapping a wall.
        let half = 40.0 / 2.0;
        assert!(!map.collides(p.pos.x - half, p.pos.y - half, 40.0, 40.0));
    }

    #[test]
    fn diagonal_speed_is_normalized() {
        let map = Map::new(20, 20); // all open floor
        let start = Vec2::new(5.0 * 64.0, 5.0 * 64.0);
        let mut p = Player::new(start, 40.0, 100.0);
        // One second at speed 100 should move ~100px total, split evenly.
        p.apply_movement(Vec2::new(1.0, 1.0), 1.0, &map);
        let expected = 100.0 / std::f32::consts::SQRT_2;
        assert!((p.pos.x - start.x - expected).abs() < 1.0);
        assert!((p.pos.y - start.y - expected).abs() < 1.0);
    }
}
