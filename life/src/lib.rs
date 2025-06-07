mod app;
mod error;
pub mod js;
mod life;
mod system;

pub use app::App;
pub use error::{Error, Result};
use life::Life;
pub use system::System;

pub mod prelude {
    pub use super::js::prelude::*;
}

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
