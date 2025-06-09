//! # TODO
//!
//!
//! Designing an efficient, generic, declarative component system seems
//! difficult, because it's not clear what context any `into_component` method
//! needs. Some components need only a reference to the document, others need
//! the window, and yet others need an `Rc`. Maybe there's no such thing as
//! a component.
use web_sys::{Document, Element};

use crate::Result;

pub trait Component {
    fn root(&self) -> &Element;
}

impl Component for Element {
    fn root(&self) -> &Element {
        self
    }
}

pub trait IntoComponent {
    fn into_component(self, document: impl AsRef<Document>) -> Result<impl Component + 'static>;
}

impl IntoComponent for &str {
    fn into_component(self, document: impl AsRef<Document>) -> Result<impl Component + 'static> {
        Ok(document.as_ref().create_element(self)?)
    }
}

impl IntoComponent for [&str; 2] {
    fn into_component(self, document: impl AsRef<Document>) -> Result<impl Component + 'static> {
        let [tag, class] = self;
        let component = tag.into_component(document)?;
        component.root().set_class_name(class);
        Ok(component)
    }
}
