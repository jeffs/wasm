pub struct LinearCongruentialGenerator(u32);

impl LinearCongruentialGenerator {
    #[must_use]
    pub fn new() -> Self {
        LinearCongruentialGenerator(0)
    }

    #[must_use]
    pub fn from_seed(seed: u32) -> Self {
        LinearCongruentialGenerator(seed)
    }

    // Replaces the current seed ith a new value according to the infamous "ANSI
    // C" constants; see also [`rand(3)`]. Returns the new value. The constants
    // used are apparently [not great].
    //
    // [`rand(3)`]: https://pubs.opengroup.org/onlinepubs/009695399/functions/rand.html
    // [not great]: https://stackoverflow.com/a/8574774/3116635
    pub fn next_u32(&mut self) -> u32 {
        self.0 = self.0.wrapping_mul(1_103_515_245).wrapping_add(12345);
        self.0
    }

    pub fn next_bool(&mut self) -> bool {
        self.next_u32() % 2 != 0
    }

    #[expect(clippy::cast_possible_wrap)]
    pub fn next_i32(&mut self) -> i32 {
        self.next_u32() as i32
    }
}

impl Default for LinearCongruentialGenerator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore = "slow"]
    #[test]
    fn period() {
        // The Rust standard library's `count` and `len` functions all return
        // `usize`, which is only 32 bits on WASM32. So, we loop over a `u64`.
        let mut period = 0u64;
        let mut lcg = LinearCongruentialGenerator::from_seed(0);
        while {
            period += 1;
            lcg.next_u32() != 0
        } {}
        assert_eq!(period, 1u64 << 32);
    }
}
