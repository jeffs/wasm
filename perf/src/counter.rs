use web_sys::{Performance, Window};

pub struct Counter {
    clock: Performance,
    /// Milliseconds.
    start: f64,
    ticks: u32,
    lap_ticks: u32,
}

impl Counter {
    /// Returns a new performance counter, initialized to the current time, or
    /// [`None`] if the [Performance API][1] is unavailable.
    ///
    /// [1]: https://developer.mozilla.org/en-US/docs/Web/API/Performance
    #[must_use]
    pub fn try_start(window: &Window, lap_ticks: u32) -> Option<Counter> {
        window.performance().map(|clock| Counter {
            start: clock.now(),
            clock,
            ticks: 0,
            lap_ticks,
        })
    }

    /// Increments this object's internal ticker counter.  If the resulting
    /// number of ticks is divisible by the supplied `lap_ticks`, resets the
    /// tick count to zero, and returns the average number of ticks per second
    /// over the past lap; otherwise, returns [`None`].
    pub fn tick(&mut self) -> Option<f64> {
        self.ticks += 1;
        if self.ticks % self.lap_ticks != 0 {
            return None;
        }
        let now = self.clock.now();
        let average = f64::from(self.ticks * 1000) / (now - self.start);
        self.ticks = 0;
        self.start = now;
        Some(average)
    }
}
