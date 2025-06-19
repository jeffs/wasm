use std::{cell::RefCell, ops, rc::Rc};

use system::System;
use wasm_bindgen::prelude::*;
use web_sys::Element;

use sugar::prelude::*;

use crate::Result;

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

struct Captive {
    state: State,
    button: Element,
    on_click: Box<dyn Fn(State) + 'static>,
}

impl Captive {
    fn handle_click(&mut self) {
        self.state = !self.state;
        self.button.set_text_content(Some(self.state.text()));
        (self.on_click)(self.state);
    }
}

pub struct Button {
    cell: Rc<RefCell<Captive>>,
    _handle_click: Closure<dyn Fn()>,
}

impl Button {
    pub fn new(system: &System, on_click: impl Fn(State) + 'static) -> Result<Button> {
        let state = State::default();
        let cell = Rc::new(RefCell::new(Captive {
            state,
            button: BUTTON
                .class("easel-pause")
                .attr("title", "Play/Pause")
                .text(state.text())
                .to_element(system)?,
            on_click: Box::new(on_click),
        }));

        // The captive has an Rc to the closure, we so don't want the closure to
        // also have an Rc to the captive, or they'll neither will ever be dropped.
        let weak = Rc::downgrade(&cell);
        let handle_click = Closure::<dyn Fn()>::new(move || {
            let Some(cell) = weak.upgrade() else {
                return;
            };
            cell.borrow_mut().handle_click();
        });

        cell.borrow()
            .button
            .add_event_listener_with_callback("click", handle_click.as_ref().unchecked_ref())?;

        Ok(Button {
            cell,
            _handle_click: handle_click,
        })
    }

    pub fn click(&mut self) {
        self.cell.borrow_mut().handle_click();
    }

    #[must_use]
    pub fn is_paused(&self) -> bool {
        self.cell.borrow().state == State::Pause
    }

    pub fn root(&self) -> Element {
        self.cell.borrow().button.clone()
    }
}
