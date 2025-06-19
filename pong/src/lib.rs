use std::{cell::RefCell, rc::Rc};

use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, Element, KeyboardEvent};

use easel::{Easel, RenderContext, Result};
use sugar::prelude::*;
use system::{Size, System};

const COURT_COLOR: &str = "rgb(40, 45, 52)";
const BALL_COLOR: &str = layout::color::IVORY;

const CANVAS_SCALE: u32 = 3;

/// Roughly 16:9.
const VIRTUAL_SIZE: Size = Size {
    width: 426,
    height: 240,
};

const CANVAS_SIZE: Size = Size {
    width: VIRTUAL_SIZE.width * CANVAS_SCALE,
    height: VIRTUAL_SIZE.height * CANVAS_SCALE,
};

const BALL_SIZE: u32 = 5;

const PADDLE_HEIGHT: u32 = 20;

/// Virtual units per second.
const PADDLE_SPEED: u32 = 200;

#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
fn paddle_distance(delta_ms: f64) -> u32 {
    // See also:
    // <https://doc.rust-lang.org/reference/expressions/operator-expr.html#r-expr.as.numeric.float-as-int>
    (f64::from(PADDLE_SPEED) * delta_ms) as u32 / 1000
}

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
}

fn paddle_pos(old: u32, dir: Option<Direction>, delta_ms: f64) -> u32 {
    match dir {
        Some(Direction::Up) => old.saturating_sub(paddle_distance(delta_ms)),
        Some(Direction::Down) => {
            (old + paddle_distance(delta_ms)).min(VIRTUAL_SIZE.height - PADDLE_HEIGHT)
        }
        None => old,
    }
}

struct Game {
    score: (u16, u16),
    paddle_y: (u32, u32),
    move_: (Option<Direction>, Option<Direction>),
}

impl Game {
    fn new() -> Self {
        // TODO: Initialize game state.
        Game {
            score: (0, 0),
            paddle_y: (20, VIRTUAL_SIZE.height - 40),
            move_: (None, None),
        }
    }

    fn player1_score(&mut self) {
        self.score.0 += 1;
    }

    fn player1_start_up(&mut self) {
        self.move_.0 = Some(Direction::Up);
    }

    fn player1_start_down(&mut self) {
        self.move_.0 = Some(Direction::Down);
    }

    fn player1_stop_up(&mut self) {
        self.move_.0 = None;
    }

    fn player1_stop_down(&mut self) {
        self.move_.0 = None;
    }

    fn player2_start_up(&mut self) {
        self.move_.1 = Some(Direction::Up);
    }

    fn player2_start_down(&mut self) {
        self.move_.1 = Some(Direction::Down);
    }

    fn player2_stop_up(&mut self) {
        self.move_.1 = None;
    }

    fn player2_stop_down(&mut self) {
        self.move_.1 = None;
    }

    fn increment_player2_score(&mut self) {
        self.score.1 += 1;
    }

    fn reset(&mut self) {
        self.score = (0, 0);
    }

    /// Updates the state of this game; should be called immediately before
    /// render. The `delta_ms` is elapsed game time since the last tick.
    #[allow(clippy::unused_self)]
    fn tick(&mut self, delta_ms: Option<f64>) {
        let Some(dt) = delta_ms else {
            return;
        };
        self.paddle_y = (
            paddle_pos(self.paddle_y.0, self.move_.0, dt),
            paddle_pos(self.paddle_y.1, self.move_.1, dt),
        );
    }
}

fn draw(context: &CanvasRenderingContext2d, game: &Game) {
    context.begin_path();
    context.set_fill_style_str(COURT_COLOR);
    // Draw the background.
    context.fill_rect(
        0.0,
        0.0,
        f64::from(CANVAS_SIZE.width),
        f64::from(CANVAS_SIZE.height),
    );
    // Draw the ball.
    context.set_fill_style_str(BALL_COLOR);
    context.fill_rect(
        f64::from(((VIRTUAL_SIZE.width - BALL_SIZE) * CANVAS_SCALE) / 2),
        f64::from(((VIRTUAL_SIZE.height - BALL_SIZE) * CANVAS_SCALE) / 2),
        f64::from(BALL_SIZE * CANVAS_SCALE),
        f64::from(BALL_SIZE * CANVAS_SCALE),
    );
    // Draw the left paddle.
    context.fill_rect(
        f64::from(5 * CANVAS_SCALE),
        f64::from(game.paddle_y.0 * CANVAS_SCALE),
        f64::from(5 * CANVAS_SCALE),
        f64::from(PADDLE_HEIGHT * CANVAS_SCALE),
    );
    // Draw the right paddle.
    context.fill_rect(
        f64::from((VIRTUAL_SIZE.width - 10) * CANVAS_SCALE),
        f64::from(game.paddle_y.1 * CANVAS_SCALE),
        f64::from(5 * CANVAS_SCALE),
        f64::from(PADDLE_HEIGHT * CANVAS_SCALE),
    );

    context.stroke();
}

