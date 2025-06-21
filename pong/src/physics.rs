//! All coordinates here are virtual. Our virtual space is defined in floating
//! point, to support movement of less than one virtual pixel per tick. All
//! speeds are virtual units per second (even though time is often measured
//! in milliseconds).

use system::SizeF64;

#[derive(Clone)]
pub struct PointF64 {
    pub x: f64,
    pub y: f64,
}

#[derive(Default)]
pub struct Vec2d {
    pub dx: f64,
    pub dy: f64,
}

#[derive(Clone, Copy)]
pub enum Direction {
    Up,
    Down,
}

/// Roughly 16:9.
pub const VIRTUAL_SIZE: SizeF64 = SizeF64 {
    width: 426.0,
    height: 240.0,
};

/// Converts speed and elapsed time to distance. Speed should be virual units
/// per second, and time should be in milliseconds.
pub fn distance(speed: f64, dt: f64) -> f64 {
    speed * dt / 1000.0 // One thousand milliseconds per second.
}
