use std::{cell::RefCell, rc::Rc};

use wasm_bindgen::prelude::*;
use web_sys::{Element, KeyboardEvent, Performance};

use easel::{Easel, RenderContext, Result};
use sugar::prelude::*;
use system::{SizeF64, System, f64_to_u32_saturating};

use crate::{constants::CANVAS_SIZE, game::Game, glass::Glass, physics::Direction};

fn random_game(system: &System) -> Game {
    let seed = system
        .window
        .performance()
        .as_ref()
        .map(Performance::now)
        .map(f64_to_u32_saturating)
        .unwrap_or_default();
    Game::from_seed(seed)
}

fn keydown_handler(
    system: &System,
    game: Rc<RefCell<Game>>,
    easel: Rc<RefCell<Easel>>,
) -> Closure<dyn Fn(KeyboardEvent)> {
    let closure_system = system.clone();
    let handle_keydown = Closure::<dyn Fn(KeyboardEvent)>::new(move |event: KeyboardEvent| {
        // The play/pause functionality of "p" is for the easel, not the
        // game state, and is mostly a debugging tool. When the easel is
        // paused, the game should do nothing at all.
        if easel.borrow().is_paused() {
            match event.key().as_str() {
                "p" => easel.borrow_mut().play(),
                _ => return,
            }
        } else {
            match event.key().as_str() {
                "p" => easel.borrow_mut().play(),
                "b" => {
                    let mut game = game.borrow_mut();
                    if !game.start() {
                        *game = random_game(&closure_system);
                    }
                }
                "1" => game.borrow_mut().player1_score(),
                "2" => game.borrow_mut().player2_score(),
                "s" => game.borrow_mut().player1_move(Some(Direction::Down)),
                "w" => game.borrow_mut().player1_move(Some(Direction::Up)),
                "ArrowDown" => game.borrow_mut().player2_move(Some(Direction::Down)),
                "ArrowUp" => game.borrow_mut().player2_move(Some(Direction::Up)),
                _ => return,
            }
        }
        event.prevent_default();
    });
    system
        .window
        .set_onkeydown(Some(handle_keydown.as_ref().unchecked_ref()));
    handle_keydown
}

fn keyup_handler(system: &System, game: Rc<RefCell<Game>>) -> Closure<dyn Fn(KeyboardEvent)> {
    let handle_keyup = Closure::<dyn Fn(KeyboardEvent)>::new(move |event: KeyboardEvent| {
        match event.key().as_str() {
            "s" | "w" => game.borrow_mut().player1_move(None),
            "ArrowDown" | "ArrowUp" => game.borrow_mut().player2_move(None),
            _ => return,
        }
        event.prevent_default();
    });
    system
        .window
        .set_onkeyup(Some(handle_keyup.as_ref().unchecked_ref()));
    handle_keyup
}

pub struct App {
    root: Element,
    _game: Rc<RefCell<Game>>,
    _easel: Rc<RefCell<Easel>>,
    _handle_keydown: Closure<dyn Fn(KeyboardEvent)>,
    _handle_keyup: Closure<dyn Fn(KeyboardEvent)>,
}

impl App {
    /// # Errors
    ///
    /// Will return [`Err`] if DOM interaction fails.
    pub fn new(system: &System) -> Result<Self> {
        let mut generation = 0;

        // TODO: Optionally seed the game from user input, a la Minecraft.
        let game = random_game(system);

        let glass = Rc::new(Glass::new(system)?);
        glass.set_state(game.state());
        glass.set_score(game.score());

        let game = Rc::new(RefCell::new(game));
        let easel_game = Rc::clone(&game);
        let easel_glass = Rc::clone(&glass);
        let mut easel = Easel::new(system, move |context: RenderContext| {
            let mut game = easel_game.borrow_mut();
            game.update(context.delta_ms);
            // Render the canvas.
            game.render(context.canvas);
            // Render the overlay.
            easel_glass.set_state(game.state());
            easel_glass.set_score(game.score());
            // Render the caption.
            generation += 1;
            let SizeF64 { width, height } = game.size();
            let caption = format!("{width}x{height} @ {generation}");
            context.caption.set_text_content(Some(&caption));
        })?;

        easel.resize_canvas(CANVAS_SIZE);
        easel.play();

        let help = DIV.class("pong-help").child3(
            DIV.class("pong-help-column").child2(
                DIV.class("pong-help-row")
                    .child(SPAN.class("pong-help-key").text("w")),
                DIV.class("pong-help-row")
                    .child(SPAN.class("pong-help-key").text("s")),
            ),
            DIV.class("pong-help-column pong-help-game").child2(
                DIV.class("pong-help-row").child2(
                    SPAN.class("pong-help-key").text("b"),
                    SPAN.text(" to begin"),
                ),
                DIV.class("pong-help-row").child2(
                    SPAN.class("pong-help-key").text("p"),
                    SPAN.text(" to pause"),
                ),
            ),
            DIV.class("pong-help-column").child2(
                DIV.class("pong-help-row")
                    .child(SPAN.class("pong-help-key").text("↑")),
                DIV.class("pong-help-row")
                    .child(SPAN.class("pong-help-key").text("↓")),
            ),
        );

        let root = DIV
            .class("pong")
            .child3(easel.as_ref(), glass.root(), help)
            .to_element(system)?;

        let easel = Rc::new(RefCell::new(easel));

        Ok(App {
            root,
            _handle_keydown: keydown_handler(system, Rc::clone(&game), Rc::clone(&easel)),
            _handle_keyup: keyup_handler(system, Rc::clone(&game)),
            _game: game,
            _easel: easel,
        })
    }
}

impl AsRef<Element> for App {
    fn as_ref(&self) -> &Element {
        &self.root
    }
}
