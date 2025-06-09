use web_sys::{Document, Element};

use crate::Result;

pub trait IntoComponent {
    /// Resources other than `self` needed to create a component.
    type Context;
    /// The target component type.
    type Component: AsRef<Element>;

    fn into_component(self, context: &Self::Context) -> Result<Self::Component>;
}

/// Self is the tag name.
impl IntoComponent for (&str,) {
    type Context = Document;
    type Component = Element;

    fn into_component(self, document: &Self::Context) -> Result<Self::Component> {
        Ok(document.create_element(self.0)?)
    }
}

/// Self is the tag name and the class name.
impl IntoComponent for (&str, &str) {
    type Context = Document;
    type Component = Element;

    fn into_component(self, document: &Self::Context) -> Result<Self::Component> {
        let element = document.create_element(self.0)?;
        element.set_class_name(self.1);
        Ok(element)
    }
}

impl<Parent: IntoComponent, Child: IntoComponent> IntoComponent for (Parent, Child)
where
    Parent::Context: AsRef<Document> + AsRef<Child::Context>,
{
    type Context = Parent::Context;
    type Component = Parent::Component;

    fn into_component(self, context: &Self::Context) -> Result<Self::Component> {
        let parent = self.0.into_component(context)?;
        let child = self.1.into_component(context.as_ref())?;
        parent.as_ref().append_child(child.as_ref())?;
        Ok(parent)
    }
}

/// The string is text content.
impl<Parent: IntoComponent> IntoComponent for (Parent, &str)
where
    Parent::Context: AsRef<Document>,
{
    type Context = Parent::Context;
    type Component = Parent::Component;

    fn into_component(self, context: &Self::Context) -> Result<Self::Component> {
        let parent = self.0.into_component(context)?;
        parent.as_ref().set_text_content(Some(self.1));
        Ok(parent)
    }
}
