pub mod js;

use wasm_bindgen::JsValue;
use web_sys::Element;

use system::System;

type Result<T> = std::result::Result<T, JsValue>;

pub trait ToElement {
    /// # Errors
    ///
    /// May return [`Err`] if DOM interaction fails.
    fn to_element(&self, system: &System) -> Result<Element>;
}

impl ToElement for Element {
    fn to_element(&self, _: &System) -> Result<Element> {
        Ok(self.clone())
    }
}

impl ToElement for &Element {
    fn to_element(&self, _: &System) -> Result<Element> {
        Ok((*self).clone())
    }
}

pub struct WithText<T: ToElement>(T, &'static str);

impl<T: ToElement> ToElement for WithText<T> {
    fn to_element(&self, system: &System) -> Result<Element> {
        let element = self.0.to_element(system)?;
        element.set_text_content(Some(self.1));
        Ok(element)
    }
}

pub struct WithHtml<T: ToElement>(T, &'static str);

impl<T: ToElement> ToElement for WithHtml<T> {
    fn to_element(&self, system: &System) -> Result<Element> {
        let element = self.0.to_element(system)?;
        element.set_inner_html(self.1);
        Ok(element)
    }
}

pub struct WithChild<T: ToElement, C: ToElement>(T, C);

impl<T: ToElement, C: ToElement> ToElement for WithChild<T, C> {
    fn to_element(&self, system: &System) -> Result<Element> {
        let element = self.0.to_element(system)?;
        element.append_with_node_1(self.1.to_element(system)?.as_ref())?;
        Ok(element)
    }
}

pub struct WithChild2<T: ToElement, C0: ToElement, C1: ToElement>(T, C0, C1);

impl<T: ToElement, C0: ToElement, C1: ToElement> ToElement for WithChild2<T, C0, C1> {
    fn to_element(&self, system: &System) -> Result<Element> {
        let element = self.0.to_element(system)?;
        element.append_with_node_2(
            self.1.to_element(system)?.as_ref(),
            self.2.to_element(system)?.as_ref(),
        )?;
        Ok(element)
    }
}

pub struct WithChild3<T: ToElement, C0: ToElement, C1: ToElement, C2: ToElement>(T, C0, C1, C2);

impl<T: ToElement, C0: ToElement, C1: ToElement, C2: ToElement> ToElement
    for WithChild3<T, C0, C1, C2>
{
    fn to_element(&self, system: &System) -> Result<Element> {
        let element = self.0.to_element(system)?;
        element.append_with_node_3(
            self.1.to_element(system)?.as_ref(),
            self.2.to_element(system)?.as_ref(),
            self.3.to_element(system)?.as_ref(),
        )?;
        Ok(element)
    }
}

macro_rules! content {
    () => {
        #[must_use]
        pub const fn text(self, text: &'static str) -> WithText<Self> {
            WithText(self, text)
        }

        #[must_use]
        pub const fn html(self, html: &'static str) -> WithHtml<Self> {
            WithHtml(self, html)
        }

        #[must_use]
        pub const fn child<C: ToElement>(self, child: C) -> WithChild<Self, C> {
            WithChild(self, child)
        }

        #[must_use]
        pub const fn child2<C0: ToElement, C1: ToElement>(
            self,
            child0: C0,
            child1: C1,
        ) -> WithChild2<Self, C0, C1> {
            WithChild2(self, child0, child1)
        }

        #[must_use]
        pub const fn child3<C0: ToElement, C1: ToElement, C2: ToElement>(
            self,
            child0: C0,
            child1: C1,
            child2: C2,
        ) -> WithChild3<Self, C0, C1, C2> {
            WithChild3(self, child0, child1, child2)
        }
    };
}

pub struct WithAttr<T: ToElement>(T, &'static str, &'static str);

impl<T: ToElement> WithAttr<T> {
    #[must_use]
    pub const fn attr(self, attr: &'static str, value: &'static str) -> WithAttr<Self> {
        WithAttr(self, attr, value)
    }

    content!();
}

impl<T: ToElement> ToElement for WithAttr<T> {
    fn to_element(&self, system: &System) -> Result<Element> {
        let element = self.0.to_element(system)?;
        element.set_attribute(self.1, self.2)?;
        Ok(element)
    }
}

pub struct WithClass<T: ToElement>(T, &'static str);

impl<T: ToElement> WithClass<T> {
    #[must_use]
    pub const fn attr(self, attr: &'static str, value: &'static str) -> WithAttr<Self> {
        WithAttr(self, attr, value)
    }

    content!();
}

impl<T: ToElement> ToElement for WithClass<T> {
    fn to_element(&self, system: &System) -> Result<Element> {
        let element = self.0.to_element(system)?;
        element.set_class_name(self.1);
        Ok(element)
    }
}

pub struct Tag(&'static str);

impl Tag {
    #[must_use]
    pub const fn class(self, class: &'static str) -> WithClass<Self> {
        WithClass(self, class)
    }

    #[must_use]
    pub const fn attr(self, attr: &'static str, value: &'static str) -> WithAttr<Self> {
        WithAttr(self, attr, value)
    }

    content!();
}

impl ToElement for Tag {
    fn to_element(&self, system: &System) -> Result<Element> {
        system.document.create_element(self.0)
    }
}

pub trait CreateElement {
    /// # Errors
    ///
    /// Will return [`Err`] if the specified element cannot be created.
    fn create_element(&self, spec: impl ToElement) -> Result<Element>;
}

impl CreateElement for System {
    fn create_element(&self, spec: impl ToElement) -> Result<Element> {
        spec.to_element(self)
    }
}

pub mod prelude {
    use super::Tag;
    pub use super::js::prelude::*;
    pub use super::{CreateElement, ToElement};

    pub const A: Tag = Tag("a");
    pub const BUTTON: Tag = Tag("button");
    pub const CANVAS: Tag = Tag("canvas");
    pub const CAPTION: Tag = Tag("caption");
    pub const H1: Tag = Tag("h1");
    pub const H2: Tag = Tag("h2");
    pub const DIV: Tag = Tag("div");
    pub const HEADER: Tag = Tag("header");
    pub const LI: Tag = Tag("li");
    pub const MAIN: Tag = Tag("main");
    pub const NAV: Tag = Tag("nav");
    pub const SPAN: Tag = Tag("span");
    pub const P: Tag = Tag("p");
    pub const UL: Tag = Tag("ul");
}
