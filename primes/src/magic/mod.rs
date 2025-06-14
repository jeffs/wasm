pub mod component;
pub mod tag;

pub mod prelude {
    pub use super::component::IntoComponent;
    pub use super::tag::prelude::*;
}
