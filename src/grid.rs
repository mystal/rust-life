use bit_vec::{self, BitVec};
use rand;

use std::iter::Enumerate;


#[derive(Clone, Copy)]
pub struct Cell {
    pub x: usize,
    pub y: usize,
    pub alive: bool,
}

#[derive(Clone)]
pub struct BoolGrid {
    width: usize,
    height: usize,
    grid: BitVec,
}

pub struct LifeBoard {
    pub grid: BoolGrid,
    old_grid: BoolGrid,
}

struct CellIterator<'a> {
    width: usize,
    height: usize,
    inner: Enumerate<bit_vec::Iter<'a>>,
}


impl Cell {
    fn new() -> Self {
        Cell {
            x: 0,
            y: 0,
            alive: false,
        }
    }
}

impl BoolGrid {
    pub fn new(width: usize, height: usize) -> Self {
        let grid = BitVec::from_elem(width * height, false);

        BoolGrid {
            width: width,
            height: height,
            grid: grid,
        }
    }

    pub fn check_point(&self, x: usize, y: usize) -> bool {
        x < self.width && y < self.height
    }

    fn get_neighbors(&self, x: usize, y: usize) -> [Cell; 8] {
        let mut neighbors = [Cell::new(); 8];
        let mut next_neighbor = 0;

        let x_start = if x == 0 { x } else { x - 1 };
        let y_start = if y == 0 { y } else { y - 1 };

        for j in y_start..(y + 2) {
            for i in x_start..(x + 2) {
                if (x != i || y != j) && self.check_point(i, j) {
                    neighbors[next_neighbor] = Cell {
                        x: i,
                        y: j,
                        alive: self.get(i, j),
                    };
                    next_neighbor += 1;
                }
            }
        }
        neighbors
    }

    pub fn get(&self, x: usize, y: usize) -> bool {
        let index = (y * self.width) + x;
        self.grid[index]
    }

    pub fn set(&mut self, x: usize, y: usize, value: bool) {
        let index = (y * self.width) + x;
        self.grid.set(index, value);
    }

    pub fn clear(&mut self) {
        self.grid.clear();
    }

    pub fn randomize(&mut self) {
        for i in 0..self.grid.len() {
            self.grid.set(i, rand::random());
        }
    }
}

impl LifeBoard {
    pub fn new(width: usize, height: usize) -> LifeBoard {
        LifeBoard {
            grid: BoolGrid::new(width, height),
            old_grid: BoolGrid::new(width, height),
        }
    }

    pub fn width(&self) -> usize {
        self.grid.width
    }

    pub fn height(&self) -> usize {
        self.grid.height
    }

    pub fn clear(&mut self) {
        self.grid.clear();
    }

    pub fn randomize(&mut self) {
        self.grid.randomize();
    }

    pub fn step(&mut self) {
        self.old_grid = self.grid.clone();

        for j in 0..self.height() {
            for i in 0..self.width() {
                let neighbor_count = self.old_grid.get_neighbors(i, j).iter()
                    .filter(|&cell| cell.alive)
                    .count();
                if self.grid.get(i, j) {
                    self.grid.set(
                        i, j, neighbor_count >= 2 && neighbor_count <= 3);
                } else {
                    self.grid.set(i, j, neighbor_count == 3);
                }
            }
        }
    }

    pub fn iter_cells<'a>(&'a self) -> impl Iterator<Item=Cell> + 'a {
        CellIterator {
            width: self.grid.width,
            height: self.grid.height,
            inner: self.grid.grid.iter().enumerate(),
        }
    }
}

impl<'a> Iterator for CellIterator<'a> {
    type Item = Cell;

    fn next(&mut self) -> Option<Cell> {
        if let Some((i, alive)) = self.inner.next() {
            Some(Cell {
                x: i % self.width,
                y: i / self.height,
                alive: alive,
            })
        } else {
            None
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_1000x1000_blank_step_once(b: &mut Bencher) {
        let mut board = LifeBoard::new(1000, 1000);

        b.iter(|| board.step());
    }
}
