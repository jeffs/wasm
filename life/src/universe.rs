use std::mem;

use system::{SizeU32, u32_to_usize, usize_to_u32};

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

fn at(rows: &[Vec<Cell>], p: Point) -> Cell {
    rows[u32_to_usize(p.i)][u32_to_usize(p.j)]
}

fn count_live_neighbors(rows: &[Vec<Cell>], size: SizeU32, p: Point) -> u8 {
    let mut count = 0;
    let SizeU32 {
        height: h,
        width: w,
    } = size;
    for di in [h - 1, 0, 1] {
        for dj in [w - 1, 0, 1] {
            if di == 0 && dj == 0 {
                continue;
            }
            let q = Point {
                i: (p.i + di) % h,
                j: (p.j + dj) % w,
            };
            count += at(rows, q).to_u8();
        }
    }
    count
}

#[derive(Clone)]
pub struct Universe {
    /// Rectangular grid.
    rows: Vec<Vec<Cell>>,
    // Cache to avoid reallocations.
    last: Vec<Vec<Cell>>,
}

impl Universe {
    pub fn new() -> Universe {
        Universe {
            rows: Vec::new(),
            last: Vec::new(),
        }
    }

    pub fn resize(&mut self, size: SizeU32) {
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

    fn size(&self) -> SizeU32 {
        let height = self.height();
        let width = self.width();
        SizeU32 { height, width }
    }

    pub fn at(&self, p: Point) -> Cell {
        self.rows[u32_to_usize(p.i)][u32_to_usize(p.j)]
    }

    pub fn set(&mut self, i: u32, j: u32, c: Cell) {
        self.rows[u32_to_usize(i)][u32_to_usize(j)] = c;
    }

    pub fn tick(&mut self) {
        let size = self.size();
        mem::swap(&mut self.rows, &mut self.last);
        self.resize(size);
        for i in 0..size.height {
            for j in 0..size.width {
                let p = Point { i, j };
                let c = at(&self.last, p);
                let n = count_live_neighbors(&self.last, size, p);
                let d = match (c, n) {
                    (Cell::Live, x) if !(2..=3).contains(&x) => Cell::Dead,
                    (Cell::Live, _) | (Cell::Dead, 3) => Cell::Live,
                    _ => c,
                };
                self.set(i, j, d);
            }
        }
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
