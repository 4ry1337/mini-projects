# Rotating ASCII Cube in Rust

A terminal-based **3D rotating cube renderer** written in Rust. It projects a cube into ASCII art with ANSI colors, updating in real time.

![Cube](https://github.com/4ry1337/mini-projects/blob/master/cube/demo.gif)

---

## Features

* Rotating cube rendered directly in the terminal
* Adjustable resolution (`height × width`) and cube size
* **Random or fixed rotation speed**
* Colorized cube faces using ANSI escape codes
* Z-buffering for correct face visibility
* Configurable initial rotation and custom speed per axis

---

## Usage

### Run

```bash
cargo run
```

This starts the cube animation with random rotation speed.

---

## Controls

Since this is a rendering demo:

* Stop with `Ctrl+C`.
* Adjust terminal size for best viewing.
* Modify parameters in `main.rs` to change resolution, cube size, or speed.

---

## TODO

* [ ] Add keyboard controls for pausing, adjusting speed, or resetting rotation
* [ ] Export frames to GIF for previews in README
* [ ] Support multiple cube instances on screen
