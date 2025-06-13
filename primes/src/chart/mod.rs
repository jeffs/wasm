//! # Thanks
//!
//! * <https://rustwasm.github.io/wasm-bindgen/examples/2d-canvas.html>
//! * <https://rustwasm.github.io/wasm-bindgen/examples/request-animation-frame.html>
//! * <https://rustwasm.github.io/docs/book/game-of-life/implementing.html>
//! * <https://developer.mozilla.org/en-US/docs/Web/API/Canvas_API/Tutorial/Optimizing_canvas>
//!
//! # TODO
//!
//! * [] Add a Play/Pause and Fast Forward buttons to the UI
//! * [] Try WebGPU
//!   - <https://demyanov.dev/past-and-future-html-canvas-brief-overview-2d-webgl-and-webgpu>
//!   - <https://developer.mozilla.org/en-US/docs/Web/API/WebGPU_API>

mod fill;
mod histogram;

use core::cell::RefCell;
use std::rc::Rc;

use fill::FillStyle;
use histogram::Histogram;
use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, Document, Element, HtmlCanvasElement, Window};

use crate::js::prelude::*;
use crate::magic::prelude::*;
use crate::magic::tag::prelude::*;
use crate::{Error, Result, System};

/// Increase this number to slow the animation. The canvas updates on every Nth
/// frame; so, at 60fps, a throttle of 60 updates about once per second.
pub const THROTTLE: u32 = 180;

const FILL_STYLE: FillStyle = FillStyle::Auto { throttle: THROTTLE };

fn new_canvas(document: &Document) -> Result<HtmlCanvasElement> {
    ("canvas", "chart__canvas")
        .into_component(document)
        .and_then(Element::dyn_cast::<HtmlCanvasElement>)
}

fn get_context(canvas: &HtmlCanvasElement) -> Result<CanvasRenderingContext2d> {
    let context = canvas
        .get_context_with_context_options("2d", &[("alpha", false)].into_js()?)?
        .ok_or(Error::Context2d)?
        .dyn_cast::<CanvasRenderingContext2d>()?;
    Ok(context)
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
    /// For more complex pages, animation could start and stop as the component
    /// is added or removed from the DOM, as detected by [Mutation Observers](
    /// https://developer.chrome.com/blog/detect-dom-changes-with-mutation-observers/
    /// ).
    pub fn new(system: &Rc<System>) -> Result<Self> {
        let document = &system.document;
        let title = document.h1(["chart__title"], "1: []")?;
        let canvas = new_canvas(document)?;
        let context = get_context(&canvas)?;

        let mut fps = perf::components::Fps::new(&system.window, document)?;
        fps.as_ref().set_class_name("chart__caption");

        let root = document.div(["chart"], (&title, &canvas, fps.as_ref()))?;

        let mut throttle = perf::Throttle::new(THROTTLE);
        let mut histogram = Histogram::new();

        let mut factors = Vec::new();
        let mut sieve = rk_primes::Sieve::new();

        let render = Rc::new(RefCell::new(None));
        let raf_cb = Rc::clone(&render);
        let raf_system = Rc::clone(system);

        *raf_cb.borrow_mut() = Some(Closure::<dyn FnMut()>::new(move || {
            request_animation_frame(&raf_system.window, render.borrow().as_ref().unwrap());
            fps.tick();

            if throttle.skip() {
                return;
            }

            histogram.clear(&context);
            histogram.incr(&mut sieve);
            histogram.fill(&context, FILL_STYLE);

            let value = histogram.value();
            factors.clear();
            factors.extend(sieve.factors(value));
            title.set_text_content(Some(&format!("{value}: {factors:?}")));
        }));

        request_animation_frame(&system.window, raf_cb.borrow().as_ref().unwrap());

        Ok(Chart { root })
    }
}
