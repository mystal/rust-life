use bit_vec::BitVec;
use rand;

#[derive(Clone)]
pub struct BoolGrid {
    pub width: usize,
    pub height: usize,
    pub grid: Vec<BitVec>,
}

struct Cell {
    x: usize,
    y: usize,
    alive: bool,
}

pub struct LifeBoard {
    pub grid: BoolGrid,
    old_grid: BoolGrid,
}

impl BoolGrid {
    pub fn new(width: usize, height: usize) -> BoolGrid {
        let mut grid = Vec::with_capacity(height as usize);
        for _ in 0..height {
            grid.push(BitVec::from_elem(width as usize, false));
        }

        BoolGrid {
            width: width,
            height: height,
            grid: grid,
        }
    }

    pub fn check_point(&self, x: i32, y: i32) -> bool {
        x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32
    }

    pub fn get_neighbors(&self, x: usize, y: usize) -> Vec<Cell> {
        let x = x as i32;
        let y = y as i32;
        let mut neighbors = vec![];
        for j in (y - 1)..(y + 2) {
            for i in (x - 1)..(x + 2) {
                if (x != i || y != j) && self.check_point(i, j) {
                    let i = i as usize;
                    let j = j as usize;
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

    pub fn get(&self, x: usize, y: usize) -> bool {
        self.grid[y][x]
    }

    pub fn set(&mut self, x: usize, y: usize, value: bool) {
        self.grid[y].set(x, value);
    }

    pub fn clear(&mut self) {
        for bv in self.grid.iter_mut() {
            bv.clear();
        }
    }

    pub fn randomize(&mut self) {
        for bv in self.grid.iter_mut() {
            for i in 0..bv.len() {
                bv.set(i, rand::random());
            }
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

    pub fn clear(&mut self) {
        self.grid.clear();
    }

    pub fn randomize(&mut self) {
        self.grid.randomize();
    }

    pub fn step(&mut self) {
        self.old_grid = self.grid.clone();

        for j in 0..self.grid.height {
            for i in 0..self.grid.width {
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
