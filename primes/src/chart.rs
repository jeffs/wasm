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

const CELL_WIDTH: u32 = 8;
const CELL_HEIGHT: u32 = 32;
const FILL_STYLE: &str = "purple";

const CANVAS_WIDTH: u32 = 800;
const CANVAS_HEIGHT: u32 = 320;

/// Increase this number to slow the animation.
const THROTTLE: u32 = 20;

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
    context.set_fill_style_str(FILL_STYLE);
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
    column: u32,
}

impl Rectangles<'_> {
    fn new(powers: &[u32]) -> Rectangles {
        Rectangles { powers, column: 0 }
    }
}

impl Iterator for Rectangles<'_> {
    type Item = Rectangle;

    fn next(&mut self) -> Option<Self::Item> {
        let column = u32_to_usize(self.column);
        if column == self.powers.len() {
            return None;
        }
        let x = CELL_WIDTH * self.column;
        let h = CELL_HEIGHT * self.powers[column];
        self.column += 1;
        Some(Rectangle {
            x: x.into(),
            y: 0.0,
            w: CELL_WIDTH.into(),
            h: h.into(),
        })
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
        for rect in Rectangles::new(&self.powers) {
            context.clear_rect(rect.x, rect.y, rect.w, rect.h);
        }
        context.stroke();
    }

    fn fill(&self, context: &CanvasRenderingContext2d) {
        context.begin_path();
        for rect in Rectangles::new(&self.powers) {
            context.fill_rect(rect.x, rect.y, rect.w, rect.h);
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
