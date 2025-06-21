use wasm_bindgen::JsValue;

fn main() {
    console_error_panic_hook::set_once();

    if let Err(err) = layout::immortalize("🏓 Pong 🕹️", |system| {
        pong::App::new(system).map_err(JsValue::from)
    }) {
        web_sys::console::error_1(&err.into());
    }
}
