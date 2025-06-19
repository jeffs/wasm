use wasm_bindgen::prelude::*;
use web_sys::{Element, Performance};

use sugar::prelude::*;
use system::System;

/// The number of frames over which to average.
const LEN: usize = 60;
const LEN_F64: f64 = 60.0;

#[derive(Debug)]
pub enum FpsError {
    /// A DOM element could not be created.
    Creation(JsValue),
    /// The Performance API is unavailable.
    Performance,
}

/// Tracks and displays Frames Per Second (FPS).
pub struct Fps {
    root: Element,
    clock: Performance,
    /// The previous reading of `clock.now()`, or zero.
    last: f64,
    /// The last N time deltas, or zeroes.
    deltas: [f64; LEN],
    /// The total number of ticks received so far.
    count: usize,
    /// The sum of `deltas`.
    sum: f64,
}

impl Fps {
    /// Creates (but does not mount) an element in the specified document,
    /// using the Performance API of the specified window. Starts a performance
    /// counter immediately upon construction.
    ///
    /// # Errors
    ///
    /// Will return [`Err`] if the element cannot be created.
    pub fn new(system: &System) -> Result<Self, FpsError> {
        let clock = system.window.performance().ok_or(FpsError::Performance)?;
        let last = clock.now();
        Ok(Fps {
            root: SPAN
                .class("perf-fps")
                .html("&nbsp;") // Rescue paragraph from having zero height.
                .to_element(system)
                .map_err(FpsError::Creation)?,
            clock,
            last,
            deltas: [0.0; LEN],
            count: 0,
            sum: 0.0,
        })
    }

    pub fn tick(&mut self) {
        let now = self.clock.now();
        if self.last != 0.0 {
            let delta = now - self.last;
            let index = self.count % LEN;
            self.sum -= self.deltas[index];
            self.sum += delta;
            self.deltas[index] = delta;
            if self.sum > 1000.0 && self.count >= LEN {
                let fps = LEN_F64 * 1000.0 / self.sum;
                self.root.set_text_content(Some(&format!("{fps:.01} fps")));
            }
        }
        self.count += 1;
        self.last = now;
    }

    #[must_use]
    pub fn root(&self) -> &Element {
        &self.root
    }
}
