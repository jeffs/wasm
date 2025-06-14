use wasm_bindgen::JsValue;

#[derive(Debug)]
pub enum Error {
    NoWindow,
    NoDocument,
    NoBody,
}

impl From<Error> for JsValue {
    fn from(value: Error) -> Self {
        JsValue::from_str(match value {
            Error::NoWindow => "no window",
            Error::NoDocument => "no document",
            Error::NoBody => "no body",
        })
    }
}

pub type Result<T> = std::result::Result<T, Error>;
