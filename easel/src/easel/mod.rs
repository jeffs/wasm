use core::cell::RefCell;
use std::rc::Rc;

use perf::components::Fps;
use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, Document, Element, HtmlCanvasElement, Window};

use magic::prelude::*;

use crate::{Error, Result, System};

/// Increase this number to slow the animation. The canvas updates on every Nth
/// frame; so, at 60fps, a throttle of 60 updates about once per second.
pub const THROTTLE: u32 = 1;

fn new_canvas(document: &Document) -> Result<HtmlCanvasElement> {
    Ok(("canvas", "primes-chart__canvas")
        .into_component(document)
        .and_then(Element::dyn_cast::<HtmlCanvasElement>)?)
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

pub struct Easel {
    root: Element,
    system: Rc<System>,
    /// Callback function for [`Window::request_animation_frame`]. So much
    /// indirection is required because the function is self-referential: It
    /// must schedule future callbacks to itself from each animation frame.
    #[expect(clippy::type_complexity)]
    animate: Rc<RefCell<Option<Closure<dyn FnMut()>>>>,
}

impl Easel {
    /// # Errors
    ///
    /// Will return [`Err`] if DOM interaction fails.
    ///
    /// # TODO
    ///
    /// For more complex pages, animation could start and stop as the component
    /// is added or removed from the DOM, as detected by [Mutation Observers](
    /// https://developer.chrome.com/blog/detect-dom-changes-with-mutation-observers/
    /// ).
    pub fn new<F: FnMut(&CanvasRenderingContext2d) + 'static>(
        system: Rc<System>,
        mut render: F,
    ) -> Result<Self> {
        let document = &system.document;

        let canvas = new_canvas(document)?;
        let context = get_context(&canvas)?;

        // Tracks and displays the number of frames per second animated.
        let mut fps = Fps::new(&system.window, document)?;
        let caption = document.caption([], ())?;
        let status = document.div(["primes-chart__status"], (&caption, fps.root()))?;

        let root = document.div([], (canvas.as_ref(), &status))?;

        let mut throttle = perf::Throttle::new(THROTTLE);

        let animate = Rc::new(RefCell::new(None));
        let raf_cb = Rc::clone(&animate);
        let raf_system = Rc::clone(&system);
        *animate.borrow_mut() = Some(Closure::<dyn FnMut()>::new(move || {
            if let Some(cb) = raf_cb.borrow().as_ref() {
                request_animation_frame(&raf_system.window, cb);
            }
            fps.tick();
            if throttle.skip() {
                return;
            }
            render(&context);
        }));

        Ok(Easel {
            root,
            system,
            animate,
        })
    }

    pub fn play(&self) {
        if let Some(cb) = self.animate.borrow().as_ref() {
            request_animation_frame(&self.system.window, cb);
        }
    }

    #[must_use]
    pub fn root(&self) -> &Element {
        &self.root
    }
}
