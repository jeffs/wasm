//! # Thanks
//!
//! * <https://rustwasm.github.io/wasm-bindgen/examples/2d-canvas.html>
//! * <https://rustwasm.github.io/wasm-bindgen/examples/request-animation-frame.html>
//! * <https://rustwasm.github.io/docs/book/game-of-life/implementing.html>
//! * <https://developer.mozilla.org/en-US/docs/Web/API/Canvas_API/Tutorial/Optimizing_canvas>
//!
//! # TODO
//!
//! * [] Add a Play/Pause and Fast Forward buttons to the UI
//! * [] Try WebGPU
//!   - <https://demyanov.dev/past-and-future-html-canvas-brief-overview-2d-webgl-and-webgpu>
//!   - <https://developer.mozilla.org/en-US/docs/Web/API/WebGPU_API>

mod fill;
mod histogram;

use std::{num::NonZeroU32, rc::Rc};

use easel::{Easel, RenderContext, Result};
use web_sys::Element;

use fill::FillStyle;
use histogram::Histogram;
use system::System;

pub struct Chart {
    easel: Easel,
}

impl Chart {
    /// # Errors
    ///
    /// Will return [`Err`] if DOM interaction fails.
    pub fn new(system: &Rc<System>) -> Result<Self> {
        let mut sieve = rk_primes::Sieve::new();
        let mut histogram = Histogram::new();
        let mut factors = Vec::new();
        let mut easel = Easel::new(system, move |easel: RenderContext| {
            // Repaint the canvas.
            let throttle = easel.throttle.into();
            histogram.clear(easel.canvas);
            histogram.incr(&mut sieve);
            histogram.fill(easel.canvas, FillStyle::Auto { throttle });
            // Update the caption.
            let value = histogram.value();
            factors.clear();
            factors.extend(sieve.factors(value));
            let caption = format!("{value}: {factors:?}");
            easel.caption.set_text_content(Some(&caption));
        })?;
        easel.throttle(NonZeroU32::new(100).unwrap_or_else(|| unreachable!()));
        Ok(Chart { easel })
    }

    pub fn play(&self) {
        self.easel.play();
    }

    #[must_use]
    pub fn root(&self) -> &Element {
        self.easel.root()
    }
}
