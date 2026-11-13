# Double Buffer

A character frame buffer with double buffering.

**Status: library only.** `main` prints "Hello, world!" and does not use the
library yet.

Double buffering keeps two buffers. You draw the next frame into the write
buffer while the screen shows the read buffer. Then the two swap, so the
screen never shows a half-drawn frame.

## Usage

```sh
cargo build
```

```rust
use dubble_buffer::scene::Scene;

let mut scene = Scene::new(10, 10);
scene.draw(|buf| {
    buf.clear();
    buf.set(2, 3, '#');
});
scene.display();
```

## API

| Item | File | Role |
| --- | --- | --- |
| `FrameBuffer` | [`src/buffer.rs`](src/buffer.rs) | A grid of `char`s, with `get`, `set`, `fill`, `clear` and `display`. |
| `Scene` | [`src/scene.rs`](src/scene.rs) | Holds a read buffer and a write buffer. |
| `Scene::draw` | | Runs your closure on the write buffer, then swaps the two buffers. |
| `Scene::swap_cloning` | | Copies the write buffer into the read buffer, without a swap. |

`Scene` derefs to the read buffer for reading and to the write buffer for
writing. `AsRef`, `AsMut`, `Borrow` and `BorrowMut` follow the same rule.

## Known Issues

- `FrameBuffer` computes the index as `row * height + col`. It should be
  `row * width + col`. Buffers that are not square read and write the wrong
  cells, and `set` can panic.
- After a swap, the write buffer holds a frame from two draws ago. A draw
  closure must redraw every cell or call `clear` first.
- `main.rs` declares the `buffer` and `scene` modules again, so they are
  compiled into both the library and the binary.

## License

[MIT](../LICENSE) © 2024-2026 Rakhat Yskak
