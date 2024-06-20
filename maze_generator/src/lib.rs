use std::fmt::Display;

const BLACK_BRICK: &'static str = "\x1B[40m  \x1B[0m";
const WHITE_BRICK: &'static str = "\x1B[47m  \x1B[0m";

#[derive(Debug)]
struct Maze {
    seed: u8,
    height: u32,
    width: u32,
    map: Vec<Vec<Cell>>,
}

#[derive(Debug)]
struct Cell {
    x: u32,
    y: u32,
    wall: bool,
}

impl Maze {
    fn generate(height: u32, width: u32) -> Self {
        let mut map = vec![];
        let seed = 0;
        for y in 0..height {
            let mut row = vec![];
            for x in 0..width {
                row.push(Cell { y, x, wall: false })
            }
            map.push(row);
        }

        Self {
            map,
            height,
            width,
            seed,
        }
    }

    fn draw(&self) {
        for row in self.map.iter() {
            for cell in row.iter() {
                if cell.wall {
                    print!("{}", BLACK_BRICK);
                } else {
                    print!("{}", WHITE_BRICK);
                }
            }
            println!();
        }
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "x: {}, y: {}, wall: {}", self.x, self.y, self.wall)
    }
}

#[cfg(test)]
mod tests {
    use crate::Maze;

    #[test]
    fn draw_test() {
        let maze = Maze::generate(10, 10);
        maze.draw()
    }
}
