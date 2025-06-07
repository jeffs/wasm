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
//! * [] Disable transparency of the canvas
//!   - Or, use a static image beneath the canvas
//! * [] Try WebGPU
//!   - <https://demyanov.dev/past-and-future-html-canvas-brief-overview-2d-webgl-and-webgpu>
//!   - <https://developer.mozilla.org/en-US/docs/Web/API/WebGPU_API>

use std::{cell::RefCell, fmt, rc::Rc};

use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, Document, Element, HtmlCanvasElement, Window};

use crate::js::prelude::*;
use crate::{Error, Result, System};

const CELL_SIZE: u32 = 12;
const GRID_COLOR: &str = "#111";
const DEAD_COLOR: &str = "#333";
const LIVE_COLOR: &str = "#CCC";

const fn u32_to_usize(value: u32) -> usize {
    const { assert!(size_of::<u32>() <= size_of::<usize>()) }
    value as usize
}

#[derive(Copy, Clone)]
struct RectangleSize {
    width: u32,
    height: u32,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Cell {
    Dead,
    Live,
}

fn prime_factor(mut value: u32) -> Vec<u32> {
    let mut powers = Vec::new();
    let mut factor = 2;
    while factor * factor <= value {
        let mut power = 0;
        while value % factor == 0 {
            power += 1;
            value /= factor;
        }
        powers.push(power);
        factor += 1;
    }
    powers
}

struct Universe {
    size: RectangleSize,
    cells: Vec<Cell>,
    value: u32,
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

    fn tick(&mut self) {
        self.value += 1;
        let powers = prime_factor(self.value);
        let (m, n) = (self.height(), self.width());
        for i in 0..m {
            for j in 0..n {
                let index = self.get_index(i, j);
                let power = powers.get(u32_to_usize(j)).copied().unwrap_or_default();
                self.cells[index] = if power > m - i {
                    Cell::Live
                } else {
                    Cell::Dead
                }
            }
        }
    }

    pub fn with_size(size: RectangleSize) -> Universe {
        let cells = (0..size.width * size.height)
            .map(|i| {
                if i % 2 == 0 || i % 7 == 0 {
                    Cell::Live
                } else {
                    Cell::Dead
                }
            })
            .collect();
        Universe {
            size,
            cells,
            value: 0,
        }
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

fn new_canvas(document: &Document, size: RectangleSize) -> Result<HtmlCanvasElement> {
    let canvas = document
        .create_element("canvas")?
        .dyn_cast::<HtmlCanvasElement>()?;

    canvas.set_class_name("life__canvas");

    // Leave room for a 1px border around each cell.
    canvas.set_width((CELL_SIZE + 1) * size.width + 1);
    canvas.set_height((CELL_SIZE + 1) * size.height + 1);

    Ok(canvas)
}

fn draw_grid(context: &CanvasRenderingContext2d, RectangleSize { width, height }: RectangleSize) {
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

fn request_animation_frame(window: &Window, f: &Closure<dyn FnMut()>) {
    window
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("requesting animation frame");
}

pub struct Chart {
    pub root: Element,
}

impl Chart {
    pub fn new(system: &Rc<System>) -> Result<Self> {
        let title = system.document.create_element("h1")?;
        title.set_class_name("life__title");
        title.set_text_content(Some("Prime factors of 1"));

        let size = RectangleSize {
            width: 64,
            height: 20,
        };

        let canvas = new_canvas(&system.document, size)?;
        let context = canvas
            .get_context("2d")?
            .ok_or(Error::Str("canvas should have 2d context"))?
            .dyn_cast::<CanvasRenderingContext2d>()?;

        let root = system.document.create_element("div")?;
        root.set_class_name("life");
        root.append_with_node_2(&title, &canvas)?;

        let render = Rc::new(RefCell::new(None));
        let raf_cb = Rc::clone(&render);

        let mut universe = Universe::with_size(size);
        let raf_system = Rc::clone(system);
        *raf_cb.borrow_mut() = Some(Closure::<dyn FnMut()>::new(move || {
            universe.tick();
            let value = universe.value;
            title.set_text_content(Some(&format!("Prime factors of {value}")));
            draw_grid(&context, size);
            draw_cells(&context, &universe);
            request_animation_frame(&raf_system.window, render.borrow().as_ref().unwrap());
        }));

        request_animation_frame(&system.window, raf_cb.borrow().as_ref().unwrap());

        Ok(Chart { root })
    }
}
