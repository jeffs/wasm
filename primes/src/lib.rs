//! # Thanks
//!
//! * <https://rustwasm.github.io/wasm-bindgen/examples/2d-canvas.html>
//! * <https://rustwasm.github.io/wasm-bindgen/examples/request-animation-frame.html>
//! * <https://rustwasm.github.io/docs/book/game-of-life/implementing.html>
//! * <https://developer.mozilla.org/en-US/docs/Web/API/Canvas_API/Tutorial/Optimizing_canvas>
//!
//! # TODO
//!
//! * [] Add settings, such as speed control and color selection, to the UI.
//! * [] Add audio, such as mapping primes to piano keys.
//! * [] Instead of a canvas, animate DOM elements with intuitive transitions.

mod fill;
mod histogram;

use web_sys::Element;

use easel::{Easel, RenderContext, Result};
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
    pub fn new(system: &System) -> Result<Self> {
        let mut sieve = rk_primes::Sieve::new();
        let mut histogram = Histogram::new();
        let mut factors = Vec::new();
        let easel = Easel::new(system, move |easel: RenderContext| {
            // Repaint the canvas.
            histogram.clear(easel.canvas);
            histogram.incr(&mut sieve);
            histogram.fill(easel.canvas, FillStyle::Color);
            // Update the caption.
            let value = histogram.value();
            factors.clear();
            factors.extend(sieve.factors(value));
            let caption = format!("{value}: {factors:?}");
            easel.caption.set_text_content(Some(&caption));
        })?;
        Ok(Chart { easel })
    }

    pub fn play(&mut self) {
        self.easel.play();
    }
}

impl AsRef<Element> for Chart {
    fn as_ref(&self) -> &Element {
        self.easel.as_ref()
    }
}
