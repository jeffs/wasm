use sugar::prelude::*;

fn main() {
    console_error_panic_hook::set_once();

    let title = "Flights of Fancy in<br />🚀 WebAssembly 🧐";

    let nav = NAV.child(
        UL.child2(
            LI.class("index-nav__item index-nav__life").child(
                A.attr("href", "/life")
                    .attr("title", "Adapted from the rustwasm book")
                    .text("Life"),
            ),
            LI.class("index-nav__item index-nav__primes").child(
                A.attr("href", "/primes")
                    .attr("title", "Adapted from the rustwasm book")
                    .text("Primes"),
            ),
        ),
    );

    if let Err(err) = layout::immortalize(title, |system| nav.to_element(system)) {
        web_sys::console::error_1(&err.into());
    }
}
