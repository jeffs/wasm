use wasm_bindgen::JsValue;

#[derive(Debug)]
pub enum Error {
    NoWindow,
    NoDocument,
    /// The document had no body.
    /// <https://www.youtube.com/watch?v=6s7Dsb0v178&t=67s>
    NoBody,
}

impl From<Error> for JsValue {
    fn from(value: Error) -> Self {
        JsValue::from_str(match value {
            Error::NoWindow => "no window",
            Error::NoDocument => "no document",
            Error::NoBody => "document should have a body element",
        })
    }
}

pub type Result<T> = std::result::Result<T, Error>;
