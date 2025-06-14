//! # Thanks
//!
//! * <https://rustwasm.github.io/wasm-bindgen/examples/2d-canvas.html>
//! * <https://rustwasm.github.io/wasm-bindgen/examples/request-animation-frame.html>
//! * <https://rustwasm.github.io/docs/book/game-of-life/implementing.html>
//! * <https://developer.mozilla.org/en-US/docs/Web/API/Canvas_API/Tutorial/Optimizing_canvas>
//!
//! # TODO
//!
//! * [] Add an FPS counter
//! * [] Buffer grid lines in an off-screen canvas
//!   - <https://web.dev/articles/canvas-performance>
//!   - Or, use a static image beneath the canvas
//! * [] Try WebGPU
//!   - <https://demyanov.dev/past-and-future-html-canvas-brief-overview-2d-webgl-and-webgpu>
//!   - <https://developer.mozilla.org/en-US/docs/Web/API/WebGPU_API>
//! * [] Make a game: <https://www.youtube.com/watch?v=c-1dBd1_G8A>
//! * [] Implement hashlife, and the exercises from the Game of Life tutorial

#![allow(dead_code)]

use std::{fmt, rc::Rc};

use easel::Result;
use system::{Size, u32_to_usize};
use web_sys::{CanvasRenderingContext2d, Element};

const CELL_SIZE: u32 = 5;
const GRID_COLOR: &str = "#CCCCCC";
const DEAD_COLOR: &str = "#FFFFFF";
const LIVE_COLOR: &str = "#000000";

#[derive(Clone, Copy, PartialEq, Eq)]
enum Cell {
    Dead,
    Live,
}

impl Cell {
    fn to_u8(self) -> u8 {
        match self {
            Cell::Dead => 0,
            Cell::Live => 1,
        }
    }
}

struct Universe {
    size: Size,
    cells: Vec<Cell>,
}

impl Universe {
    fn width(&self) -> u32 {
        self.size.width
    }

    fn height(&self) -> u32 {
        self.size.height
    }

    fn get_index(&self, row: u32, column: u32) -> usize {
        u32_to_usize(row * self.width() + column)
    }

    fn at(&self, row: u32, column: u32) -> Cell {
        self.cells[self.get_index(row, column)]
    }

    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;
        for delta_row in [self.height() - 1, 0, 1] {
            for delta_col in [self.width() - 1, 0, 1] {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }
                let neighbor_row = (row + delta_row) % self.height();
                let neighbor_col = (column + delta_col) % self.width();
                count += self.at(neighbor_row, neighbor_col).to_u8();
            }
        }
        count
    }

    fn tick(&mut self) {
        let mut next = self.cells.clone();

        for row in 0..self.height() {
            for col in 0..self.width() {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbors = self.live_neighbor_count(row, col);

                let next_cell = match (cell, live_neighbors) {
                    // Rule 1: Any live cell with fewer than two live neighbours
                    // dies, as if caused by underpopulation.
                    //
                    // Rule 3: Any live cell with more than three live
                    // neighbours dies, as if by overpopulation.
                    (Cell::Live, x) if !(2..=3).contains(&x) => Cell::Dead,
                    // Rule 2: Any live cell with two or three live neighbours
                    // lives on to the next generation.
                    //
                    // Rule 4: Any dead cell with exactly three live neighbours
                    // becomes a live cell, as if by reproduction.
                    (Cell::Live, _) | (Cell::Dead, 3) => Cell::Live,
                    // All other cells remain in the same state.
                    _ => cell,
                };

                next[idx] = next_cell;
            }
        }

        self.cells = next;
    }

    pub fn with_size(size: Size) -> Universe {
        let cells = (0..size.width * size.height)
            .map(|i| {
                if i % 2 == 0 || i % 7 == 0 {
                    Cell::Live
                } else {
                    Cell::Dead
                }
            })
            .collect();
        Universe { size, cells }
    }
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(u32_to_usize(self.width())) {
            for &cell in line {
                let symbol = if cell == Cell::Dead { ' ' } else { '\u{2588}' };
                write!(f, "{symbol}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn draw_grid(context: &CanvasRenderingContext2d, Size { width, height }: Size) {
    context.begin_path();
    context.set_stroke_style_str(GRID_COLOR);

    // Vertical lines.
    for i in 0..=width {
        let x = (i * (CELL_SIZE + 1) + 1).into();
        context.move_to(x, 0.0);
        context.line_to(x, ((CELL_SIZE + 1) * height + 1).into());
    }

    // Horizontal lines.
    for j in 0..=width {
        let y = (j * (CELL_SIZE + 1) + 1).into();
        context.move_to(0.0, y);
        context.line_to(((CELL_SIZE + 1) * width + 1).into(), y);
    }

    context.stroke();
}

/// TODO: Draw all the dead cells, then the live cells, to avoid gratuitous
///  state changes in the rendering context. See the canvas performance article
///  linked in the module docs.
fn draw_cells(context: &CanvasRenderingContext2d, universe: &Universe) {
    context.begin_path();
    for row in 0..universe.height() {
        for column in 0..universe.width() {
            let cell = universe.at(row, column);
            context.set_fill_style_str(match cell {
                Cell::Dead => DEAD_COLOR,
                Cell::Live => LIVE_COLOR,
            });
            context.fill_rect(
                (column * (CELL_SIZE + 1) + 1).into(),
                (row * (CELL_SIZE + 1) + 1).into(),
                CELL_SIZE.into(),
                CELL_SIZE.into(),
            );
        }
    }
    context.stroke();
}

pub struct App {
    root: Element,
}

impl App {
    /// # Errors
    ///
    /// Will return [`Err`] if DOM interaction fails.
    #[allow(clippy::needless_pass_by_value)]
    pub fn new(_system: Rc<system::System>) -> Result<Self> {
        // let size = Size {
        //     width: 64,
        //     height: 64,
        // };
        // let mut universe = Universe::with_size(size);

        // draw_grid(&context, size);
        // draw_cells(&context, &universe);

        todo!()
    }

    #[must_use]
    pub fn root(&self) -> &Element {
        &self.root
    }
}
