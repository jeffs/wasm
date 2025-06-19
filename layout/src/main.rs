use sugar::prelude::*;

fn main() {
    console_error_panic_hook::set_once();
    if let Err(err) = layout::immortalize("Sample", |system| {
        P.text("Your App Here").to_element(system)
    }) {
        web_sys::console::error_1(&err.into());
    }
}
