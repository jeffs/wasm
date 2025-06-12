use web_sys::CanvasRenderingContext2d;

use super::fill::FillStyle;

pub const CANVAS_WIDTH: u32 = 800;
pub const CANVAS_HEIGHT: u32 = 320;

const FILL_STYLE: FillStyle = FillStyle::Auto;
const GAP: f64 = 2.0;

const fn u32_to_usize(value: u32) -> usize {
    const { assert!(size_of::<u32>() <= size_of::<usize>()) }
    value as usize
}

/// Replaces `powers` with the exponents of all prime factors in `n`, including
/// zeroes, up to the maximum prime factor.
fn prime_factor(powers: &mut Vec<u32>, mut n: u32) {
    powers.clear();
    if n < 2 {
        return;
    }
    for p in rk_primes::Sieve::new().primes() {
        let mut e = 0;
        while n % p == 0 {
            n /= p;
            e += 1;
        }
        powers.push(e);
        if n == 1 {
            return;
        }
    }
    unreachable!()
}

struct Rectangle {
    x: f64,
    y: f64,
    w: f64,
    h: f64,
}

struct Rectangles<'a> {
    powers: &'a [u32],
    height: f64,
    column: u32,
}

impl Rectangles<'_> {
    fn new(powers: &[u32]) -> Rectangles {
        let max_power = powers.iter().copied().max().unwrap_or(1);
        Rectangles {
            powers,
            height: (f64::from(CANVAS_HEIGHT * 1000 / max_power) / 1000.0).round(),
            column: 0,
        }
    }
}

impl Iterator for Rectangles<'_> {
    type Item = Vec<Rectangle>;

    fn next(&mut self) -> Option<Self::Item> {
        let length = self.powers.len();
        let index = u32_to_usize(self.column);
        if index == length {
            return None;
        }

        let length = u32::try_from(length).unwrap();

        let w = f64::from(CANVAS_WIDTH * 1000 / length) / 1000.0;
        let x = f64::from(self.column) * w;
        let rects = (0..self.powers[index])
            .map(|p| Rectangle {
                x: (x + GAP).round(),
                y: f64::from(p) * self.height + GAP,
                w: (w - GAP * 2.0).max(0.0).round(),
                h: (self.height - GAP * 2.0).max(0.0),
            })
            .collect();

        self.column += 1;

        Some(rects)
    }
}

#[derive(Default)]
pub struct Histogram {
    powers: Vec<u32>,
    value: u32,
}

impl Histogram {
    pub fn with_value(value: u32) -> Self {
        let mut powers = Vec::new();
        prime_factor(&mut powers, value);
        Histogram { powers, value }
    }

    pub fn value(&self) -> u32 {
        self.value
    }

    pub fn incr(&mut self) -> &[u32] {
        self.value += 1;
        prime_factor(&mut self.powers, self.value);
        &self.powers
    }

    pub fn clear(&self, context: &CanvasRenderingContext2d) {
        context.begin_path();
        for rect in Rectangles::new(&self.powers).flatten() {
            context.clear_rect(rect.x, rect.y, rect.w, rect.h);
        }
        context.stroke();
    }

    pub fn fill(&self, context: &CanvasRenderingContext2d) {
        context.begin_path();
        for (index, rects) in Rectangles::new(&self.powers).enumerate() {
            context.set_fill_style_str(FILL_STYLE.get(index).as_str());
            for rect in rects {
                context.fill_rect(rect.x, rect.y, rect.w, rect.h);
            }
        }
        context.stroke();
    }
}
