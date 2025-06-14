use std::rc::Rc;

use crate::{Chart, Result, System, magic::prelude::*};

pub struct App {
    pub root: web_sys::Element,
    _life: Chart,
}

impl App {
    /// # Errors
    ///
    /// Will return [`Err`] if DOM interaction fails.
    pub fn new(system: &Rc<System>) -> Result<Self> {
        let life = Chart::new(system)?;
        let root = system.document.div(["primes"], (&life.root,))?;
        Ok(App { root, _life: life })
    }
}
