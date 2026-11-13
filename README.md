# Mini Projects

Small, self-contained projects in Rust and C: terminal demos, algorithms and a few games.

**Status: a collection.** Each project stands alone and has its own status.
Some are finished, some are stubs. Pull requests are welcome.

Each folder is its own Cargo crate or Make project. Nothing is shared between
them, so you can build any one without the others.

## Table of Contents

- [Projects](#projects)
- [Install](#install)
- [Usage](#usage)
- [Maintainers](#maintainers)
- [Contributing](#contributing)
- [License](#license)

## Projects

| Name | Language | Description | Status |
| --- | --- | --- | --- |
| [collatz_sequence](./collatz_sequence) | Rust | Length of a Collatz sequence | Runs |
| [cube](./cube) | Rust | Rotating 3D cube drawn in ASCII with ANSI colors | Runs |
| [dubble_buffer](./dubble_buffer) | Rust | Double-buffered character frame buffer | Library only |
| [dungeon_crawler](./dungeon_crawler) | Rust | "Flappy Dragon", a one-button game in bracket-lib | Build fails with CMake 4; see its notes |
| [game-of-life](./game-of-life) | Rust | Conway's Game of Life in the terminal | Runs |
| [grep_lite](./grep_lite) | Rust | Planned `grep` clone | Stub |
| [huffman](./huffman) | Rust | Canonical Huffman encoding and decoding | Library; one test |
| [ip_sniffer](./ip_sniffer) | Rust | Multithreaded TCP port scanner | Runs |
| [maze_generator](./maze_generator) | Rust | Maze generation with Prim's algorithm, solved with A\* | Runs |
| [rle](./rle) | Rust | Run-length encoding and decoding | Library; two tests |
| [robot-demolition-derby](./robot-demolition-derby) | C | Turn-based game: crash robots into each other | Runs |
| [temp-rs](./temp-rs) | Rust | Kelvin, Celsius and Fahrenheit types | Library; nine tests |
| [transponse_matrix](./transponse_matrix) | Rust | 3×3 matrix transpose | Does not compile |

## Install

### Dependencies

- [Rust](https://www.rust-lang.org/tools/install) and Cargo, for the Rust
  projects. Some crates use the 2024 edition, so use a recent toolchain.
  The projects were last checked with Rust 1.99.0.
- `gcc`, `g++` and `make`, for the C project.
- [devenv](https://devenv.sh/) (optional). Five projects ship a `devenv.nix`:
  `collatz_sequence`, `cube`, `game-of-life`, `maze_generator` and `temp-rs`.
  It provides the Rust toolchain.

A project's own README lists anything else it needs.

### Steps

```sh
git clone https://github.com/4ry1337/mini-projects.git
cd mini-projects
```

## Usage

Go into a project folder and build or run it there.

For a Rust project:

```sh
cd game-of-life
cargo run
```

For a library crate, run its tests instead:

```sh
cd huffman
cargo test -- --nocapture
```

For the C project:

```sh
cd robot-demolition-derby
make
./build/robot-demolition-derby
```

With devenv, enter the shell first:

```sh
cd cube
devenv shell
cargo run
```

## Maintainers

Rakhat Yskak: [@4ry1337](https://github.com/4ry1337), yskak.rakhat@gmail.com

## Contributing

Ask questions and report bugs in [GitHub Issues](https://github.com/4ry1337/mini-projects/issues).
Pull requests are welcome. For a big change, open an issue first so we can
agree on the plan.

## License

[MIT](LICENSE) © 2024-2026 Rakhat Yskak
