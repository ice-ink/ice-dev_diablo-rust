//! A simple 2D follow camera.
//!
//! The camera stores a world-space `offset` for the top-left corner of the
//! screen. `world_to_screen` subtracts that offset, translating world
//! coordinates into screen coordinates for drawing.

use macroquad::prelude::Vec2;

pub struct Camera {
    /// World-space position of the screen's top-left corner.
    pub offset: Vec2,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            offset: Vec2::new(0.0, 0.0),
        }
    }

    /// Move the camera toward `target` (typically the player) with smooth
    /// easing, then clamp it so it never scrolls past the world edges.
    pub fn follow(&mut self, target: Vec2, dt: f32, world_size: Vec2, screen_size: Vec2) {
        // To center the target, the top-left should be target minus half a
        // screen in each axis.
        let desired = target - screen_size * 0.5;

        // Exponential easing: move a fixed fraction of the remaining distance
        // each frame. Scaling the fraction by `dt` keeps it frame-rate
        // independent. `.min(1.0)` guarantees we never overshoot.
        let t = (5.0 * dt).min(1.0);
        self.offset = self.offset + (desired - self.offset) * t;

        // Clamp so the camera never shows empty space past the world edge. If
        // the world is smaller than the screen, the clamp range collapses to
        // 0, which centers the (small) world.
        let max_x = (world_size.x - screen_size.x).max(0.0);
        let max_y = (world_size.y - screen_size.y).max(0.0);
        self.offset.x = self.offset.x.clamp(0.0, max_x);
        self.offset.y = self.offset.y.clamp(0.0, max_y);
    }

    /// Convert a world-space position into a screen-space position.
    pub fn world_to_screen(&self, world: Vec2) -> Vec2 {
        world - self.offset
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn world_to_screen_subtracts_offset() {
        let cam = Camera {
            offset: Vec2::new(100.0, 200.0),
        };
        let screen = cam.world_to_screen(Vec2::new(150.0, 260.0));
        assert!((screen.x - 50.0).abs() < 0.001);
        assert!((screen.y - 60.0).abs() < 0.001);
    }

    #[test]
    fn follow_clamps_when_world_fits_screen() {
        let mut cam = Camera::new();
        // World exactly the size of the screen: the camera must stay at 0,0.
        cam.follow(
            Vec2::new(999.0, 999.0),
            1.0,
            Vec2::new(1280.0, 720.0),
            Vec2::new(1280.0, 720.0),
        );
        assert!((cam.offset.x - 0.0).abs() < 0.001);
        assert!((cam.offset.y - 0.0).abs() < 0.001);
    }

    #[test]
    fn follow_does_not_scroll_past_edges() {
        let mut cam = Camera::new();
        let world = Vec2::new(2000.0, 2000.0);
        let screen = Vec2::new(1280.0, 720.0);
        // Target far beyond the world; the offset must clamp to world-screen.
        cam.follow(Vec2::new(5000.0, 5000.0), 1.0, world, screen);
        assert!(cam.offset.x <= world.x - screen.x + 0.001);
        assert!(cam.offset.y <= world.y - screen.y + 0.001);
    }
}
