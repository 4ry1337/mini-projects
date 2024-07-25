use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    usize,
};

use rand::{thread_rng, Rng};

pub const OBSTICLE: &'static str = "\x1B[40m  \x1B[0m"; //BLACK
pub const PATH: &'static str = "\x1B[47m  \x1B[0m"; //WHITE
pub const START: &'static str = "\x1B[42m  \x1B[0m"; //GREEN
pub const DESTINATION: &'static str = "\x1B[41m  \x1B[0m"; //RED

pub const TRACE: &'static str = "\x1B[44m  \x1B[0m"; //BLUE
pub const EXPANDED: &'static str = "\x1B[46m  \x1B[0m"; //CYAN

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Point: x: {}, y: {}", self.x, self.y)
    }
}

impl Point {
    pub fn distance(&self, point: &Point) -> usize {
        self.manhattan_distance(point)
    }
    pub fn manhattan_distance(&self, point: &Point) -> usize {
        self.x.abs_diff(point.x) + self.y.abs_diff(point.y)
    }
    pub fn euclidian_distance(&self, point: &Point) -> usize {
        let diff_x = self.x.abs_diff(point.x);
        let diff_y = self.y.abs_diff(point.y);
        if diff_x > diff_y {
            return 14 * diff_y + (diff_x - diff_y);
        }
        return 14 * diff_x + (diff_y - diff_x);
    }
}

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

        while destanation_cell.point == start_cell.point
            || destanation_cell.obsticle == true
                // not really necessary just to make distance between start and end cells bigger
            || destanation_cell.point.distance(&start_cell.point) < (height + width) / 2
        {
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

    fn neighbors(&self, point: &Point) -> Vec<&Cell> {
        let mut points = Vec::new();
        if 0 < point.x {
            points.push(&self.grid[point.y][point.x - 1]);
        }
        if point.x < self.grid[0].len() - 1 {
            points.push(&self.grid[point.y][point.x + 1]);
        }
        if 0 < point.y {
            points.push(&self.grid[point.y - 1][point.x]);
        }
        if point.y < self.grid.len() - 1 {
            points.push(&self.grid[point.y + 1][point.x]);
        }
        points
    }

    pub fn astar(&self) {
        // the set of nodes to be evaluated
        // the set of nodes already evaluated
        // add start point to open

        let mut open = HashSet::new();

        let mut closed = HashSet::new();

        open.insert(&self.start);

        let mut g_scores = HashMap::new();

        g_scores.insert(&self.start, 0);

        let mut h_scores = HashMap::new();

        h_scores.insert(&self.start, self.start.distance(&self.destanation));

        let mut parent = HashMap::new();

        // current = node in Open with the lowest f cost
        // remove from Open
        // add to Closed
        // if current is the target node path has been found
        // or else
        // for each neighbor is not traversable or neighbor is in closed
        //  skip to the next neighbor
        // if new path to neighbor is shorter OR neighbor is not on Open
        // set f cost of neighbor
        // set parent of neighbor to current
        // if neighbor is not in open
        // add neighbor to open

        let mut expanded = HashSet::new();

        while open.len() > 0 {
            let mut lowest_fscore = usize::max_value();
            let mut point = &self.start;
            let mut f_score;
            for el in open.iter() {
                f_score = g_scores[*el] + h_scores[*el];
                if f_score <= lowest_fscore {
                    lowest_fscore = f_score;
                    point = *el;
                }
            }
            expanded.insert(point);
            open.remove(point);
            closed.insert(point);
            if point == &self.destanation {
                break;
            }
            for neighbor in self.neighbors(point) {
                if closed.contains(&neighbor.point) || neighbor.obsticle {
                    continue;
                }

                let new_cost_to_neighbor: usize = g_scores[point] + point.distance(&neighbor.point);

                open.insert(&neighbor.point);

                if g_scores.get(&neighbor.point).is_some() {
                    if new_cost_to_neighbor < g_scores[&neighbor.point] {
                        g_scores.insert(&neighbor.point, new_cost_to_neighbor);
                    }
                } else {
                    g_scores.insert(&neighbor.point, new_cost_to_neighbor);
                }

                parent.insert(&neighbor.point, point);

                h_scores.insert(&neighbor.point, neighbor.point.distance(&self.destanation));
            }
            //
            //for row in self.grid.iter() {
            //    for cell in row.iter() {
            //        if cell.obsticle {
            //            print!("{}", OBSTICLE);
            //        } else {
            //            if cell.point == self.start {
            //                print!("{}", START);
            //            } else if cell.point == self.destanation {
            //                print!("{}", DESTINATION);
            //            } else if expanded.contains(&cell.point) {
            //                print!("{}", EXPANDED);
            //            } else {
            //                print!("{}", PATH);
            //            }
            //        }
            //    }
            //    println!();
            //}
        }
        let mut path = HashSet::new();

        let mut current = &self.destanation;

        while current != &self.start {
            path.insert(current);
            current = parent[current];
        }

        println!("---");

        for row in self.grid.iter() {
            for cell in row.iter() {
                if cell.obsticle {
                    print!("{}", OBSTICLE);
                } else {
                    if cell.point == self.start {
                        print!("{}", START);
                    } else if cell.point == self.destanation {
                        print!("{}", DESTINATION);
                    } else if path.contains(&cell.point) {
                        print!("{}", TRACE);
                    } else if expanded.contains(&cell.point) {
                        print!("{}", EXPANDED);
                    } else {
                        print!("{}", PATH);
                    }
                }
            }
            println!();
        }
    }
}
