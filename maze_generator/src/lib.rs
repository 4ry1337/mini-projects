use std::fmt::Display;

use rand::{thread_rng, Rng};

pub const OBSTICLE: &'static str = "\x1B[40m  \x1B[0m"; //block
pub const PATH: &'static str = "\x1B[47m  \x1B[0m"; //white
pub const START: &'static str = "\x1B[42m  \x1B[0m"; //GREEN
pub const DESTINATION: &'static str = "\x1B[41m  \x1B[0m"; //WHITE

#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Point: x: {}, y: {}", self.x, self.y)
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
    fn ne(&self, other: &Self) -> bool {
        self.x != other.x && self.y != other.y
    }
}

impl Eq for Point {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell {
    pub point: Point,
    pub obsticle: bool,
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Cell: point: {}, obsticle: {}",
            self.point, self.obsticle
        )
    }
}

#[derive(Debug)]
pub struct Maze {
    pub height: usize,
    pub width: usize,
    pub start: Point,
    pub destanation: Point,
    pub grid: Vec<Vec<Cell>>,
}

impl Maze {
    pub fn generate(height: usize, width: usize) -> Self {
        fn mark(x: usize, y: usize, grid: &mut Vec<Vec<Cell>>, frontiers: &mut Vec<Cell>) {
            grid[y][x].obsticle = false;

            if y >= 2 && !frontiers.contains(&grid[y - 2][x]) && grid[y - 2][x].obsticle == true {
                let frontier = grid[y - 2][x];
                // println!("North frontier: {}", frontier);
                frontiers.push(frontier);
            }
            if y + 2 < grid.len()
                && !frontiers.contains(&grid[y + 2][x])
                && grid[y + 2][x].obsticle == true
            {
                let frontier = grid[y + 2][x];
                // println!("South frontier: {}", frontier);
                frontiers.push(frontier);
            }
            if x >= 2 && !frontiers.contains(&grid[y][x - 2]) && grid[y][x - 2].obsticle == true {
                let frontier = grid[y][x - 2];
                // println!("West frontier: {}", frontier);
                frontiers.push(frontier);
            }
            if x + 2 < grid[0].len()
                && !frontiers.contains(&grid[y][x + 2])
                && grid[y][x + 2].obsticle == true
            {
                let frontier = grid[y][x + 2];
                // println!("East frontier: {}", frontier);
                frontiers.push(frontier);
            }
        }

        let mut grid = vec![];

        for y in 0..height {
            let mut row = vec![];
            for x in 0..width {
                row.push(Cell {
                    point: Point { x, y },
                    obsticle: true,
                })
            }
            grid.push(row);
        }

        //TODO add random start and destanation

        //TODO generation Prim's Algorithm
        let mut rng = thread_rng();

        let x = rng.gen_range(0..width);
        let y = rng.gen_range(0..height);

        let start_cell = grid[y][x];

        let mut frontiers: Vec<Cell> = vec![];

        mark(x, y, &mut grid, &mut frontiers);

        //  0
        //3 x 1
        //  2

        while frontiers.len() > 0 {
            let mut possible_to_crave = false;

            let cell_index = rng.gen_range(0..frontiers.len());

            let x = frontiers[cell_index].point.x;
            let y = frontiers[cell_index].point.y;

            frontiers.remove(cell_index);

            while !possible_to_crave {
                let direction = rng.gen_range(0..4);
                if direction == 0 {
                    if y >= 2 && grid[y - 2][x].obsticle == false {
                        possible_to_crave = true;
                        grid[y - 1][x].obsticle = false;
                    }
                } else if direction == 1 {
                    if x + 2 < width && grid[y][x + 2].obsticle == false {
                        possible_to_crave = true;
                        grid[y][x + 1].obsticle = false;
                    }
                } else if direction == 2 {
                    if y + 2 < height && grid[y + 2][x].obsticle == false {
                        possible_to_crave = true;
                        grid[y + 1][x].obsticle = false
                    }
                } else {
                    if x >= 2 && grid[y][x - 2].obsticle == false {
                        possible_to_crave = true;
                        grid[y][x - 1].obsticle = false;
                    }
                }
            }

            mark(x, y, &mut grid, &mut frontiers);
        }
        let mut x = rng.gen_range(0..width);
        let mut y = rng.gen_range(0..height);

        let mut destanation_cell = grid[y][x];

        while destanation_cell.point == start_cell.point || destanation_cell.obsticle == true {
            x = rng.gen_range(0..width);
            y = rng.gen_range(0..height);
            destanation_cell = grid[y][x];
        }

        Self {
            height,
            width,
            grid,
            start: start_cell.point,
            destanation: destanation_cell.point,
        }
    }

    pub fn set_start(&mut self, x: usize, y: usize) -> Result<Point, String> {
        let point = Point { x, y };

        if self.grid[y][x].obsticle == true {
            return Err(format!("Can't place path on wall at {}", point));
        }

        if self.destanation == point {
            return Err(format!("Can't place path on destanation at {}", point));
        }

        self.start = point;

        Ok(self.start)
    }

    pub fn set_destanation(&mut self, x: usize, y: usize) -> Result<Point, String> {
        let point = Point { x, y };

        if self.grid[y][x].obsticle == true {
            return Err(format!("Can't place path on wall at {}", point));
        }

        if self.start == point {
            return Err(format!("Can't place path on start at {}", point));
        }

        self.destanation = point;

        Ok(self.destanation)
    }

    pub fn draw(&self) {
        for row in self.grid.iter() {
            for cell in row.iter() {
                if cell.obsticle {
                    print!("{}", OBSTICLE);
                } else {
                    if cell.point == self.start {
                        print!("{}", START);
                    } else if cell.point == self.destanation {
                        print!("{}", DESTINATION);
                    } else {
                        print!("{}", PATH);
                    }
                }
            }
            println!();
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use crate::Maze;
//
//     #[test]
//     fn draw_test() {
//     }
// }
