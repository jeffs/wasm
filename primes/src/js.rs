//! Interoperability with JavaScript via the [`web_sys`], [`js_sys`], and
//! [`wasm_bindgen`] crates.

use std::result::Result as StdResult;

use wasm_bindgen::prelude::*;

use crate::{Error, Result};

pub trait IntoJs {
    /// # Errors
    ///
    /// Forwards any error from recursive conversion (e.g., from map values to
    /// object properties), or from JavaScript reflection API failures.
    fn into_js(self) -> StdResult<JsValue, JsValue>;
}

impl IntoJs for JsValue {
    fn into_js(self) -> StdResult<JsValue, JsValue> {
        Ok(self)
    }
}

macro_rules! into {
    ($t:ty) => {
        impl IntoJs for $t {
            fn into_js(self) -> StdResult<JsValue, JsValue> {
                Ok(self.into())
            }
        }
    };
}

into!(&str);
into!(bool);
into!(f64);
into!(u32);
into!(usize);

impl<K: IntoJs, V: IntoJs, const N: usize> IntoJs for [(K, V); N] {
    fn into_js(self) -> StdResult<JsValue, JsValue> {
        let object = js_sys::Object::new();
        for (key, value) in self {
            js_sys::Reflect::set(&object, &key.into_js()?, &value.into_js()?)?;
        }
        Ok(object.into())
    }
}

pub trait DynCast {
    /// # Errors
    ///
    /// Returns [`Error::Cast`] if the receiver is not of the target type.
    fn dyn_cast<T: JsCast>(self) -> Result<T>;
}

impl<J: JsCast> DynCast for J {
    fn dyn_cast<T: JsCast>(self) -> Result<T> {
        self.dyn_into::<T>().map_err(|e| {
            let from = std::any::type_name_of_val(&e);
            let to = std::any::type_name::<T>();
            Error::Cast { from, to }
        })
    }
}

pub mod prelude {
    pub use super::{DynCast, IntoJs};
}
