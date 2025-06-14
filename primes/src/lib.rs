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

use std::rc::Rc;

use easel::{Easel, Result};
use web_sys::{CanvasRenderingContext2d, Element};

use fill::FillStyle;
use histogram::Histogram;
use system::System;

const FILL_STYLE: FillStyle = FillStyle::Auto;

pub struct Chart {
    easel: Easel,
}

impl Chart {
    /// # Errors
    ///
    /// Will return [`Err`] if DOM interaction fails.
    ///
    /// # TODO
    ///
    /// For more complex pages, animation could start and stop as the component
    /// is added or removed from the DOM, as detected by [Mutation Observers](
    /// https://developer.chrome.com/blog/detect-dom-changes-with-mutation-observers/
    /// ).
    pub fn new(system: Rc<System>) -> Result<Self> {
        let mut sieve = rk_primes::Sieve::new();
        let mut histogram = Histogram::new();
        let mut factors = Vec::new();

        let render = move |context: &CanvasRenderingContext2d, caption: &Element| {
            histogram.clear(context);
            histogram.incr(&mut sieve);
            histogram.fill(context, FILL_STYLE);
            let value = histogram.value();
            factors.clear();
            factors.extend(sieve.factors(value));
            caption.set_text_content(Some(&format!("{value}: {factors:?}")));
        };

        let easel = Easel::new(system, render)?;
        easel.play();

        Ok(Chart { easel })
    }

    #[must_use]
    pub fn root(&self) -> &Element {
        self.easel.root()
    }
}
