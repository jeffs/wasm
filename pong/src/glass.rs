use wasm_bindgen::prelude::*;
use web_sys::Element;

use sugar::prelude::*;
use system::System;

use crate::state::State;

/// Data shown in front of the canvas.
pub struct Glass {
    root: Element,
    state: Element,
    score: (Element, Element),
}

impl Glass {
    pub fn new(system: &System) -> Result<Self, JsValue> {
        let state = SPAN.class("pong-hello").to_element(system)?;

        let score = (
            SPAN.class("pong-score__span").to_element(system)?,
            SPAN.class("pong-score__span").to_element(system)?,
        );

        let root = DIV
            .class("pong-glass")
            .child2(&state, SPAN.class("pong-score").child2(&score.0, &score.1))
            .to_element(system)?;

        Ok(Glass { root, state, score })
    }

    pub fn set_state(&self, state: &State) {
        self.state
            .set_text_content(Some(&format!("Hello, {state:?} state.")));
    }

    pub fn set_score(&self, score: (u16, u16)) {
        self.score.0.set_text_content(Some(&score.0.to_string()));
        self.score.1.set_text_content(Some(&score.1.to_string()));
    }

    pub fn root(&self) -> &Element {
        &self.root
    }
}
