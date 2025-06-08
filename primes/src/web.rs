pub trait Log {
    /// # Errors
    ///
    /// Will return an error if [`self`] cannot be converted to a [`JsValue`].
    fn log(self) -> Result<(), JsValue>;
}

impl<T: IntoJs> Log for T {
    fn log(self) -> Result<(), JsValue> {
        web_sys::console::log_1(&self.into_js()?);
        Ok(())
    }
}

impl<T0: IntoJs, T1: IntoJs> Log for (T0, T1) {
    fn log(self) -> Result<(), JsValue> {
        web_sys::console::log_2(&self.0.into_js()?, &self.1.into_js()?);
        Ok(())
    }
}
