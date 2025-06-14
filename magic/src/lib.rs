pub mod component;
pub mod js;
pub mod tag;

pub mod prelude {
    pub use super::component::IntoComponent;
    pub use super::js::prelude::*;
    pub use super::tag::prelude::*;
}
