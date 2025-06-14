use std::rc::Rc;

use wasm_bindgen::prelude::*;

use life as lib;

fn main_imp() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    let system = Rc::new(system::System::new()?);
    let app = Box::new(lib::App::new(&system)?);
    system.body.append_child(app.root())?;
    Box::leak(app);
    Ok(())
}

fn main() {
    if let Err(err) = main_imp() {
        web_sys::console::error_1(&err);
    }
}
