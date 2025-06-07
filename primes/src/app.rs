use std::rc::Rc;

use crate::{Chart, Result, System};

pub struct App {
    pub root: web_sys::Element,
    _life: Chart,
}

impl App {
    /// # Errors
    ///
    /// Will return [`Err`] if DOM interaction fails.
    pub fn new(system: &Rc<System>) -> Result<Self> {
        let root = system.document.create_element("div")?;
        root.set_class_name("app");

        let life = Chart::new(system)?;

        root.append_with_node_1(&life.root)?;

        Ok(App { root, _life: life })
    }
}
