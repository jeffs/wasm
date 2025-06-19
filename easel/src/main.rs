use easel::{Easel, RenderContext};

fn main() {
    console_error_panic_hook::set_once();

    let mut count = 0;
    let render = move |context: RenderContext| {
        count += 1;
        let caption = format!("count={count}");
        context.caption.set_text_content(Some(&caption));
    };

    if let Err(err) = layout::immortalize("Easel", |system| {
        let mut app = Easel::new(system, render)?;
        app.play();
        Ok(app)
    }) {
        web_sys::console::error_1(&err.into());
    }
}
