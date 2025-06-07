use std::rc::Rc;

fn main_imp() -> life::Result<()> {
    console_error_panic_hook::set_once();
    let system = Rc::new(life::System::new()?);
    let app = Box::new(life::App::new(&system)?);
    system.body.append_child(&app.root)?;
    Box::leak(app);
    Ok(())
}

fn main() {
    if let Err(err) = main_imp() {
        web_sys::console::error_1(&err.into());
    }
}
