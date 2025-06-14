use std::rc::Rc;

use easel::{self, Easel, Result};
use system::System;
use web_sys::{CanvasRenderingContext2d, Element};

fn main_imp() -> Result<()> {
    console_error_panic_hook::set_once();

    let mut count = 0;
    let render = move |_: &CanvasRenderingContext2d, caption: &Element| {
        count += 1;
        caption.set_text_content(Some(&count.to_string()));
    };

    let system = Rc::new(System::new()?);
    let app = Box::new(Easel::new(Rc::clone(&system), render)?);
    system.body.append_child(app.root())?;
    app.play();
    Box::leak(app);
    Ok(())
}

fn main() {
    if let Err(err) = main_imp() {
        web_sys::console::error_1(&err.into());
    }
}
