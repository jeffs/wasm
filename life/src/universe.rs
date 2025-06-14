use std::mem;

use system::{Size, u32_to_usize, usize_to_u32};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Cell {
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

/// Zero-based row and column indexes.
#[derive(Clone, Copy)]
pub struct Point {
    pub i: u32,
    pub j: u32,
}

#[derive(Clone)]
pub struct Universe {
    /// Rectangular grid.
    rows: Vec<Vec<Cell>>,
}

impl Universe {
    pub fn new() -> Universe {
        Universe { rows: Vec::new() }
    }

    pub fn resize(&mut self, size: Size) {
        self.rows.resize_with(u32_to_usize(size.height), Vec::new);
        for row in &mut self.rows {
            row.resize(u32_to_usize(size.width), Cell::Dead);
        }
    }

    pub fn height(&self) -> u32 {
        usize_to_u32(self.rows.len())
    }

    pub fn width(&self) -> u32 {
        self.rows
            .first()
            .map(Vec::len)
            .map(usize_to_u32)
            .unwrap_or_default()
    }

    pub fn at(&self, p: Point) -> Cell {
        self.rows[u32_to_usize(p.i)][u32_to_usize(p.j)]
    }

    pub fn set(&mut self, i: u32, j: u32, c: Cell) {
        self.rows[u32_to_usize(i)][u32_to_usize(j)] = c;
    }

    fn live_neighbor_count(&self, p: Point) -> u8 {
        let mut count = 0;
        let [h, w] = [self.height(), self.width()];
        for di in [h - 1, 0, 1] {
            for dj in [w - 1, 0, 1] {
                if di == 0 && dj == 0 {
                    continue;
                }
                let q = Point {
                    i: (p.i + di) % h,
                    j: (p.j + dj) % w,
                };
                count += self.at(q).to_u8();
            }
        }
        count
    }

    pub fn tick(&mut self) {
        // TODO: Cache second buffer in self.
        let mut next = self.clone();

        for i in 0..self.height() {
            for j in 0..self.width() {
                let p = Point { i, j };
                let c = self.at(p);
                let n = self.live_neighbor_count(p);
                let d = match (c, n) {
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
                    _ => c,
                };
                next.set(i, j, d);
            }
        }

        mem::swap(self, &mut next);
    }

    pub fn speckle(&mut self) {
        for i in 0..self.height() {
            for j in 0..self.width() {
                let k = i * self.width() + j;
                let c = if k % 2 == 0 || k % 7 == 0 {
                    Cell::Live
                } else {
                    Cell::Dead
                };
                self.set(i, j, c);
            }
        }
    }
}
