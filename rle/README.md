# Run-Length Encoding (RLE) in Rust

A simple implementation of **run-length encoding (RLE)** and decoding in Rust.

---

## Features

* **Encoding**: Compresses a string by replacing consecutive repeating characters with a count followed by the character.

  * Example: `"wwwwddr"` → `"4w2d1r"`
* **Decoding**: Expands the encoded string back to its original form.

  * Example: `"4w2d2r"` → `"wwwwddrr"`
* Includes unit tests for correctness.

---

## Usage

### Encode

```rust
use crate::encoding;

fn main() {
    let message = "wwwwddr";
    let encoded = encoding(message);
    println!("Encoded: {}", encoded); // "4w2d1r"
}
```

### Decode

```rust
use crate::decoding;

fn main() {
    let encoded = "4w2d2r";
    let decoded = decoding(encoded);
    println!("Decoded: {}", decoded); // "wwwwddrr"
}
```

---

## Tests

Run tests with:

```bash
cargo test
```

Example test output:

```
running 2 tests
test tests::encoding_test ... ok
test tests::decoding_test ... ok
```

---

## Debugging

To see the debug output inside tests, use `-- --nocapture`:

```bash
cargo test -- --nocapture
```

Example output:

```
running 2 tests
Message: wwwwddr
Encoded Message: 4w2d1r
Encoded Message:: 4w2d2r
Decoded Message: wwwwddrr
test tests::encoding_test ... ok
test tests::decoding_test ... ok
```

---

## Code Overview

* **`encoding(data: &str) -> String`**
  Iterates over input, counts consecutive characters, and outputs `<count><char>` pairs.
* **`decoding(data: &str) -> String`**
  Parses counts and expands characters accordingly.

---

## TODO

* [ ] Handle multi-digit counts more robustly in encoding
* [ ] Add CLI interface for file compression/decompression
* [ ] Benchmark performance on large inputs
