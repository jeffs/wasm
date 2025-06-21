use web_sys::CanvasRenderingContext2d;

use crate::{
    constants::CANVAS_SCALE,
    physics::{PointF64, VIRTUAL_SIZE, Vec2d, distance},
};

pub const SIZE: f64 = 5.0;

pub struct Ball {
    top_left: PointF64,
    velocity: Vec2d,
}

impl Ball {
    pub fn new(top_left: PointF64, velocity: Vec2d) -> Self {
        Ball { top_left, velocity }
    }

    pub fn update(&mut self, dt: f64) {
        const MAX_TOP: f64 = VIRTUAL_SIZE.height - SIZE;
        const MAX_LEFT: f64 = VIRTUAL_SIZE.width - SIZE;
        self.top_left = PointF64 {
            x: (self.top_left.x + distance(self.velocity.dx, dt)).clamp(0.0, MAX_LEFT),
            y: (self.top_left.y + distance(self.velocity.dy, dt)).clamp(0.0, MAX_TOP),
        }
    }

    pub fn render(&self, context: &CanvasRenderingContext2d) {
        context.fill_rect(
            (CANVAS_SCALE * self.top_left.x).round(),
            (CANVAS_SCALE * self.top_left.y).round(),
            (CANVAS_SCALE * SIZE).round(),
            (CANVAS_SCALE * SIZE).round(),
        );
    }
}
