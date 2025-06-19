use wasm_bindgen::JsValue;

#[derive(Debug)]
pub enum Error {
    NoWindow,
    NoDocument,
    /// The document had no body.
    /// <https://www.youtube.com/watch?v=6s7Dsb0v178&t=67s>
    NoBody,
    JsValue(JsValue),
}

impl From<Error> for JsValue {
    fn from(value: Error) -> Self {
        match value {
            Error::NoWindow => JsValue::from_str("no window"),
            Error::NoDocument => JsValue::from_str("no document"),
            Error::NoBody => JsValue::from_str("document should have a body element"),
            Error::JsValue(v) => v,
        }
    }
}

impl From<JsValue> for Error {
    fn from(value: JsValue) -> Self {
        Error::JsValue(value)
    }
}

pub type Result<T> = std::result::Result<T, Error>;
