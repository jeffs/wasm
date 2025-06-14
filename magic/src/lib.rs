pub mod component;
pub mod js;
pub mod tag;

pub mod prelude {
    pub use super::component::IntoComponent;
    pub use super::js::prelude::*;
    pub use super::tag::prelude::*;
}

/// Requires the web-sys "console" feature in the calling crate.
#[macro_export]
macro_rules! dbg {
    ($expr:expr) => {
        web_sys::console::debug_3(
            &stringify!($expr).into(),
            &"=".into(),
            &$crate::js::IntoJs::into_js($expr).unwrap(),
        );
    };
}
