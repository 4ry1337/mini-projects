use maze_generator::Maze;

fn main() {
    let maze = Maze::generate(20, 20);
    maze.draw();
    maze.astar();
}
