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

const CELL_SIZE: u32 = 12;

#[derive(Copy, Clone)]
struct RectangleSize {
    width: u32,
    height: u32,
}

fn new_canvas(document: &Document, size: RectangleSize) -> Result<HtmlCanvasElement> {
    let canvas = document
        .create_element("canvas")?
        .dyn_cast::<HtmlCanvasElement>()?;
    canvas.set_class_name("chart__canvas");
    canvas.set_width(CELL_SIZE * size.width);
    canvas.set_height(CELL_SIZE * size.height);
    Ok(canvas)
}

fn get_context(canvas: &HtmlCanvasElement) -> Result<CanvasRenderingContext2d> {
    canvas
        .get_context_with_context_options("2d", &[("alpha", false)].into_js()?)?
        .ok_or(Error::Str("canvas should have 2d context"))?
        .dyn_cast::<CanvasRenderingContext2d>()
}

/// TODO: Use prime factoring from Rust Kart. Profile.
fn prime_factor(mut value: u32) -> Vec<u32> {
    let mut powers = Vec::new();
    let mut factor = 2;
    while factor * factor <= value {
        let mut power = 0;
        while value % factor == 0 {
            power += 1;
            value /= factor;
        }
        powers.push(power);
        factor += 1;
    }
    powers
}

#[derive(Default)]
struct Histogram {
    powers: Vec<u32>,
    value: u32,
}

impl Histogram {
    fn incr(&mut self) -> &[u32] {
        self.value += 1;
        self.powers = prime_factor(self.value);
        &self.powers
    }

    fn clear(&self, context: &CanvasRenderingContext2d) {
        context.begin_path();
        for (j, i) in self.powers.iter().enumerate() {
            let x = CELL_SIZE * u32::try_from(j).unwrap();
            let h = CELL_SIZE * i;
            context.clear_rect(x.into(), 0.0, CELL_SIZE.into(), h.into());
        }
        context.stroke();
    }

    fn fill(&self, context: &CanvasRenderingContext2d) {
        context.begin_path();
        context.set_fill_style_str("#EEE");
        for (j, i) in self.powers.iter().enumerate() {
            let x = CELL_SIZE * u32::try_from(j).unwrap();
            let h = CELL_SIZE * i;
            context.fill_rect(x.into(), 0.0, CELL_SIZE.into(), h.into());
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
        title.set_class_name("life__title");
        title.set_text_content(Some("Prime factors of 1"));

        let size = RectangleSize {
            width: 64,
            height: 20,
        };

        let canvas = new_canvas(&system.document, size)?;
        let context = get_context(&canvas)?;

        let root = system.document.create_element("div")?;
        root.set_class_name("life");
        root.append_with_node_2(&title, &canvas)?;

        let render = Rc::new(RefCell::new(None));
        let raf_cb = Rc::clone(&render);

        let mut histogram = Histogram::default();
        let raf_system = Rc::clone(system);
        *raf_cb.borrow_mut() = Some(Closure::<dyn FnMut()>::new(move || {
            histogram.clear(&context);
            histogram.incr();
            let value = histogram.value;
            title.set_text_content(Some(&format!("Prime factors of {value}")));
            histogram.fill(&context);
            request_animation_frame(&raf_system.window, render.borrow().as_ref().unwrap());
        }));

        request_animation_frame(&system.window, raf_cb.borrow().as_ref().unwrap());

        Ok(Chart { root })
    }
}
