use web_sys::{Document, HtmlElement, Window};

use crate::error::{Error, Result};

pub struct System {
    pub window: Window,
    pub document: Document,
    pub body: HtmlElement,
}

impl System {
    /// # Errors
    ///
    /// Will return [`Err`] if DOM interaction fails.
    pub fn new() -> Result<System> {
        let window = web_sys::window().ok_or(Error::NoWindow)?;
        let document = window.document().ok_or(Error::NoDocument)?;
        Ok(System {
            body: document.body().ok_or(Error::NoBody)?,
            window,
            document,
        })
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
