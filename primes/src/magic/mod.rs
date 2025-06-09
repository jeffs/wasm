use web_sys::{Document, Element};

use crate::Result;

pub trait IntoComponent {
    /// Resources other than `self` needed to create a component.
    type Context;
    /// The target component type.
    type Component;

    fn into_component(self, context: &Self::Context) -> Result<Self::Component>;
}

/// Self is the tag name.
impl IntoComponent for &str {
    type Context = Document;
    type Component = Element;

    fn into_component(self, document: &Self::Context) -> Result<Self::Component> {
        Ok(document.create_element(self)?)
    }
}

/// Self is the tag name and the class name.
impl IntoComponent for [&str; 2] {
    type Context = Document;
    type Component = Element;

    fn into_component(self, document: &Self::Context) -> Result<Self::Component> {
        let [tag, class] = self;
        let element = tag.into_component(document)?;
        element.set_class_name(class);
        Ok(element)
    }
}
