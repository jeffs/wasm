use web_sys::CanvasRenderingContext2d;

use math::LinearCongruentialGenerator;
use system::SizeF64;

use crate::{
    ball::{self, Ball},
    constants::CANVAS_SIZE,
    paddle::{self, Paddle},
    physics::{Direction, PointF64, VIRTUAL_SIZE, Vec2d},
    state::State,
};

/// Initial gap between paddles and court edges.
const PADDING: SizeF64 = paddle::SIZE;

pub const COURT_COLOR: &str = "rgb(40, 45, 52)";
pub const BALL_COLOR: &str = layout::color::IVORY;

pub struct Game {
    state: State,
    score: (u16, u16),
    paddles: (Paddle, Paddle),
    ball: Ball,
    _random: LinearCongruentialGenerator,
}

impl Game {
    pub fn from_seed(seed: u32) -> Self {
        let mut random = LinearCongruentialGenerator::from_seed(seed);
        Game {
            state: State::Start,
            score: (0, 0),
            paddles: (
                Paddle::new(PointF64 {
                    x: PADDING.width,
                    y: PADDING.height,
                }),
                Paddle::new(PointF64 {
                    x: VIRTUAL_SIZE.width - PADDING.width - paddle::SIZE.width,
                    y: VIRTUAL_SIZE.height - PADDING.height - paddle::SIZE.height,
                }),
            ),
            ball: Ball::new(
                PointF64 {
                    x: (VIRTUAL_SIZE.width - ball::SIZE) / 2.0,
                    y: (VIRTUAL_SIZE.height - ball::SIZE) / 2.0,
                },
                Vec2d {
                    // TODO: Factor out constants.
                    dx: if random.next_bool() { 100.0 } else { -100.0 },
                    dy: (random.next_i32() % 50).into(),
                },
            ),
            _random: random,
        }
    }

    /// Returns true on success, and false if the game was already in play.
    pub fn start(&mut self) -> bool {
        std::mem::replace(&mut self.state, State::Play) == State::Start
    }

    #[expect(clippy::unused_self)]
    pub fn size(&self) -> SizeF64 {
        // This my become a run-time value at some point.
        VIRTUAL_SIZE
    }

    pub fn state(&self) -> &State {
        &self.state
    }

    pub fn score(&self) -> (u16, u16) {
        self.score
    }

    pub fn player1_move(&mut self, direction: Option<Direction>) {
        self.paddles.0.set_direction(direction);
    }

    pub fn player2_move(&mut self, direction: Option<Direction>) {
        self.paddles.1.set_direction(direction);
    }

    pub fn player1_score(&mut self) {
        self.score.0 += 1;
    }

    pub fn player2_score(&mut self) {
        self.score.1 += 1;
    }

    /// Updates the state of this game, according to the specified amount of
    /// elapsed time since the previous call (if any).
    pub fn update(&mut self, dt: Option<f64>) {
        let Some(dt) = dt else {
            return;
        };
        self.paddles.0.update(dt);
        self.paddles.1.update(dt);
        if let State::Play = self.state {
            self.ball.update(dt);
        }
    }

    pub fn render(&self, context: &CanvasRenderingContext2d) {
        context.begin_path();
        context.set_fill_style_str(COURT_COLOR);
        // Draw the background.
        context.fill_rect(
            0.0,
            0.0,
            f64::from(CANVAS_SIZE.width),
            f64::from(CANVAS_SIZE.height),
        );
        // Draw the ball and paddles using a single color.
        context.set_fill_style_str(BALL_COLOR);
        self.ball.render(context);
        self.paddles.0.render(context);
        self.paddles.1.render(context);
        context.stroke();
    }
}
