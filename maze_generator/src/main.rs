use maze_generator::Maze;

fn main() {
    let maze = Maze::generate(20, 20);
    // maze.toggle(5, 5).unwrap();
    // maze.toggle(1, 1).unwrap();
    maze.draw();
}
