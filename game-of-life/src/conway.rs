use std::{thread::sleep, time::Duration};

use rand::{rngs::StdRng, Rng, SeedableRng};

pub const DEATH: &'static str = "\x1B[47m  \x1B[0m"; //WHITE
pub const LIFE: &'static str = "\x1B[42m  \x1B[0m"; //GREEN

pub struct Conway {
    seed: u64,
    rng: StdRng,
    population: usize,
    height: usize,
    width: usize,
    tick: usize,
    grid: Vec<bool>,
    next_grid: Vec<bool>,
}

impl Conway {
    pub fn new(seed: u64, population: usize, height: usize, width: usize) -> Self {
        let rng = StdRng::seed_from_u64(seed);
        let size = height * width;
        Self {
            seed,
            rng,
            height,
            width,
            population,
            tick: 0,
            grid: vec![false; size],
            next_grid: vec![false; size],
        }
    }

    pub fn get(&self, row: usize, col: usize) -> Option<&bool> {
        self.grid.get(row * self.width + col)
    }

    pub fn set(&mut self, row: usize, col: usize, b: bool) {
        self.grid[row * self.width + col] = b;
    }

    fn get_next(&self, row: usize, col: usize) -> Option<&bool> {
        self.next_grid.get(row * self.width + col)
    }

    fn set_next(&mut self, row: usize, col: usize, b: bool) {
        self.next_grid[row * self.width + col] = b;
    }

    pub fn seed(&mut self, seed: u64) {
        self.seed = seed;
        self.rng = StdRng::seed_from_u64(seed);
    }

    pub fn random_generation(&mut self) {
        let mut population = self.population;
        let mut row;
        let mut col;
        while population != 0 {
            row = self.rng.gen_range(0..self.height);
            col = self.rng.gen_range(0..self.width);
            if !self.get(row, col).unwrap() {
                self.set(row, col, true);
                population -= 1;
            }
        }
    }

    pub fn neighbour_count(&self, row: usize, col: usize) -> usize {
        let mut count: usize = 0;
        for i in 0..3 {
            for j in 0..3 {
                if i == 1 && j == 1 {
                    continue;
                }

                let new_row = row.wrapping_add(i).wrapping_sub(1);
                let new_col = col.wrapping_add(j).wrapping_sub(1);

                if new_row < self.height && new_col < self.width {
                    if let Some(&true) = self.get(new_row, new_col) {
                        count += 1;
                    }
                }
            }
        }
        count
    }

    pub fn update(&mut self) {
        self.next_grid.fill(false);
        for row in 0..self.height {
            for col in 0..self.width {
                let count = self.neighbour_count(row, col);
                if *self.get(row, col).unwrap() {
                    if count < 2 || count > 3 {
                        self.set_next(row, col, false);
                        self.population -= 1;
                    } else {
                        self.set_next(row, col, true);
                    }
                } else {
                    if count == 3 {
                        self.set_next(row, col, true);
                        self.population += 1;
                    }
                }
            }
        }
        self.tick += 1;
        self.swap();
    }

    fn swap(&mut self) {
        self.grid = self.next_grid.clone();
        //std::mem::swap(&mut self.grid, &mut self.next_grid);
    }

    pub fn render(&self) {
        print!("\x1b[H");
        println!("seed: {}", self.seed);
        println!("tick: {}", self.tick);
        println!("population: {}", self.population);
        for row in 0..self.height {
            for col in 0..self.width {
                match *self.get(row, col).unwrap() {
                    true => print!("{}", LIFE),
                    false => print!("{}", DEATH),
                }
            }
            println!()
        }
    }

    pub fn run(&mut self) {
        print!("\x1b[2J");
        loop {
            self.update();
            self.render();
            sleep(Duration::from_millis(33));
            if self.population == 0 {
                self.render();
                break;
            }
        }
    }
}
