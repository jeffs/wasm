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
//! * [] Disable transparency of the canvas
//! * [] Try WebGPU
//!   - <https://demyanov.dev/past-and-future-html-canvas-brief-overview-2d-webgl-and-webgpu>
//!   - <https://developer.mozilla.org/en-US/docs/Web/API/WebGPU_API>

use std::{cell::RefCell, rc::Rc};

use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, Document, Element, HtmlCanvasElement, Window};

use crate::js::prelude::*;
use crate::{Error, Result, System};

const CELL_WIDTH: u32 = 8;
const CELL_HEIGHT: u32 = 64;
const FILL_STYLE: &str = "purple";

const CANVAS_WIDTH: u32 = 800;
const CANVAS_HEIGHT: u32 = 320;

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
    canvas
        .get_context_with_context_options("2d", &[("alpha", false)].into_js()?)?
        .ok_or(Error::Str("canvas should have 2d context"))?
        .dyn_cast::<CanvasRenderingContext2d>()
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

/// Returns the `x` and `h` parameters Canvas `clear_rect` and `fill_rect`.  I
/// know that's weird, but it's what varies from one histogram bar to the next.
fn rect(i: u32, j: usize) -> (f64, f64) {
    let x = CELL_WIDTH * u32::try_from(j).unwrap();
    let h = CELL_HEIGHT * i;
    (x.into(), h.into())
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

    #[allow(dead_code)]
    fn incr(&mut self) -> &[u32] {
        self.value += 1;
        self.powers = prime_factor(self.value);
        &self.powers
    }

    fn clear(&self, context: &CanvasRenderingContext2d) {
        context.begin_path();
        for (j, &i) in self.powers.iter().enumerate() {
            let (x, h) = rect(i, j);
            context.clear_rect(x, 0.0, CELL_WIDTH.into(), h);
        }
        context.stroke();
    }

    fn fill(&self, context: &CanvasRenderingContext2d) {
        context.begin_path();
        context.set_fill_style_str(FILL_STYLE);
        for (j, &i) in self.powers.iter().enumerate() {
            let (x, h) = rect(i, j);
            context.fill_rect(x, 0.0, CELL_WIDTH.into(), h);
        }
        context.stroke();
    }
}

fn request_animation_frame(window: &Window, f: &Closure<dyn FnMut()>) {
    window
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("requesting animation frame");
}

pub struct Chart {
    pub root: Element,
}

impl Chart {
    pub fn new(system: &Rc<System>) -> Result<Self> {
        let title = system.document.create_element("h1")?;
        title.set_class_name("chart__title");
        title.set_text_content(Some("Prime factors of 1"));

        let label_x = system.document.create_element("pre")?;
        label_x.set_class_name("chart__label_x");
        // label_x.set_text_content(Some("02 03 05 07 11 13 17 19 23 29"));

        let canvas = new_canvas(&system.document)?;
        let context = get_context(&canvas)?;

        let root = system.document.create_element("div")?;
        root.set_class_name("life");
        root.append_with_node_3(&title, &canvas, &label_x)?;

        let render = Rc::new(RefCell::new(None));
        let raf_cb = Rc::clone(&render);

        #[allow(unused_variables)]
        let mut counter = 0;
        let mut histogram = Histogram::with_value(0);
        let raf_system = Rc::clone(system);
        *raf_cb.borrow_mut() = Some(Closure::<dyn FnMut()>::new(move || {
            histogram.clear(&context);
            // if counter % 10 == 0 {
            histogram.incr();
            // }
            let value = histogram.value;
            title.set_text_content(Some(&format!("Prime factors of {value}")));
            histogram.fill(&context);
            request_animation_frame(&raf_system.window, render.borrow().as_ref().unwrap());
            counter += 1;
        }));

        request_animation_frame(&system.window, raf_cb.borrow().as_ref().unwrap());

        Ok(Chart { root })
    }
}
