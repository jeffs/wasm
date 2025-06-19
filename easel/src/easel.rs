use std::cell::RefCell;
use std::rc::{Rc, Weak};

use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, Element, HtmlCanvasElement, Performance, Window};

use perf::components::Fps;
use sugar::prelude::*;
use system::{Size, System};

use crate::{Error, Result, pause};

struct Captive {
    system: System,
    is_paused: bool,
    canvas: HtmlCanvasElement,
    /// ID returned by `request_animation_frame` so we can cancel a request.
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

/// When the fall is all there is, it matters.
/// <https://www.youtube.com/watch?v=lKGPiecEEbA>
impl Drop for Captive {
    fn drop(&mut self) {
        if let Some(id) = self.raf_handle {
            _ = self.system.window.cancel_animation_frame(id);
        }
    }
}

fn new_canvas(system: &System) -> Result<HtmlCanvasElement> {
    Ok(CANVAS
        .class("easel-canvas")
        .to_element(system)
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
///
/// # TODO:
///
/// * [] Pass time delta since last render, not counting pauses. Maybe as part
///   of the throttle?
/// * [] Rename, or support separate render and update callbacks.
pub struct RenderContext<'a> {
    pub canvas: &'a CanvasRenderingContext2d,
    pub caption: &'a Element,
    /// Milliseconds since the last render, if available.
    pub delta_ms: Option<f64>,
}

fn new_pause_button(cell: &Rc<RefCell<Captive>>) -> Result<pause::Button> {
    let weak = Rc::downgrade(cell);
    let handle_pause = move |state: pause::State| {
        let Some(strong) = weak.upgrade() else {
            return;
        };
        let mut captive = strong.borrow_mut();
        if captive.raf_handle.is_none() {
            captive.raf_handle = captive
                .raf_callback
                .as_ref()
                .map(|callback| request_animation_frame(&captive.system.window, callback));
        }
        captive.is_paused = state == pause::State::Pause;
    };
    // TODO: Pass `is_paused` to the button constructor.
    let button = pause::Button::new(&cell.borrow().system, handle_pause)?;
    debug_assert_eq!(cell.borrow().is_paused, button.is_paused());
    Ok(button)
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
    mut render: impl FnMut() + 'static,
) -> Closure<dyn FnMut()> {
    Closure::<dyn FnMut()>::new(move || {
        let Some(strong) = cell.upgrade() else {
            return;
        };
        let mut captive = strong.borrow_mut();
        if let Some(cb) = captive.raf_callback.as_ref() {
            captive.raf_handle = Some(request_animation_frame(&captive.system.window, cb));
            captive.fps.tick();
            if !captive.is_paused {
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
    cell: Rc<RefCell<Captive>>,
    pause: pause::Button,
}

impl Easel {
    /// # Errors
    ///
    /// Will return [`Err`] if DOM interaction fails.
    pub fn new<F: FnMut(RenderContext) + 'static>(system: &System, mut render: F) -> Result<Self> {
        let canvas = new_canvas(system)?;
        let context = get_context(&canvas)?;
        let canvas_element = canvas.clone();
        let cell = Rc::new(RefCell::new(Captive {
            is_paused: true,
            canvas,
            raf_handle: None,
            raf_callback: None,
            fps: Fps::new(system)?,
            system: system.clone(),
        }));

        let pause = new_pause_button(&cell)?;

        let caption = CAPTION.to_element(system)?;
        let root = DIV
            .child3(
                canvas_element.as_ref(),
                DIV.class("easel-controls").child(pause.root()),
                DIV.class("easel-status")
                    .child2(&caption, cell.borrow().fps.root()),
            )
            .to_element(system)?;

        // The time of the most recent (not throttle-skipped) call to `render`.
        //
        // TODO: Consolidate timekeeping among easel, FPS, and throttle.  Dt
        //  should omit pause time for gameplay (which is throttled), but not
        //  for FPS (which isn't).
        let mut last_ms = None::<f64>;
        let clock = system.window.performance();
        let raf_render = move || {
            let now_ms = clock.as_ref().map(Performance::now);
            let delta_ms = now_ms.and_then(|t1| last_ms.map(|t0| t1 - t0));
            render(RenderContext {
                canvas: &context,
                caption: &caption,
                delta_ms,
            });
            last_ms = now_ms;
        };

        let weak = Rc::downgrade(&cell);
        cell.borrow_mut().raf_callback = Some(new_raf_callback(weak, raf_render));

        Ok(Easel { root, cell, pause })
    }

    pub fn play(&mut self) {
        self.pause.click();
    }

    #[must_use]
    pub fn pause_button(&self) -> Element {
        self.pause.root()
    }

    #[must_use]
    pub fn is_paused(&self) -> bool {
        self.pause.is_paused()
    }

    pub fn with_canvas(&self, mut f: impl FnMut(&mut HtmlCanvasElement)) {
        f(&mut self.cell.borrow_mut().canvas);
    }

    pub fn resize_canvas(&mut self, size: Size) {
        let captive = self.cell.borrow_mut();
        captive.canvas.set_width(size.width);
        captive.canvas.set_height(size.height);
    }

    /// Creates a new [`Easel`] and immediately begins playing its animation.
    ///
    /// # Errors
    ///
    /// Will return [`Err`] if DOM interaction fails.
    pub fn start<F: FnMut(RenderContext) + 'static>(system: &System, render: F) -> Result<Self> {
        let mut easel = Easel::new(system, render)?;
        easel.play();
        Ok(easel)
    }
}

impl AsRef<Element> for Easel {
    fn as_ref(&self) -> &Element {
        &self.root
    }
}
