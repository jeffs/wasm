mod easel;
mod error;
mod pause;

pub use easel::{Easel, RenderContext, canvas_size};
pub use error::{Error, Result};
pub use system::System;
use wasm_bindgen::prelude::Closure;

/// Callback function for [`Window::request_animation_frame`].
type RafCallback = Closure<dyn FnMut()>;
