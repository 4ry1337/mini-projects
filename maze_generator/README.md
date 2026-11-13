# Maze Generator and Solver

Generates a random maze with Prim's algorithm and solves it with A\*, drawn in the terminal with ANSI colors.

**Status: runs.**

The generator starts with a grid of walls. It opens a random cell, then grows
the maze with randomized Prim's algorithm: it picks a random frontier cell two
steps away and carves a path to it. Then it places a destination far from the
start. The solver runs A\* with the Manhattan distance as its heuristic.

## Usage

```sh
cargo run
```

The program prints the maze, a `---` line, and then the solved maze. Each run
gives a new maze.

| Color | Meaning |
| --- | --- |
| Black | Wall |
| White | Open path |
| Green | Start |
| Red | Destination |
| Cyan | Cells that A\* expanded |
| Blue | The path found |

With devenv, run `devenv shell` first. It provides the Rust toolchain.

## Configuration

`src/main.rs` calls `Maze::generate(20, 20)`. Change the height and width
there. `Maze::set_start` and `Maze::set_distanation` move the two end points.
They return an error if the cell is a wall or the other end point.

## Known Issues

- `Point::euclidian_distance` exists, but nothing calls it. The solver only
  uses the Manhattan distance.
- A\* finds the next node with a linear scan of the open set, not a priority
  queue. This is fine for small mazes but slow for big ones.
- The generator uses `thread_rng`, so a maze cannot be repeated with a seed.

## License

[MIT](../LICENSE) © 2024-2026 Rakhat Yskak
