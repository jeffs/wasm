use std::{cell::RefCell, ops, rc::Rc};

use magic::tag::CreateButton;
use wasm_bindgen::prelude::*;
use web_sys::{Document, Element};

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
    on_click: Box<dyn FnMut(State) + 'static>,
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
    _handle_click: Closure<dyn FnMut()>,
}

impl Button {
    pub fn new(document: &Document, on_click: impl FnMut(State) + 'static) -> Result<Button> {
        let state = State::default();
        let button = document.button(["easel-pause"], state.text())?;
        button.set_attribute("title", "Play/Pause")?;

        let cell = Rc::new(RefCell::new(Captive {
            state,
            button,
            on_click: Box::new(on_click),
        }));

        let handle_click = Closure::<dyn FnMut()>::new({
            let cell = Rc::clone(&cell);
            move || cell.borrow_mut().handle_click()
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

    pub fn root(&self) -> Element {
        self.cell.borrow().button.clone()
    }
}
