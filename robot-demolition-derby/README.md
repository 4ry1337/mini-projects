# Robot Demolition Derby

A turn-based terminal game in C: steer robots on a grid until they all crash.

**Status: runs.**

Ten robots start at random places on a 10×10 grid, each facing a random way.
On each turn, you pick one robot and move it forward or turn it. A robot that
drives into another robot crashes, and so does the robot it hits. You win
when no robot is left running.

## Install

### Dependencies

- `gcc`, `g++` and `make`. The makefile links with `$(CXX)`.

### Build

```sh
make
```

The program is built at `build/robot-demolition-derby`. Run `make clean` to
remove the build folder.

## Usage

```sh
./build/robot-demolition-derby
```

At the start menu, enter `Y` to play, `N` to quit, or `C` to type a seed. The
same seed gives the same start. Without one, the seed is the current time.

The grid shows `.` for an empty cell, `R` for a running robot and `@` for a
crashed one. Rows and columns are numbered from 0.

| Command | Action |
| --- | --- |
| `F row col` | Move the robot forward one cell |
| `L row col` | Turn the robot left |
| `R row col` | Turn the robot right |
| `S` | Stop and print the seed |
| `H` | Show help |

For example, `F 2 3` moves the robot in row 2, column 3. A robot at the west
or north edge does not move past it.

## Known Issues

- The east and south bounds checks compare with `SIZE` instead of
  `SIZE - 1`. Moving east from the last column jumps to the first cell of the
  next row. From the last row, moving south or east writes past the end of the
  grid.
- Turning left while facing west sets the direction to −1, since `%` in C
  keeps the sign. The robot cannot move until you turn it right again.
- `W` sets the robot count to 0, which wins the game at once. It is a
  leftover debug key.
- `drd_game.c` defines its structs again instead of including `drd_game.h`.
- `scanf` return values are not checked. Text where a number belongs is not
  handled.

## License

[MIT](../LICENSE) © 2024-2026 Rakhat Yskak
