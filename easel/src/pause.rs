use std::{cell::Cell, ops, rc::Rc};

use magic::tag::CreateButton;
use wasm_bindgen::prelude::*;
use web_sys::{Document, Element};

use crate::{RafCallback, Result};

#[derive(Clone, Copy, Default, PartialEq)]
pub enum State {
    #[default]
    Pause,
    Play,
}

impl State {
    /// Returns a "pause" indicator for play, and a "play" indicator for pause.
    /// This may not be intuitive: The text on a button shows what the button
    /// will do, not what it already did.
    fn text(self) -> &'static str {
        match self {
            State::Pause => "⏵︎",
            State::Play => "⏸︎",
        }
    }
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
    root: Element,
    state: Rc<Cell<State>>,
    on_click: Rc<dyn Fn(State) + 'static>,
    _handle_click: RafCallback,
}

impl Button {
    pub fn new(document: &Document, on_click: Rc<dyn Fn(State) + 'static>) -> Result<Button> {
        let state = Rc::new(Cell::new(State::default()));
        let button = document.button(["easel-pause"], state.get().text())?;
        button.set_attribute("title", "Play/Pause")?;
        let cb_state = Rc::clone(&state);
        let cb_button = button.clone();
        let cb_on_click = Rc::clone(&on_click);
        let cb = Closure::<dyn FnMut()>::new(move || {
            let state = !cb_state.get();
            cb_state.set(state);
            cb_button.set_text_content(Some(state.text()));
            cb_on_click(state);
        });
        button.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref())?;
        Ok(Button {
            root: button,
            state,
            on_click,
            _handle_click: cb,
        })
    }

    pub fn click(&self) {
        let state = !self.state.get();
        self.state.set(state);
        self.root.set_text_content(Some(state.text()));
        self.on_click.as_ref()(state);
    }

    pub fn root(&self) -> &Element {
        &self.root
    }
}
