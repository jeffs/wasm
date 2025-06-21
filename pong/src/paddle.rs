use system::SizeF64;
use web_sys::CanvasRenderingContext2d;

use crate::{
    constants::CANVAS_SCALE,
    physics::{Direction, PointF64, VIRTUAL_SIZE, distance},
};

pub const SIZE: SizeF64 = SizeF64 {
    width: 5.0,
    height: 20.0,
};

/// Virtual units per second.
pub const SPEED: f64 = 200.0;

pub struct Paddle {
    top_left: PointF64,
    direction: Option<Direction>,
}

impl Paddle {
    pub fn new(top_left: PointF64) -> Self {
        Paddle {
            top_left,
            direction: None,
        }
    }

    pub fn update(&mut self, dt: f64) {
        const MAX_TOP: f64 = VIRTUAL_SIZE.height - SIZE.height;
        if let Some(direction) = self.direction {
            let top = self.top_left.y;
            self.top_left.y = match direction {
                Direction::Up => (top - distance(SPEED, dt)).max(0.0),
                Direction::Down => (top + distance(SPEED, dt)).min(MAX_TOP),
            };
        }
    }

    pub fn render(&self, context: &CanvasRenderingContext2d) {
        context.fill_rect(
            (CANVAS_SCALE * self.top_left.x).round(),
            (CANVAS_SCALE * self.top_left.y).round(),
            (CANVAS_SCALE * SIZE.width).round(),
            (CANVAS_SCALE * SIZE.height).round(),
        );
    }

    pub fn set_direction(&mut self, direction: Option<Direction>) {
        self.direction = direction;
    }
}
