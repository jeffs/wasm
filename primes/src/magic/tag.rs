//! Provides functions for common tags. Each tag function (`h1`, `p`, etc.)
//! takes two arguments: An array of class names, and a tuple of children.
//! Either may be empty.
//!
//! The array of class names currently has a maximum size of 1. To specify
//! multiple class names, separate them with spaces.

use web_sys::{Document, Element};

use super::component::IntoComponent;

pub trait Class {
    type Output: IntoComponent<Context = Document, Component = Element>;

    fn class(self, tag: &'static str) -> Self::Output;
}

impl Class for [&str; 0] {
    type Output = (&'static str,);

    fn class(self, tag: &'static str) -> Self::Output {
        (tag,)
    }
}

impl<'a> Class for [&'a str; 1] {
    type Output = (&'static str, &'a str);

    fn class(self, tag: &'static str) -> Self::Output {
        let [class] = self;
        (tag, class)
    }
}

pub trait Content<T> {
    type Output;

    fn content(self, parent: T) -> Self::Output;
}

impl<P: IntoComponent> Content<P> for &str {
    type Output = (P, Self);

    fn content(self, parent: P) -> Self::Output {
        (parent, self)
    }
}

/// No content.
impl<P: IntoComponent> Content<P> for () {
    type Output = P;

    fn content(self, parent: P) -> Self::Output {
        parent
    }
}

impl<P: IntoComponent, C: IntoComponent> Content<P> for C {
    type Output = (P, Self);

    fn content(self, parent: P) -> Self::Output {
        (parent, self)
    }
}

macro_rules! tag_with_class_and_text {
    ($tag:ident) => {
        pub fn $tag<C: Class, Co: Content<C::Output>>(class: C, content: Co) -> Co::Output {
            content.content(class.class(stringify!(tag)))
        }
    };
}

tag_with_class_and_text!(h1);
tag_with_class_and_text!(p);

pub mod prelude {
    pub use super::{h1, p};
}
