/// Supports periodicity; e.g., updating a canvas every Nth animation frame.
pub struct Throttle {
    counter: u32,
    period: u32,
}

impl Throttle {
    /// # Panics
    ///
    /// Will panic if the specified `period` is zero.
    #[must_use]
    pub fn new(period: u32) -> Throttle {
        assert_ne!(period, 0);
        Throttle { counter: 0, period }
    }

    /// Increments this object's internal counter, and returns true unless its
    /// new value is evenly divisble by the supplied period.
    pub fn skip(&mut self) -> bool {
        let counter = self.counter;
        self.counter += 1;
        counter % self.period != 0
    }
}
