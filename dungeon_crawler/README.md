# Flappy Dragon

A one-button game made with [bracket-lib](https://github.com/amethyst/bracket-lib): fly through the gaps in the walls.

**Status: not checked.** With CMake 4, the native build fails. See
[Known Issues](#known-issues).

The folder is named `dungeon_crawler`, but the code is a Flappy Bird-style
game called "Flappy Dragon". The dragon (`@`) falls under gravity. Press
`Space` to flap up. Each wall you pass adds a point, and each point makes the
next gap smaller, down to 2 cells.

## Install

### Dependencies

bracket-lib 0.8.7 builds native C libraries. Its build scripts asked for at
least these, beyond Rust:

- `cmake` and `make`
- `pkg-config`
- the fontconfig and freetype development files

### Build

```sh
cargo build
```

## Usage

```sh
cargo run
```

| Key | Screen | Action |
| --- | --- | --- |
| `P` | Menu, game over | Start a new game |
| `Q` | Menu, game over | Quit |
| `Space` | Playing | Flap |

The game ends if you hit a wall or fall off the bottom of the screen.

On Wayland, if the window fails to open, force the X11 backend:

```sh
WINIT_UNIX_BACKEND=x11 cargo run
```

## Known Issues

- CMake 4 dropped support for projects that ask for a CMake version below
  3.5, so the `expat-sys` build script fails. CMake's error message suggests
  this workaround:

  ```sh
  CMAKE_POLICY_VERSION_MINIMUM=3.5 cargo build
  ```

  With that set, the build then failed in `servo-fontconfig-sys`, in a nix
  shell with fontconfig present. A full build has not been confirmed with
  CMake 4.4.

## License

[MIT](../LICENSE) © 2024-2026 Rakhat Yskak
