mod error;
mod size;

use web_sys::{Document, HtmlElement, Window};

pub use crate::error::{Error, Result};

pub use size::{SizeF64, SizeU32, f64_to_u32_saturating, u32_to_usize, usize_to_u32};

#[derive(Clone)]
pub struct System {
    pub window: Window,
    pub document: Document,
}

impl System {
    /// # Errors
    ///
    /// Will return [`Err`] if DOM interaction fails.
    pub fn new() -> Result<System> {
        let window = web_sys::window().ok_or(Error::NoWindow)?;
        let document = window.document().ok_or(Error::NoDocument)?;
        Ok(System { window, document })
    }

    /// # Errors
    ///
    /// Will return [`Error::NoBody`] if the document body cannot be accessed.
    pub fn body(&self) -> Result<HtmlElement> {
        self.document.body().ok_or(Error::NoBody)
    }
}

impl AsRef<Document> for System {
    fn as_ref(&self) -> &Document {
        &self.document
    }
}

impl Default for System {
    fn default() -> Self {
        System::new().unwrap()
    }
}
