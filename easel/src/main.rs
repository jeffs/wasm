use easel::{Easel, Error, RenderContext, Result};
use system::System;

fn main_imp() -> Result<()> {
    console_error_panic_hook::set_once();

    let mut count = 0;
    let render = move |context: RenderContext| {
        count += 1;
        let caption = format!("throttle={} count={count}", context.throttle);
        context.caption.set_text_content(Some(&caption));
    };

    let system = System::new()?;
    let body = system.document.body().ok_or(Error::NoBody)?;
    let mut app = Box::new(Easel::new(system, render)?);
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
