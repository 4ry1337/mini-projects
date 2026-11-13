# Run-Length Encoding

Run-length encoding and decoding of strings.

**Status: library with two tests.**

Run-length encoding replaces each run of the same character with a count and
the character. For example, `"wwwwddr"` becomes `"4w2d1r"`. Decoding expands
each count again. Counts can have more than one digit, such as `"12a"`.

## Usage

The crate has no binary. Run the tests:

```sh
cargo test
```

Add `-- --nocapture` to see the messages the tests print.

```rust
use rle::{decoding, encoding};

assert_eq!(encoding("wwwwddr"), "4w2d1r");
assert_eq!(decoding("4w2d2r"), "wwwwddrr");
```

## Known Issues

- Digits in the input are lost, because the decoder reads them as counts. For
  example, `"a11"` encodes and then decodes to `"a"`.
- `encoding` panics on non-ASCII input such as `"éé"`. It mixes byte
  positions with character positions.
- `decoding` panics if a character has no count before it, as in `"ab"`.
- `encoding` calls `chars().nth(i)` in a loop, so it takes quadratic time.

## License

[MIT](../LICENSE) © 2024-2026 Rakhat Yskak