/// Data shown in front of the canvas.
struct Glass {
    score: (Element, Element),
}

impl Glass {
    fn set_score(&self, score: (u16, u16)) {
        self.score.0.set_text_content(Some(&score.0.to_string()));
        self.score.1.set_text_content(Some(&score.1.to_string()));
    }
}

fn keydown_handler(
    system: &System,
    game: Rc<RefCell<Game>>,
    easel: Rc<RefCell<Easel>>,
) -> Closure<dyn Fn(KeyboardEvent)> {
    let handle_keydown = Closure::<dyn Fn(KeyboardEvent)>::new(move |event: KeyboardEvent| {
        match event.key().as_str() {
            " " => easel.borrow_mut().play(),
            _ if easel.borrow().is_paused() => return,
            "1" => game.borrow_mut().player1_score(),
            "2" => game.borrow_mut().increment_player2_score(),
            "s" => game.borrow_mut().player1_start_down(),
            "w" => game.borrow_mut().player1_start_up(),
            "ArrowDown" => game.borrow_mut().player2_start_down(),
            "ArrowUp" => game.borrow_mut().player2_start_up(),
            "Escape" => game.borrow_mut().reset(),
            _ => return,
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
            "s" => game.borrow_mut().player1_stop_down(),
            "w" => game.borrow_mut().player1_stop_up(),
            "ArrowDown" => game.borrow_mut().player2_stop_down(),
            "ArrowUp" => game.borrow_mut().player2_stop_up(),
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
    easel: Rc<RefCell<Easel>>,
    _handle_keydown: Closure<dyn Fn(KeyboardEvent)>,
    _handle_keyup: Closure<dyn Fn(KeyboardEvent)>,
}

impl App {
    /// # Errors
    ///
    /// Will return [`Err`] if DOM interaction fails.
    pub fn new(system: &System) -> Result<Self> {
        let mut generation = 0;
        let game = Game::new();

        let glass = Rc::new(Glass {
            score: (
                SPAN.class("pong-score__span").to_element(system)?,
                SPAN.class("pong-score__span").to_element(system)?,
            ),
        });
        glass.set_score(game.score);

        let game = Rc::new(RefCell::new(game));
        let easel_game = Rc::clone(&game);
        let easel_glass = Rc::clone(&glass);
        let mut easel = Easel::new(system, move |context: RenderContext| {
            let mut game = easel_game.borrow_mut();
            // TODO: Update state.
            game.tick(context.delta_ms);
            // Render the canvas.
            draw(context.canvas, &game);
            // Render the overlay.
            easel_glass.set_score(game.score);
            // Render the caption.
            generation += 1;
            let Size { width, height } = VIRTUAL_SIZE;
            let caption = format!("{width}x{height} @ {generation}");
            context.caption.set_text_content(Some(&caption));
        })?;

        easel.resize_canvas(CANVAS_SIZE);
        easel.play();

        let easel_root = easel.as_ref();
        let root = DIV
            .class("pong")
            .child2(
                easel_root,
                DIV.class("pong-glass").child2(
                    SPAN.class("pong-hello").text("Hello, Pong."),
                    SPAN.class("pong-score")
                        .child2(&glass.score.0, &glass.score.1)
                        .to_element(system)?,
                ),
            )
            .to_element(system)?;

        let easel = Rc::new(RefCell::new(easel));

        Ok(App {
            root,
            _handle_keydown: keydown_handler(system, Rc::clone(&game), Rc::clone(&easel)),
            _handle_keyup: keyup_handler(system, Rc::clone(&game)),
            _game: game,
            easel,
        })
    }

    /// Clicks the play/pause button.
    pub fn play(&mut self) {
        self.easel.borrow_mut().play();
    }
}

impl AsRef<Element> for App {
    fn as_ref(&self) -> &Element {
        &self.root
    }
}
