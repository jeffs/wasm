use wasm_bindgen::JsValue;

fn main() {
    console_error_panic_hook::set_once();

    if let Err(err) = layout::immortalize("🧱 Prime 🏗 <br />Factorization", |system| {
        let mut chart = primes::Chart::new(system).map_err(JsValue::from)?;
        chart.play();
        Ok(chart)
    }) {
        web_sys::console::error_1(&err.into());
    }
}
