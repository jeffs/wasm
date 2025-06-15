use primes as lib;
use system::System;

fn main_imp() -> easel::Result<()> {
    console_error_panic_hook::set_once();
    let system = System::new()?;
    let body = system.body()?;
    let mut app = Box::new(lib::Chart::new(system)?);
    body.append_child(app.root())?;
    app.play();
    Box::leak(app);
    Ok(())
}

fn main() {
    if let Err(err) = main_imp() {
        web_sys::console::error_1(&err.into());
    }
}
