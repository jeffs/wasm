//! # TODO
//!
//! * Factor out a single object, held by a single `Rc`, and obviate all of the
//!   lower level `Rc` pointers.
//! * Once that's done, factor out the play/pause functionality so we
//!   can programmatically start the animation without having to call a
//!   `JsFunction`.

use std::cell::{Cell, RefCell};
use std::{num::NonZeroU32, rc::Rc};

use perf::components::Fps;
use system::Size;
use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, Document, Element, HtmlCanvasElement, Window};

use magic::prelude::*;

use crate::{Error, RafCallback, Result, System, pause};

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

fn request_animation_frame(window: &Window, f: &RafCallback) -> i32 {
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

fn new_pause_button(
    frame_id: Rc<Cell<Option<i32>>>,
    animate: Rc<RefCell<Option<RafCallback>>>,
    system: &Rc<System>,
) -> Result<pause::Button> {
    let cb_system = Rc::clone(system);
    let handle_pause = move |state: pause::State| {
        frame_id.set(match state {
            pause::State::Pause => frame_id.get().and_then(|handle| {
                // TODO: Cancel animation frame in Drop as well.
                cb_system
                    .window
                    .cancel_animation_frame(handle)
                    .err()
                    .map(|_| handle)
            }),
            pause::State::Play => animate
                .borrow()
                .as_ref()
                .map(|animate| request_animation_frame(&cb_system.window, animate)),
        });
    };
    pause::Button::new(&system.document, handle_pause)
}

/// Well, I'll be a reference-counted cell of an option! We need to hand the
/// `animate` function to `request_animation_frame` so it can be called back.
/// But it also needs a reference to itself, so that on the first call, it can
/// schedule the next, and so on. Thus the complicated type.
///
/// Kudos to the wasm-bindgen guide for the Rc/RefCell/Option workaround:
/// <https://rustwasm.github.io/wasm-bindgen/examples/request-animation-frame.html>
fn new_raf_callback(
    system: Rc<System>,
    throttle: Rc<RefCell<perf::Throttle>>,
    frame_id: Rc<Cell<Option<i32>>>,
    animate: Rc<RefCell<Option<RafCallback>>>,
    mut fps: Fps,
    mut render: impl FnMut() + 'static,
) -> RafCallback {
    Closure::<dyn FnMut()>::new(move || {
        if let Some(cb) = animate.borrow().as_ref() {
            frame_id.set(Some(request_animation_frame(&system.window, cb)));
            fps.tick();
            if !throttle.borrow_mut().skip() {
                render();
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
    throttle: Rc<RefCell<perf::Throttle>>,
    pause: pause::Button,
    _animate: Rc<RefCell<Option<RafCallback>>>,
}

impl Easel {
    /// # Errors
    ///
    /// Will return [`Err`] if DOM interaction fails.
    pub fn new<F: FnMut(RenderContext) + 'static>(
        system: &Rc<System>,
        mut render: F,
    ) -> Result<Self> {
        let document = &system.document;

        let canvas = new_canvas(document)?;
        let context = get_context(&canvas)?;

        let frame_id = Rc::new(Cell::new(None));
        let animate = Rc::new(RefCell::new(None));

        let pause = new_pause_button(Rc::clone(&frame_id), Rc::clone(&animate), system)?;
        let controls = document.div(["easel-controls"], (pause.root(),))?;

        let fps = Fps::new(&system.window, document)?;
        let caption = document.caption([], ())?;
        let status = document.div(["easel-status"], (&caption, fps.root()))?;

        let root = document.div([], (canvas.as_ref(), &controls, &status))?;

        let throttle = Rc::new(RefCell::new(perf::Throttle::default()));
        let cb_throttle = Rc::clone(&throttle);
        let raf_render = move || {
            render(RenderContext {
                canvas: &context,
                caption: &caption,
                throttle: cb_throttle.borrow().period(),
            });
        };

        *animate.borrow_mut() = Some(new_raf_callback(
            Rc::clone(system),
            Rc::clone(&throttle),
            Rc::clone(&frame_id),
            Rc::clone(&animate),
            fps,
            raf_render,
        ));

        Ok(Easel {
            root,
            throttle,
            pause,
            _animate: animate,
        })
    }

    /// Increase this number to slow the animation. The canvas updates on every Nth
    /// frame; so, at 60fps, a throttle of 60 updates about once per second.
    pub fn throttle(&mut self, period: NonZeroU32) {
        self.throttle.borrow_mut().set_period(period);
    }

    pub fn play(&mut self) {
        self.pause.click();
    }

    /// Creates a new [`Easel`] and immediately begins playing its animation.
    ///
    /// # Errors
    ///
    /// Will return [`Err`] if DOM interaction fails.
    pub fn start<F: FnMut(RenderContext) + 'static>(
        system: &Rc<System>,
        render: F,
    ) -> Result<Self> {
        let mut easel = Easel::new(system, render)?;
        easel.play();
        Ok(easel)
    }

    #[must_use]
    pub fn root(&self) -> &Element {
        &self.root
    }
}
