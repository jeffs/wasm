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
    /// The canvas could not provide a 2D drawing context.
    Context2d,
    /// The FPS component could not be instantiated.
    Fps(perf::FpsError),
    JsValue(JsValue),
}

impl From<perf::FpsError> for Error {
    fn from(value: perf::FpsError) -> Self {
        Error::Fps(value)
    }
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
            Error::Context2d => JsValue::from_str("canvas should have a 2D drawing context"),
            Error::Fps(e) => JsValue::from_str(&format!("{e:?}")),
            Error::JsValue(value) => value,
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;
