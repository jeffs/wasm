use wasm_bindgen::prelude::*;

use web_sys::{Document, Element, Window};

use crate::Counter;

/// Update @ ~2 Hz when [`Fps::tick`] is called from animation frames.
const LAP_TICKS: u32 = 30;

#[derive(Debug)]
pub enum FpsError {
    /// A DOM element could not be created.
    Creation(JsValue),
    /// The Performance API is unavailable.
    Performance,
}

/// Tracks and displays Frames Per Second (FPS).
pub struct Fps {
    root: Element,
    counter: Counter,
}

impl Fps {
    /// Creates (but does not mount) an element in the specified document,
    /// using the Performance API of the specified window. Starts a performance
    /// counter immediately upon construction.
    ///
    /// # Errors
    ///
    /// Will return [`Err`] if the element cannot be created.
    pub fn new(window: &Window, document: &Document) -> Result<Self, FpsError> {
        let root = document.create_element("p").map_err(FpsError::Creation)?;
        let counter = Counter::try_start(window, LAP_TICKS).ok_or(FpsError::Performance)?;
        Ok(Fps { root, counter })
    }

    pub fn tick(&mut self) {
        if let Some(fps) = self.counter.tick() {
            self.root.set_text_content(Some(&format!("FPS: {fps:.1}")));
        }
    }
}

impl AsRef<Element> for Fps {
    fn as_ref(&self) -> &Element {
        &self.root
    }
}
