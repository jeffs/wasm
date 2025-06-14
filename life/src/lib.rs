mod app;
mod error;

pub use app::App;
pub use error::{Error, Result};

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
