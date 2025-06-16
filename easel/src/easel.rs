//! # TODO
//!
//! * Factor out a single object, held by a single `Rc`, and obviate all of the
//!   lower level `Rc` pointers.
//! * Once that's done, factor out the play/pause functionality so we
//!   can programmatically start the animation without having to call a
//!   `JsFunction`.

use std::cell::RefCell;
use std::rc::Weak;
use std::{num::NonZeroU32, rc::Rc};

use perf::components::Fps;
use system::Size;
use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, Document, Element, HtmlCanvasElement, Window};

use magic::prelude::*;

use crate::{Error, Result, pause};
use system::System;

struct Captive {
    system: System,
    throttle: perf::Throttle,
    raf_handle: Option<i32>,
    /// Argument for `request_animation_frame`.  [`Option`] because the closure
    /// must capture this shared (captive) state, but is referenced by that
    /// state; so, to break that cycle, we create the state with the option set
    /// to `None`, then mutate it.
    ///
    /// Kudos to the wasm-bindgen guide for the Rc/RefCell/Option workaround:
    /// <https://rustwasm.github.io/wasm-bindgen/examples/request-animation-frame.html>
    raf_callback: Option<Closure<dyn FnMut()>>,
    fps: Fps,
}

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
pub fn canvas_size(canvas: impl AsRef<HtmlCanvasElement>) -> Size {
    Size {
        height: canvas.as_ref().height(),
        width: canvas.as_ref().width(),
    }
}

fn request_animation_frame(window: &Window, f: &Closure<dyn FnMut()>) -> i32 {
    window
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("requesting animation frame")
}

/// Argument to the easel render callback function.
pub struct RenderContext<'a> {
    pub canvas: &'a CanvasRenderingContext2d,
    pub caption: &'a Element,
    pub throttle: NonZeroU32,
}

fn new_pause_button(cell: &Rc<RefCell<Captive>>) -> Result<pause::Button> {
    let weak = Rc::downgrade(cell);
    let handle_pause = move |state: pause::State| {
        let Some(strong) = weak.upgrade() else {
            return;
        };
        let mut captive = strong.borrow_mut();
        captive.raf_handle = match state {
            // On pause: If there's an active frame, cancel it. Iff the
            // cancellation succeeds, set the frame ID to None.
            pause::State::Pause => captive.raf_handle.and_then(|id| {
                // TODO: Cancel animation frame in Drop as well.
                captive
                    .system
                    .window
                    .cancel_animation_frame(id)
                    .err()
                    .map(|_| id)
            }),
            pause::State::Play => captive
                .raf_callback
                .as_ref()
                .map(|callback| request_animation_frame(&captive.system.window, callback)),
        };
    };
    pause::Button::new(&cell.borrow().system.document, handle_pause)
}

/// Well, I'll be a reference-counted cell of an option! We need to hand the
/// `animate` function to `request_animation_frame` so it can be called back.
/// But it also needs a reference to itself, so that on the first call, it can
/// schedule the next, and so on. Thus the complicated type.
///
/// Kudos to the wasm-bindgen guide for the Rc/RefCell/Option workaround:
/// <https://rustwasm.github.io/wasm-bindgen/examples/request-animation-frame.html>
fn new_raf_callback(
    cell: Weak<RefCell<Captive>>,
    mut render: impl FnMut(NonZeroU32) + 'static,
) -> Closure<dyn FnMut()> {
    Closure::<dyn FnMut()>::new(move || {
        let Some(strong) = cell.upgrade() else {
            return;
        };
        let mut captive = strong.borrow_mut();
        if let Some(cb) = captive.raf_callback.as_ref() {
            captive.raf_handle = Some(request_animation_frame(&captive.system.window, cb));
            captive.fps.tick();
            if !captive.throttle.skip() {
                render(captive.throttle.period());
            }
        }
    })
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
    cell: Rc<RefCell<Captive>>,
    pause: pause::Button,
}

impl Easel {
    /// # Errors
    ///
    /// Will return [`Err`] if DOM interaction fails.
    pub fn new<F: FnMut(RenderContext) + 'static>(system: System, mut render: F) -> Result<Self> {
        let document = &system.document.clone();

        let canvas = new_canvas(document)?;
        let context = get_context(&canvas)?;

        let cell = Rc::new(RefCell::new(Captive {
            throttle: perf::Throttle::default(),
            raf_handle: None,
            raf_callback: None,
            fps: Fps::new(&system.window, document)?,
            system,
        }));

        // /// Callback function supplied to the [`Easel`] constructor.
        // render: Box<dyn FnMut(RenderContext) + 'static>,

        let pause = new_pause_button(&cell)?;
        let controls = document.div(["easel-controls"], (pause.root(),))?;

        let caption = document.caption([], ())?;
        let status = document.div(["easel-status"], (&caption, cell.borrow().fps.root()))?;

        let root = document.div([], (canvas.as_ref(), &controls, &status))?;

        let raf_render = move |throttle: NonZeroU32| {
            render(RenderContext {
                canvas: &context,
                caption: &caption,
                throttle,
            });
        };

        let weak = Rc::downgrade(&cell);
        cell.borrow_mut().raf_callback = Some(new_raf_callback(weak, raf_render));

        Ok(Easel { root, cell, pause })
    }

    /// Increase this number to slow the animation. The canvas updates on every Nth
    /// frame; so, at 60fps, a throttle of 60 updates about once per second.
    pub fn throttle(&mut self, period: NonZeroU32) {
        self.cell.borrow_mut().throttle.set_period(period);
    }

    pub fn play(&mut self) {
        self.pause.click();
    }

    /// Creates a new [`Easel`] and immediately begins playing its animation.
    ///
    /// # Errors
    ///
    /// Will return [`Err`] if DOM interaction fails.
    pub fn start<F: FnMut(RenderContext) + 'static>(system: System, render: F) -> Result<Self> {
        let mut easel = Easel::new(system, render)?;
        easel.play();
        Ok(easel)
    }

    #[must_use]
    pub fn root(&self) -> &Element {
        &self.root
    }
}
