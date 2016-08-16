use rand;


#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Cell {
    pub x: usize,
    pub y: usize,
    pub alive: bool,
}

#[derive(Clone, Copy, Debug)]
struct InternalCell {
    alive: bool,
    neighbor_count: u8,
}

#[derive(Clone)]
pub struct Grid {
    width: usize,
    height: usize,
    inner: Vec<InternalCell>,
}

pub struct LifeBoard {
    pub grid: Grid,
    old_grid: Grid,
}

struct CellIterator<I>
    where I: Iterator {
    width: usize,
    height: usize,
    inner: I,
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

impl Grid {
    pub fn new(width: usize, height: usize) -> Self {
        let inner = vec![InternalCell { alive: false, neighbor_count: 0 }; width * height];

        Grid {
            width: width,
            height: height,
            inner: inner,
        }
    }

    pub fn check_point(&self, x: usize, y: usize) -> bool {
        x < self.width && y < self.height
    }

    // TODO: Return an iterator??
    fn get_neighbors(&self, x: usize, y: usize) -> ([Cell; 8], usize) {
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
        (neighbors, next_neighbor)
    }

    fn update_neighbors(&mut self, x: usize, y: usize, value: bool) {
        let (neighbors, count) = self.get_neighbors(x, y);
        let amount: i8 = if value { 1 } else { -1 };
        for i in 0..count {
            let neighbor = neighbors[i];
            let index = (neighbor.y * self.width) + neighbor.x;
            self.inner[index].neighbor_count = (self.inner[index].neighbor_count as i8 + amount) as u8;
        }
    }

    fn get_internal(&self, x: usize, y: usize) -> InternalCell {
        let index = (y * self.width) + x;
        self.inner[index]
    }

    pub fn get(&self, x: usize, y: usize) -> bool {
        let index = (y * self.width) + x;
        self.inner[index].alive
    }

    pub fn set(&mut self, x: usize, y: usize, value: bool) {
        // Get current value, if changing, update neighbors
        let index = (y * self.width) + x;
        //println!("(SET) x: {}, y: {}, index: {}", x, y, index);
        if value != self.inner[index].alive {
            self.inner[index].alive = value;
            self.update_neighbors(x, y, value);
        }
    }

    pub fn clear(&mut self) {
        self.inner = vec![InternalCell { alive: false, neighbor_count: 0 }; self.width * self.height];
    }

    pub fn randomize(&mut self) {
        for i in 0..self.inner.len() {
            let x = i % self.width;
            let y = i / self.width;
            self.set(x, y, rand::random());
        }
    }
}

impl LifeBoard {
    pub fn new(width: usize, height: usize) -> LifeBoard {
        LifeBoard {
            grid: Grid::new(width, height),
            old_grid: Grid::new(width, height),
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

    pub fn set(&mut self, x: usize, y: usize, value: bool) {
        self.grid.set(x, y, value);
    }

    pub fn get(&mut self, x: usize, y: usize) -> bool {
        self.grid.get(x, y)
    }

    pub fn step(&mut self) {
        self.old_grid = self.grid.clone();

        // TODO: Iterate over cells that are alive or neighbor_count > 0

        for j in 0..self.height() {
            for i in 0..self.width() {
                let cell = self.old_grid.get_internal(i, j);
                if cell.alive {
                    self.grid.set(
                        i, j, cell.neighbor_count >= 2 && cell.neighbor_count <= 3);
                } else {
                    self.grid.set(i, j, cell.neighbor_count == 3);
                }
            }
        }
    }

    pub fn iter_live_cells<'a>(&'a self) -> impl Iterator<Item=Cell> + 'a {
        let inner = self.grid.inner.iter().enumerate().filter(|&(_, cell)| {
            cell.alive
        });

        CellIterator {
            width: self.grid.width,
            height: self.grid.height,
            inner: inner,
        }
    }
}

impl<'a, I> Iterator for CellIterator<I>
    where I: Iterator<Item=(usize, &'a InternalCell)> {
    type Item = Cell;

    fn next(&mut self) -> Option<Cell> {
        if let Some((i, _)) = self.inner.next() {
            Some(Cell {
                x: i % self.width,
                y: i / self.width,
                alive: true,
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

    #[test]
    fn get_set() {
        let mut board = LifeBoard::new(3, 3);

        board.set(1, 1, true);

        assert_eq!(board.get(1, 1), true);
    }

    #[test]
    fn update_neighbors_increment() {
        let mut board = LifeBoard::new(3, 3);

        board.set(1, 1, true);

        assert_eq!(board.grid.get_internal(0, 0).neighbor_count, 1);
        assert_eq!(board.grid.get_internal(1, 0).neighbor_count, 1);
        assert_eq!(board.grid.get_internal(2, 0).neighbor_count, 1);
        assert_eq!(board.grid.get_internal(0, 1).neighbor_count, 1);
        assert_eq!(board.grid.get_internal(1, 1).neighbor_count, 0);
        assert_eq!(board.grid.get_internal(2, 1).neighbor_count, 1);
        assert_eq!(board.grid.get_internal(0, 2).neighbor_count, 1);
        assert_eq!(board.grid.get_internal(1, 2).neighbor_count, 1);
        assert_eq!(board.grid.get_internal(2, 2).neighbor_count, 1);
    }

    #[test]
    fn update_neighbors_increment_corner() {
        let mut board = LifeBoard::new(3, 3);

        board.set(0, 0, true);

        assert_eq!(board.grid.get_internal(0, 0).neighbor_count, 0);
        assert_eq!(board.grid.get_internal(1, 0).neighbor_count, 1);
        assert_eq!(board.grid.get_internal(2, 0).neighbor_count, 0);
        assert_eq!(board.grid.get_internal(0, 1).neighbor_count, 1);
        assert_eq!(board.grid.get_internal(1, 1).neighbor_count, 1);
        assert_eq!(board.grid.get_internal(2, 1).neighbor_count, 0);
        assert_eq!(board.grid.get_internal(0, 2).neighbor_count, 0);
        assert_eq!(board.grid.get_internal(1, 2).neighbor_count, 0);
        assert_eq!(board.grid.get_internal(2, 2).neighbor_count, 0);
    }

    #[test]
    fn update_neighbors_increment_decrement() {
        let mut board = LifeBoard::new(3, 3);

        board.set(1, 1, true);

        assert_eq!(board.grid.get_internal(0, 0).neighbor_count, 1);
        assert_eq!(board.grid.get_internal(1, 0).neighbor_count, 1);
        assert_eq!(board.grid.get_internal(2, 0).neighbor_count, 1);
        assert_eq!(board.grid.get_internal(0, 1).neighbor_count, 1);
        assert_eq!(board.grid.get_internal(1, 1).neighbor_count, 0);
        assert_eq!(board.grid.get_internal(2, 1).neighbor_count, 1);
        assert_eq!(board.grid.get_internal(0, 2).neighbor_count, 1);
        assert_eq!(board.grid.get_internal(1, 2).neighbor_count, 1);
        assert_eq!(board.grid.get_internal(2, 2).neighbor_count, 1);

        board.set(1, 1, false);

        assert_eq!(board.grid.get_internal(0, 0).neighbor_count, 0);
        assert_eq!(board.grid.get_internal(1, 0).neighbor_count, 0);
        assert_eq!(board.grid.get_internal(2, 0).neighbor_count, 0);
        assert_eq!(board.grid.get_internal(0, 1).neighbor_count, 0);
        assert_eq!(board.grid.get_internal(1, 1).neighbor_count, 0);
        assert_eq!(board.grid.get_internal(2, 1).neighbor_count, 0);
        assert_eq!(board.grid.get_internal(0, 2).neighbor_count, 0);
        assert_eq!(board.grid.get_internal(1, 2).neighbor_count, 0);
        assert_eq!(board.grid.get_internal(2, 2).neighbor_count, 0);
    }

    #[test]
    fn iter_live_cells() {
        let mut board = LifeBoard::new(3, 3);

        board.set(1, 1, true);

        let mut board_iter = board.iter_live_cells();

        assert_eq!(board_iter.next(), Some(Cell {
            x: 1,
            y: 1,
            alive: true,
        }));
    }

    #[bench]
    fn bench_10x10_blank_step_once(b: &mut Bencher) {
        let mut board = LifeBoard::new(10, 10);

        b.iter(|| board.step());
    }

    #[bench]
    fn bench_100x100_blank_step_once(b: &mut Bencher) {
        let mut board = LifeBoard::new(100, 100);

        b.iter(|| board.step());
    }

    #[bench]
    fn bench_1000x1000_blank_step_once(b: &mut Bencher) {
        let mut board = LifeBoard::new(1000, 1000);

        b.iter(|| board.step());
    }

    #[bench]
    fn bench_100x100_glider_step_100(b: &mut Bencher) {
        let mut board = LifeBoard::new(100, 100);
        board.set(0, 0, true);
        board.set(1, 0, true);
        board.set(2, 0, true);
        board.set(2, 1, true);
        board.set(1, 2, true);

        b.iter(|| {
            for _ in 0..100 {
                board.step();
            }
        });
    }

    #[bench]
    fn bench_100x100_acorn_step_100(b: &mut Bencher) {
        let mut board = LifeBoard::new(100, 100);
        board.set(0, 0, true);
        board.set(1, 0, true);
        board.set(1, 2, true);
        board.set(3, 1, true);
        board.set(4, 0, true);
        board.set(5, 0, true);
        board.set(6, 0, true);

        b.iter(|| {
            for _ in 0..100 {
                board.step();
            }
        });
    }

    // TODO: Glider gun benchmark?

    //#[bench]
    //fn bench_10000x10000_blank_step_once(b: &mut Bencher) {
    //    let mut board = LifeBoard::new(10000, 10000);

    //    b.iter(|| board.step());
    //}
}
