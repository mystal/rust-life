use std::collections::Bitv;
use std::rand;

#[deriving(Clone)]
pub struct BoolGrid {
    pub width: uint,
    pub height: uint,
    pub grid: Vec<Bitv>,
}

struct Cell {
    x: uint,
    y: uint,
    alive: bool,
}

pub struct LifeBoard {
    pub grid: BoolGrid,
    old_grid: BoolGrid,
}

impl BoolGrid {
    pub fn new(width: uint, height: uint) -> BoolGrid {
        let mut grid = Vec::with_capacity(height);
        for _ in range(0, height) {
            grid.push(Bitv::with_capacity(width, false));
        }

        BoolGrid {
            width: width,
            height: height,
            grid: grid,
        }
    }

    pub fn check_point(&self, x: uint, y: uint) -> bool {
        x < self.width && y < self.height
    }

    pub fn get_neighbors(&self, x: uint, y: uint) -> Vec<Cell> {
        let mut neighbors = vec![];
        for j in range(y - 1, y + 2) {
            for i in range(x - 1, x + 2) {
                if (x != i || y != j) && self.check_point(i, j) {
                    neighbors.push(Cell {
                        x: i,
                        y: j,
                        alive: self.grid[j][i],
                    });
                }
            }
        }
        neighbors
    }

    pub fn get(&self, x: uint, y: uint) -> bool {
        self.grid[y][x]
    }

    pub fn set(&mut self, x: uint, y: uint, value: bool) {
        self.grid[y].set(x, value);
    }

    pub fn clear(&mut self) {
        for bv in self.grid.iter_mut() {
            bv.clear();
        }
    }

    pub fn randomize(&mut self) {
        for bv in self.grid.iter_mut() {
            for i in range(0, bv.len()) {
                bv.set(i, rand::random());
            }
        }
    }
}

impl LifeBoard {
    pub fn new(width: uint, height: uint) -> LifeBoard {
        LifeBoard {
            grid: BoolGrid::new(width, height),
            old_grid: BoolGrid::new(width, height),
        }
    }

    pub fn clear(&mut self) {
        self.grid.clear();
    }

    pub fn randomize(&mut self) {
        self.grid.randomize();
    }

    pub fn step(&mut self) {
        self.old_grid = self.grid.clone();

        for j in range(0, self.grid.height) {
            for i in range(0, self.grid.width) {
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
}
