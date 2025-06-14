use wasm_bindgen::prelude::*;

use web_sys::{Document, Element, Window};

use crate::Counter;

/// Update @ ~1 Hz when [`Fps::tick`] is called from animation frames @ 60 Hz.
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
        let root = document
            .create_element("span")
            .map_err(FpsError::Creation)?;
        // Set content so the paragraph isn't collapsed to zero height.
        root.set_inner_html("&nbsp;");
        let counter = Counter::try_start(window, LAP_TICKS).ok_or(FpsError::Performance)?;
        Ok(Fps { root, counter })
    }

    pub fn tick(&mut self) {
        if let Some(fps) = self.counter.tick() {
            self.root.set_text_content(Some(&format!("{fps:.01} fps")));
        }
    }

    #[must_use]
    pub fn root(&self) -> &Element {
        &self.root
    }
}
