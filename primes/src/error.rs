use wasm_bindgen::JsValue;

#[derive(Debug)]
pub enum Error {
    NoWindow,
    NoDocument,
    NoBody,
    /// A type conversion failed.
    Cast {
        from: &'static str,
        to: &'static str,
    },
    Str(&'static str),
    JsValue(JsValue),
}

impl From<JsValue> for Error {
    fn from(value: JsValue) -> Self {
        Error::JsValue(value)
    }
}

impl From<Error> for JsValue {
    fn from(value: Error) -> Self {
        match value {
            Error::NoWindow => JsValue::from_str("no window"),
            Error::NoDocument => JsValue::from_str("no document"),
            Error::NoBody => JsValue::from_str("no body"),
            Error::Cast { from, to } => JsValue::from_str(&format!("{from} is not {to}")),
            Error::Str(s) => JsValue::from_str(s),
            Error::JsValue(value) => value,
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;
