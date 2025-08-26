# Maze Generator & Solver

A Rust library and CLI tool for generating random mazes and solving them using the **A\*** pathfinding algorithm. The maze is displayed directly in the terminal with ANSI colors for walls, paths, start, destination, and the search trace.

---

## Features

* Random maze generation using randomized Prim’s algorithm
* Configurable size (`height × width`)
* Customizable **start** and **destination** points
* Maze visualization with colored output:
  * **Black** = Wall
  * **White** = Path
  * **Green** = Start
  * **Red** = Destination
  * **Cyan** = Expanded nodes during search
  * **Blue** = Final path trace
* Pathfinding with the **A\*** algorithm (Manhattan/Euclidean distance heuristics)

---

## Run

```bash
cargo run
```

This will:

1. Generate a random maze (default `20 × 20`).
2. Print the maze with start and destination marked.
3. Solve it using A\* and display the explored space and final path.

---

## TODO

* [ ] Add alternative pathfinding algorithms (BFS, Dijkstra)
* [ ] Command-line arguments for maze size and seed
* [ ] Export maze to image formats
