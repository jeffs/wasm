use wasm_bindgen::JsValue;
use web_sys::Element;

use sugar::prelude::*;
use system::System;

/// Values matching the styles defined in this package's CSS.
pub mod color {
    pub const IVORY: &str = "hsl(60, 100%, 97%)";
    pub const PEWTER: &str = "hsl(159, 7%, 57%)";

    pub const FG: &str = PEWTER;
}

/// Initializes the document body with the specified title and main component.
///
/// # Errors
///
/// Will return [`Err`] if DOM interaction fails.
pub fn showcase<C: AsRef<Element>, F: FnOnce(&System) -> Result<C, JsValue>>(
    title_html: &'static str,
    create_app: F,
) -> system::Result<Box<C>> {
    let system = System::new()?;
    let body = system.body()?;
    let app = Box::new(create_app(&system)?);
    body.append_with_node_2(
        HEADER
            .class("layout-header")
            .child(A.attr("href", "/").text("💭 Jeff's Wasm"))
            .to_element(&system)?
            .as_ref(),
        MAIN.class("layout-main")
            .child2(
                H1.class("layout-main__title").html(title_html),
                (*app).as_ref(),
            )
            .to_element(&system)?
            .as_ref(),
    )?;
    Ok(app)
}

/// Initializes the document body with the specified title and main component.
/// Leaks the component, so that it won't be dropped.
///
/// # Errors
///
/// Will return [`Err`] if DOM interaction fails.
pub fn immortalize<C: AsRef<Element>, F: FnOnce(&System) -> Result<C, JsValue>>(
    title_html: &'static str,
    create_app: F,
) -> system::Result<()> {
    let app = showcase(title_html, create_app)?;
    Box::leak(app);
    Ok(())
}
