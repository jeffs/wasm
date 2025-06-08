use std::rc::Rc;

fn main_imp() -> lib::Result<()> {
    console_error_panic_hook::set_once();
    let system = Rc::new(lib::System::new()?);
    let app = Box::new(lib::App::new(&system)?);
    system.body.append_child(&app.root)?;
    Box::leak(app);
    Ok(())
}

fn main() {
    if let Err(err) = main_imp() {
        web_sys::console::error_1(&err.into());
    }
}
