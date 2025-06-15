use std::cell::{Cell, RefCell};
use std::{num::NonZeroU32, rc::Rc};

use perf::components::Fps;
use system::Size;
use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, Document, Element, HtmlCanvasElement, Window};

use magic::prelude::*;

use crate::{Error, Result, System, pause};

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
    /// Callback for [`Window::request_animation_frame`]. So much indirection
    /// is required because the function is self-referential: Each time it is
    /// called, it must schedule the next call to itself.
    #[expect(clippy::type_complexity)]
    _animate: Rc<RefCell<Option<Closure<dyn FnMut()>>>>,
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

        let cb_frame_id = Rc::clone(&frame_id);
        let cb_animate = Rc::clone(&animate);
        let cb_system = Rc::clone(system);
        let handle_pause = Rc::new(move |state| {
            cb_frame_id.set(match state {
                pause::State::Pause => cb_frame_id.get().and_then(|handle| {
                    cb_system
                        .window
                        .cancel_animation_frame(handle)
                        .err()
                        .map(|_| handle)
                }),
                pause::State::Play => cb_animate
                    .borrow()
                    .as_ref()
                    .map(|animate| request_animation_frame(&cb_system.window, animate)),
            });
        });
        let pause = pause::Button::new(document, handle_pause)?;
        let controls = document.div(["easel-controls"], (pause.root(),))?;

        let mut fps = Fps::new(&system.window, document)?;
        let caption = document.caption([], ())?;
        let status = document.div(["easel-status"], (&caption, fps.root()))?;

        let root = document.div([], (canvas.as_ref(), &controls, &status))?;

        // Well, I'll be a reference-counted cell of an option! We need to
        // hand the `request_animation_frame` callback to the runtime system so
        // it can be, uh, called back. But it also needs a reference to itself,
        // so that on the first call, it can schedule the next, and so on.
        //
        // Kudos to the wasm-bindgen guide for the Rc/RefCell/Option workaround:
        // <https://rustwasm.github.io/wasm-bindgen/examples/request-animation-frame.html>
        let throttle = Rc::new(RefCell::new(perf::Throttle::default()));
        let raf_system = Rc::clone(system);
        let raf_throttle = Rc::clone(&throttle);
        let raf_frame_id = Rc::clone(&frame_id);
        let raf_animate = Rc::clone(&animate);
        *animate.borrow_mut() = Some(Closure::<dyn FnMut()>::new(move || {
            if let Some(cb) = raf_animate.borrow().as_ref() {
                raf_frame_id.set(Some(request_animation_frame(&raf_system.window, cb)));
                fps.tick();
                let mut throttle = raf_throttle.borrow_mut();
                if throttle.skip() {
                    return;
                }
                render(RenderContext {
                    canvas: &context,
                    caption: &caption,
                    throttle: throttle.period(),
                });
            }
        }));

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

    pub fn play(&self) {
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
        let easel = Easel::new(system, render)?;
        easel.play();
        Ok(easel)
    }

    #[must_use]
    pub fn root(&self) -> &Element {
        &self.root
    }
}
