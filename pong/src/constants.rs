//! Details used for rendering.

use system::{SizeU32, f64_to_u32_saturating};

use crate::physics::VIRTUAL_SIZE;

pub const CANVAS_SCALE: f64 = 3.0;

pub const CANVAS_SIZE: SizeU32 = SizeU32 {
    width: f64_to_u32_saturating(VIRTUAL_SIZE.width * CANVAS_SCALE),
    height: f64_to_u32_saturating(VIRTUAL_SIZE.height * CANVAS_SCALE),
};
