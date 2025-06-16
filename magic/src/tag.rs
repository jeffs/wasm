use web_sys::Element;

/// Provides functions for creating DOM elements from tag names, with optional
/// class names and contents.. Each tag function (`h1`, `p`, etc.) takes two
/// arguments: An array of class names, and a tuple of children. Either may
/// be empty. Moreover, rather than a tuple, a single element or string may be
/// specified as content.
///
/// The array of class names currently has a maximum size of 1. To specify
/// multiple class names, separate them with spaces.
use super::component::IntoComponent;
use super::js::Result;

pub trait Class {
    type Output: IntoComponent;

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

    /// Augments the specified tag with CSS class `self`.
    fn class(self, tag: &'static str) -> Self::Output {
        let [class] = self;
        (tag, class)
    }
}

pub trait Content<T> {
    type Output: IntoComponent;

    /// Augments the specified parent element with `self` as content.
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

/// Single child, when the child type is unambiguous..
impl<P: IntoComponent> Content<P> for Element {
    type Output = (P, Element);

    fn content(self, parent: P) -> Self::Output {
        (parent, self)
    }
}

impl<P: IntoComponent, C: IntoComponent> Content<P> for (C,)
where
    P::Context: AsRef<C::Context>,
{
    type Output = (P, Self);

    fn content(self, parent: P) -> Self::Output {
        (parent, self)
    }
}

impl<P: IntoComponent, C0: IntoComponent, C1: IntoComponent> Content<P> for (C0, C1)
where
    P::Context: AsRef<C0::Context> + AsRef<C1::Context>,
{
    type Output = (P, Self);

    fn content(self, parent: P) -> Self::Output {
        (parent, self)
    }
}

impl<P: IntoComponent, C0: IntoComponent, C1: IntoComponent, C2: IntoComponent> Content<P>
    for (C0, C1, C2)
where
    P::Context: AsRef<C0::Context> + AsRef<C1::Context> + AsRef<C2::Context>,
{
    type Output = (P, Self);

    fn content(self, parent: P) -> Self::Output {
        (parent, self)
    }
}

macro_rules! tag_with_class_and_text {
    ($trait:ident, $tag:ident) => {
        pub fn $tag<C: Class, Co: Content<C::Output>>(class: C, content: Co) -> Co::Output {
            content.content(class.class(stringify!($tag)))
        }

        pub trait $trait<C: Class, Co: Content<C::Output>> {
            /// Creates a component having the specified class and content.
            ///
            /// # Errors
            ///
            /// May return [`Err`] if DOM interaction fails.
            fn $tag(
                &self,
                class: C,
                content: Co,
            ) -> Result<<Co::Output as IntoComponent>::Component>;
        }

        impl<C: Class, Co: Content<C::Output>, T> $trait<C, Co> for T
        where
            T: AsRef<<Co::Output as IntoComponent>::Context>,
        {
            fn $tag(
                &self,
                class: C,
                content: Co,
            ) -> Result<<Co::Output as IntoComponent>::Component> {
                $tag(class, content).into_component(self.as_ref())
            }
        }
    };
}

tag_with_class_and_text!(CreateButton, button);
tag_with_class_and_text!(CreateDiv, div);
tag_with_class_and_text!(CreateH1, h1);
tag_with_class_and_text!(CreateP, p);
tag_with_class_and_text!(CreateCaption, caption);
tag_with_class_and_text!(CreateSpan, span);

pub mod prelude {
    pub use super::{
        CreateButton, CreateCaption, CreateDiv, CreateH1, CreateP, CreateSpan, button, caption,
        div, h1, p, span,
    };
}
