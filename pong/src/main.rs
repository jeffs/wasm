use wasm_bindgen::JsValue;

fn main() {
    console_error_panic_hook::set_once();

    let app = match layout::showcase("🏓 Pong 🕹️", |system| {
        pong::App::new(system).map_err(JsValue::from)
    }) {
        Ok(app) => app,
        Err(err) => {
            web_sys::console::error_1(&err.into());
            return;
        }
    };

    Box::leak(app);
}
