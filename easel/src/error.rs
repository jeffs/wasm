use perf::components::FpsError;
use wasm_bindgen::JsValue;

#[derive(Debug)]
pub enum Error {
    /// The window or document could not be accessed.
    System(system::Error),
    /// The canvas could not provide a 2D drawing context.
    Context2d,
    /// The FPS component could not be instantiated.
    Fps(FpsError),
    /// May indicate failure of DOM node interaction or type coercion.
    JsValue(JsValue),
}

impl From<system::Error> for Error {
    fn from(value: system::Error) -> Self {
        Error::System(value)
    }
}

impl From<FpsError> for Error {
    fn from(value: FpsError) -> Self {
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
            Error::System(e) => e.into(),
            Error::Context2d => JsValue::from_str("canvas should have a 2D drawing context"),
            Error::Fps(e) => JsValue::from_str(&format!("{e:?}")),
            Error::JsValue(value) => value,
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;
