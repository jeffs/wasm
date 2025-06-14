use std::num::NonZeroU32;

/// Supports periodicity; e.g., updating a canvas every Nth animation frame.
pub struct Throttle {
    counter: u32,
    period: NonZeroU32,
}

impl Throttle {
    /// # Panics
    ///
    /// Will panic if the specified `period` is zero.
    #[must_use]
    pub fn new(period: NonZeroU32) -> Throttle {
        Throttle { counter: 0, period }
    }

    #[must_use]
    pub fn period(&self) -> NonZeroU32 {
        self.period
    }

    pub fn set_period(&mut self, period: NonZeroU32) {
        self.period = period;
    }

    /// Increments this object's internal counter, and returns true unless its
    /// new value is evenly divisble by the supplied period.
    pub fn skip(&mut self) -> bool {
        let counter = self.counter;
        self.counter += 1;
        counter % self.period != 0
    }
}

impl Default for Throttle {
    /// Constructs a throttle having a period of one; i.e., unthrottled.
    fn default() -> Self {
        let period = NonZeroU32::new(1).unwrap_or_else(|| unreachable!());
        Throttle::new(period)
    }
}
