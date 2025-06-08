mod app;
mod chart;
mod error;
pub mod js;
mod system;

pub use app::App;
use chart::Chart;
pub use error::{Error, Result};
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
