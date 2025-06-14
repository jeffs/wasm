use easel::canvas_size;
use rk_primes::Sieve;
use system::{Size, usize_to_u32};
use web_sys::CanvasRenderingContext2d;

use crate::fill::FillStyle;

const GAP: u32 = 2;

/// Replaces `powers` with the exponents of all prime factors in `n`, including
/// zeroes, up to the maximum prime factor.
fn prime_factor(sieve: &mut Sieve, powers: &mut Vec<u32>, mut n: u32) {
    powers.clear();
    for p in sieve.primes() {
        if n < 2 {
            break;
        }
        let mut e = 0;
        while n % p == 0 {
            n /= p;
            e += 1;
        }
        powers.push(e);
    }
}

struct Rectangle {
    x: f64,
    y: f64,
    w: f64,
    h: f64,
}

struct Columns<'a> {
    /// The exponents represented by these columns.
    powers: &'a [u32],
    /// The size of each brick.
    brick: Size,
    /// The index in [`Self::powers`] of the next column to return.
    index: usize,
}

impl Columns<'_> {
    fn new(powers: &[u32], canvas: Size) -> Columns {
        let max_power = powers.iter().copied().max().unwrap_or(1);
        Columns {
            powers,
            brick: Size {
                height: canvas.height.checked_div(max_power).unwrap_or_default(),
                width: canvas
                    .width
                    .checked_div(usize_to_u32(powers.len()))
                    .unwrap_or_default(),
            },
            index: 0,
        }
    }
}

impl Iterator for Columns<'_> {
    type Item = Vec<Rectangle>;

    fn next(&mut self) -> Option<Self::Item> {
        let length = self.powers.len();
        if self.index == length {
            return None;
        }
        let x = usize_to_u32(self.index) * self.brick.width;
        let rects = (0..self.powers[self.index])
            .map(|p| Rectangle {
                x: f64::from(x + GAP),
                y: f64::from(p * self.brick.height + GAP),
                w: f64::from(self.brick.width.saturating_sub(GAP * 2)),
                h: f64::from(self.brick.height.saturating_sub(GAP * 2)),
            })
            .collect();
        self.index += 1;
        Some(rects)
    }
}

pub struct Histogram {
    powers: Vec<u32>,
    value: u32,
}

impl Histogram {
    pub fn new() -> Self {
        Histogram {
            powers: Vec::new(),
            value: 1,
        }
    }

    pub fn value(&self) -> u32 {
        self.value
    }

    pub fn incr(&mut self, sieve: &mut Sieve) -> &[u32] {
        self.value += 1;
        prime_factor(sieve, &mut self.powers, self.value);
        &self.powers
    }

    /// Erases this histogram from the canvas.
    pub fn clear(&self, context: &CanvasRenderingContext2d) {
        let Some(canvas) = context.canvas() else {
            return;
        };
        context.begin_path();
        for brick in Columns::new(&self.powers, canvas_size(&canvas)).flatten() {
            context.clear_rect(brick.x, brick.y, brick.w, brick.h);
        }
        context.stroke();
    }

    /// Draws this histogram to the canvas.
    pub fn fill(&self, context: &CanvasRenderingContext2d, style: FillStyle) {
        let Some(canvas) = context.canvas() else {
            return;
        };
        context.begin_path();
        for (index, column) in Columns::new(&self.powers, canvas_size(&canvas)).enumerate() {
            context.set_fill_style_str(style.get(index).as_str());
            for brick in column {
                context.fill_rect(brick.x, brick.y, brick.w, brick.h);
            }
        }
        context.stroke();
    }
}
