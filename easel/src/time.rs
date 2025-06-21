use web_sys::Performance;

use system::System;

pub struct Stopwatch {
    /// The timestamp of the most recent call to [`Self::delta_ms`].
    last_ms: Option<f64>,
    clock: Performance,
}

impl Stopwatch {
    pub fn new(system: &System) -> Option<Self> {
        Some(Stopwatch {
            last_ms: None,
            clock: system.window.performance()?,
        })
    }

    /// Returns [`None`] on the first call, or if the clock is unavailable.
    pub fn delta_ms(&mut self) -> Option<f64> {
        let t1 = self.clock.now();
        self.last_ms.replace(t1).map(|t0| t1 - t0)
    }
}
