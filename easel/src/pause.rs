use std::{cell::Cell, ops, rc::Rc};

use wasm_bindgen::prelude::*;
use web_sys::{Document, Element};

use crate::Result;

#[derive(Clone, Copy, Default, PartialEq)]
pub enum State {
    #[default]
    Pause,
    Play,
}

impl ops::Not for State {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            State::Pause => State::Play,
            State::Play => State::Pause,
        }
    }
}

pub struct Button {
    button: Element,
    state: Rc<Cell<State>>,
    on_click: Rc<dyn Fn(State) + 'static>,
    _handle_click: Closure<dyn FnMut()>,
}

impl Button {
    pub fn new(document: &Document, on_click: Rc<dyn Fn(State) + 'static>) -> Result<Button> {
        let state = Rc::new(Cell::new(State::default()));
        let cb_state = Rc::clone(&state);
        let cb_on_click = Rc::clone(&on_click);
        let cb = Closure::<dyn FnMut()>::new(move || {
            let state = !cb_state.get();
            cb_state.set(state);
            cb_on_click(state);
        });
        let button = document.create_element("button")?;
        button.set_text_content(Some("||"));
        button.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref())?;
        Ok(Button {
            button,
            state,
            on_click,
            _handle_click: cb,
        })
    }

    pub fn click(&self) {
        let state = !self.state.get();
        self.state.set(state);
        self.on_click.as_ref()(state);
    }

    pub fn root(&self) -> &Element {
        &self.button
    }
}
