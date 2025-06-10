use web_sys::{Document, Element, HtmlCanvasElement};

use crate::Result;

pub struct EmptyContext;

impl AsRef<EmptyContext> for Document {
    fn as_ref(&self) -> &EmptyContext {
        &EmptyContext
    }
}

pub trait IntoComponent {
    /// Resources other than `self` needed to create a component.
    type Context;
    /// The target component type.
    type Component: AsRef<Element>;

    fn into_component(self, context: &Self::Context) -> Result<Self::Component>;
}

/// TODO: Is this useful?
impl IntoComponent for Element {
    type Context = EmptyContext;
    type Component = Self;

    fn into_component(self, _: &Self::Context) -> Result<Self::Component> {
        Ok(self)
    }
}

impl IntoComponent for &Element {
    type Context = EmptyContext;
    type Component = Self;

    fn into_component(self, _: &Self::Context) -> Result<Self::Component> {
        Ok(self)
    }
}

impl IntoComponent for &HtmlCanvasElement {
    type Context = EmptyContext;
    type Component = Self;

    fn into_component(self, _: &Self::Context) -> Result<Self::Component> {
        Ok(self)
    }
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

impl<Parent: IntoComponent, Child: IntoComponent> IntoComponent for (Parent, (Child,))
where
    Parent::Context: AsRef<Document> + AsRef<Child::Context>,
{
    type Context = Parent::Context;
    type Component = Parent::Component;

    fn into_component(self, context: &Self::Context) -> Result<Self::Component> {
        let parent = self.0.into_component(context)?;
        let child = self.1.0.into_component(context.as_ref())?;
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

impl<Parent: IntoComponent, Child0: IntoComponent, Child1: IntoComponent> IntoComponent
    for (Parent, (Child0, Child1))
where
    Parent::Context: AsRef<Document> + AsRef<Child0::Context> + AsRef<Child1::Context>,
{
    type Context = Parent::Context;
    type Component = Parent::Component;

    fn into_component(self, context: &Self::Context) -> Result<Self::Component> {
        let parent = self.0.into_component(context)?;
        let child0 = self.1.0.into_component(context.as_ref())?;
        let child1 = self.1.1.into_component(context.as_ref())?;
        parent
            .as_ref()
            .append_with_node_2(child0.as_ref(), child1.as_ref())?;
        Ok(parent)
    }
}

impl<Parent: IntoComponent, Child0: IntoComponent, Child1: IntoComponent, Child2: IntoComponent>
    IntoComponent for (Parent, (Child0, Child1, Child2))
where
    Parent::Context:
        AsRef<Document> + AsRef<Child0::Context> + AsRef<Child1::Context> + AsRef<Child2::Context>,
{
    type Context = Parent::Context;
    type Component = Parent::Component;

    fn into_component(self, context: &Self::Context) -> Result<Self::Component> {
        let parent = self.0.into_component(context)?;
        let child0 = self.1.0.into_component(context.as_ref())?;
        let child1 = self.1.1.into_component(context.as_ref())?;
        let child2 = self.1.2.into_component(context.as_ref())?;
        parent
            .as_ref()
            .append_with_node_3(child0.as_ref(), child1.as_ref(), child2.as_ref())?;
        Ok(parent)
    }
}
