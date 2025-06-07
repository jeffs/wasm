use crate::error::{Error, Result};

pub struct System {
    pub window: web_sys::Window,
    pub document: web_sys::Document,
    pub body: web_sys::HtmlElement,
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

impl Default for System {
    fn default() -> Self {
        System::new().unwrap()
    }
}
