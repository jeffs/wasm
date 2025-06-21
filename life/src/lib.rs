//! # Thanks
//!
//! * <https://rustwasm.github.io/wasm-bindgen/examples/2d-canvas.html>
//! * <https://rustwasm.github.io/wasm-bindgen/examples/request-animation-frame.html>
//! * <https://rustwasm.github.io/docs/book/game-of-life/implementing.html>
//! * <https://developer.mozilla.org/en-US/docs/Web/API/Canvas_API/Tutorial/Optimizing_canvas>
//!
//! # TODO
//!
//! * [] Buffer grid lines in an off-screen canvas
//!   - <https://web.dev/articles/canvas-performance>
//!   - Or, use a static image beneath the canvas
//! * [] Try WebGPU
//!   - <https://demyanov.dev/past-and-future-html-canvas-brief-overview-2d-webgl-and-webgpu>
//!   - <https://developer.mozilla.org/en-US/docs/Web/API/WebGPU_API>
//! * [] Implement hashlife, and the exercises from the Game of Life tutorial

mod universe;

use easel::{Easel, RenderContext, Result, canvas_size};
use system::{SizeU32, System};
use universe::{Cell, Point, Universe};
use web_sys::{CanvasRenderingContext2d, Element};

const CELL_SIZE: u32 = 2;
const LIVE_COLOR: &str = "hsl(145, 19%, 45%)"; // Dark jade.

fn draw_cells(context: &CanvasRenderingContext2d, universe: &Universe) {
    context.begin_path();
    context.set_fill_style_str(LIVE_COLOR);
    for i in 0..universe.height() {
        for j in 0..universe.width() {
            let Cell::Live = universe.at(Point { i, j }) else {
                continue;
            };
            context.fill_rect(
                (j * CELL_SIZE).into(),
                (i * CELL_SIZE).into(),
                CELL_SIZE.into(),
                CELL_SIZE.into(),
            );
        }
    }
    context.stroke();
}

pub struct App {
    easel: Easel,
}

impl App {
    /// # Errors
    ///
    /// Will return [`Err`] if DOM interaction fails.
    ///
    /// # TODO
    ///
    /// * Decouple state update from rendering.
    pub fn new(system: &System) -> Result<Self> {
        let mut generation = 0;
        let mut universe = Universe::new();
        Ok(App {
            easel: Easel::start(system, move |easel: RenderContext| {
                // Update state.
                let is_new = universe.height() == 0;
                let canvas = easel.canvas;
                let size = canvas.canvas().map(canvas_size).unwrap_or_default();
                universe.resize(SizeU32 {
                    width: size.width / CELL_SIZE,
                    height: size.height / CELL_SIZE,
                });
                if is_new {
                    // Let there be light.
                    universe.speckle();
                } else {
                    universe.tick();
                }
                // Render the canvas.
                canvas.clear_rect(0.0, 0.0, size.width.into(), size.height.into());
                draw_cells(easel.canvas, &universe);
                // Render the caption.
                generation += 1;
                let (width, height) = (universe.width(), universe.height());
                let caption = format!("{width}x{height} @ {generation}");
                easel.caption.set_text_content(Some(&caption));
            })?,
        })
    }
}

impl AsRef<Element> for App {
    fn as_ref(&self) -> &Element {
        self.easel.as_ref()
    }
}
