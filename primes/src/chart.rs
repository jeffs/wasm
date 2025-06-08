//! # Thanks
//!
//! * <https://rustwasm.github.io/wasm-bindgen/examples/2d-canvas.html>
//! * <https://rustwasm.github.io/wasm-bindgen/examples/request-animation-frame.html>
//! * <https://rustwasm.github.io/docs/book/game-of-life/implementing.html>
//! * <https://developer.mozilla.org/en-US/docs/Web/API/Canvas_API/Tutorial/Optimizing_canvas>
//!
//! # TODO
//!
//! * [] Add an FPS counter
//! * [] Try WebGPU
//!   - <https://demyanov.dev/past-and-future-html-canvas-brief-overview-2d-webgl-and-webgpu>
//!   - <https://developer.mozilla.org/en-US/docs/Web/API/WebGPU_API>

use std::{cell::RefCell, rc::Rc};

use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, Document, Element, HtmlCanvasElement, Window};

use crate::js::prelude::*;
use crate::{Error, Result, System};

const CANVAS_WIDTH: u32 = 800;
const CANVAS_HEIGHT: u32 = 320;

const GAP: f64 = 2.0;

/// Increase this number to slow the animation.
const THROTTLE: u32 = 1;

const COLORS: [&str; 14] = [
    "#FF0000", //  2,  47 Red
    "#00FF00", //  3,  53 Lime
    "#0000FF", //  5,  59 Blue
    "#FFFF00", //  7,  61 Yellow
    "#00FFFF", // 11,  67 Cyan
    "#FF00FF", // 13,  71 Magenta
    "#C0C0C0", // 17,  73 Silver
    "#808080", // 19,  79 Gray
    "#800000", // 23,  83 Maroon
    "#808000", // 29,  89 Olive
    "#008000", // 31,  97 Green
    "#800080", // 37, 101 Purple
    "#008080", // 41, 103 Teal
    "#000080", // 43, 107 Navy
];

const fn u32_to_usize(value: u32) -> usize {
    const { assert!(size_of::<u32>() <= size_of::<usize>()) }
    value as usize
}

fn new_canvas(document: &Document) -> Result<HtmlCanvasElement> {
    let canvas = document
        .create_element("canvas")?
        .dyn_cast::<HtmlCanvasElement>()?;
    canvas.set_class_name("chart__canvas");
    canvas.set_width(CANVAS_WIDTH);
    canvas.set_height(CANVAS_HEIGHT);
    Ok(canvas)
}

fn get_context(canvas: &HtmlCanvasElement) -> Result<CanvasRenderingContext2d> {
    let context = canvas
        .get_context_with_context_options("2d", &[("alpha", false)].into_js()?)?
        .ok_or(Error::Str("canvas should have 2d context"))?
        .dyn_cast::<CanvasRenderingContext2d>()?;
    Ok(context)
}

fn prime_factor(mut n: u32) -> Vec<u32> {
    let mut powers = Vec::new();
    if n < 2 {
        return powers;
    }
    for p in rk_primes::Sieve::default().primes() {
        let mut e = 0;
        while n % p == 0 {
            n /= p;
            e += 1;
        }
        powers.push(e);
        if n == 1 {
            return powers;
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
struct Histogram {
    powers: Vec<u32>,
    value: u32,
}

impl Histogram {
    fn with_value(value: u32) -> Self {
        Histogram {
            powers: prime_factor(value),
            value,
        }
    }

    fn incr(&mut self) -> &[u32] {
        self.value += 1;
        self.powers = prime_factor(self.value);
        &self.powers
    }

    fn clear(&self, context: &CanvasRenderingContext2d) {
        context.begin_path();
        for rect in Rectangles::new(&self.powers).flatten() {
            context.clear_rect(rect.x, rect.y, rect.w, rect.h);
        }
        context.stroke();
    }

    fn fill(&self, context: &CanvasRenderingContext2d) {
        context.begin_path();
        for (index, rects) in Rectangles::new(&self.powers).enumerate() {
            if THROTTLE > 1 {
                context.set_fill_style_str(COLORS[index % COLORS.len()]);
            } else {
                // Use grayscale, because flashing colors are jarring.
                let color = format!("#{i:02x}{i:02x}{i:02x}", i = 15 + index % 16 * 14);
                context.set_fill_style_str(&color);
            }
            for rect in rects {
                context.fill_rect(rect.x, rect.y, rect.w, rect.h);
            }
        }
        context.stroke();
    }
}

fn request_animation_frame(window: &Window, f: &Closure<dyn FnMut()>) {
    window
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("requesting animation frame");
}

struct Throttle {
    counter: u32,
    period: u32,
}

impl Throttle {
    fn new(period: u32) -> Throttle {
        assert_ne!(period, 0);
        Throttle { counter: 0, period }
    }

    fn skip(&mut self) -> bool {
        let counter = self.counter;
        self.counter += 1;
        counter % self.period != 0
    }
}

pub struct Chart {
    pub root: Element,
}

impl Chart {
    pub fn new(system: &Rc<System>) -> Result<Self> {
        let title = system.document.create_element("h1")?;
        title.set_class_name("chart__title");
        title.set_text_content(Some("Prime factors of 1"));

        let canvas = new_canvas(&system.document)?;
        let context = get_context(&canvas)?;

        let root = system.document.create_element("div")?;
        root.set_class_name("chart");
        root.append_with_node_2(&title, &canvas)?;

        let render = Rc::new(RefCell::new(None));
        let raf_cb = Rc::clone(&render);

        let mut throttle = Throttle::new(THROTTLE);
        let mut histogram = Histogram::with_value(0);
        let raf_system = Rc::clone(system);
        *raf_cb.borrow_mut() = Some(Closure::<dyn FnMut()>::new(move || {
            request_animation_frame(&raf_system.window, render.borrow().as_ref().unwrap());
            if throttle.skip() {
                return;
            }
            histogram.clear(&context);
            histogram.incr();
            let value = histogram.value;
            title.set_text_content(Some(&format!("Prime factors of {value}")));
            histogram.fill(&context);
        }));

        request_animation_frame(&system.window, raf_cb.borrow().as_ref().unwrap());

        Ok(Chart { root })
    }
}
