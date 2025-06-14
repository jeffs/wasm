use core::cell::RefCell;
use std::rc::Rc;

use perf::components::Fps;
use system::Size;
use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, Document, Element, HtmlCanvasElement, Window};

use magic::prelude::*;

use crate::{Error, Result, System};

/// Increase this number to slow the animation. The canvas updates on every Nth
/// frame; so, at 60fps, a throttle of 60 updates about once per second.
pub const THROTTLE: u32 = 1;

fn new_canvas(document: &Document) -> Result<HtmlCanvasElement> {
    Ok(("canvas", "easel-canvas")
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

#[must_use]
pub fn canvas_size(canvas: &HtmlCanvasElement) -> Size {
    Size {
        height: canvas.height(),
        width: canvas.width(),
    }
}

fn request_animation_frame(window: &Window, f: &Closure<dyn FnMut()>) {
    window
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("requesting animation frame");
}

/// Argument to the easel render callback function.
pub struct RenderContext<'a> {
    pub canvas: &'a CanvasRenderingContext2d,
    pub caption: &'a Element,
    pub throttle: u32,
}

/// A component that holds a canvas, along with a status bar including a caption
/// and an FPS counter.
///
/// # TODO
///
/// Automatically play and pause animation as the component is added
/// or removed from the DOM, as detected by [Mutation Observers](
/// https://developer.chrome.com/blog/detect-dom-changes-with-mutation-observers
/// ).
pub struct Easel {
    root: Element,
    system: Rc<System>,
    /// Callback for [`Window::request_animation_frame`]. So much indirection
    /// is required because the function is self-referential: Each time it is
    /// called, it must schedule the next call to itself.
    #[expect(clippy::type_complexity)]
    animate: Rc<RefCell<Option<Closure<dyn FnMut()>>>>,
}

impl Easel {
    /// # Errors
    ///
    /// Will return [`Err`] if DOM interaction fails.
    pub fn new<F: FnMut(RenderContext) + 'static>(
        system: Rc<System>,
        mut render: F,
    ) -> Result<Self> {
        let document = &system.document;

        let canvas = new_canvas(document)?;
        let context = get_context(&canvas)?;

        let mut fps = Fps::new(&system.window, document)?;
        let caption = document.caption([], ())?;
        let status = document.div(["easel-status"], (&caption, fps.root()))?;

        let root = document.div([], (canvas.as_ref(), &status))?;

        let mut throttle = perf::Throttle::new(THROTTLE);

        // Well, I'll be a reference-counted cell of an option! We need to
        // hand the `request_animation_frame` callback to the runtime system so
        // it can be, uh, called back. But it also needs a reference to itself,
        // so that on the first call, it can schedule the next, and so on.
        //
        // Kudos to the wasm-bindgen guide for the Rc/RefCell/Option workaround:
        // <https://rustwasm.github.io/wasm-bindgen/examples/request-animation-frame.html>
        let animate = Rc::new(RefCell::new(None));
        let raf_cb = Rc::clone(&animate);
        let raf_system = Rc::clone(&system);
        *animate.borrow_mut() = Some(Closure::<dyn FnMut()>::new(move || {
            if let Some(cb) = raf_cb.borrow().as_ref() {
                request_animation_frame(&raf_system.window, cb);
                fps.tick();
                if throttle.skip() {
                    return;
                }
                render(RenderContext {
                    canvas: &context,
                    caption: &caption,
                    throttle: THROTTLE,
                });
            }
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

    /// Creates a new [`Easel`] and immediately begins playing its animation.
    /// # Errors
    ///
    /// Will return [`Err`] if DOM interaction fails.
    pub fn start<F: FnMut(RenderContext) + 'static>(system: Rc<System>, render: F) -> Result<Self> {
        let easel = Easel::new(system, render)?;
        easel.play();
        Ok(easel)
    }

    #[must_use]
    pub fn root(&self) -> &Element {
        &self.root
    }
}
