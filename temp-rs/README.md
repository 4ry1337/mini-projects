# temp-rs

Temperature types for Kelvin, Celsius and Fahrenheit, with conversions between them.

**Status: library with nine tests.**

Each scale is an `f32` newtype: `Kelvin`, `Celsius` and `Fahrenheit`. `From`
converts between any two of them. `valid()` checks that a value is at or above
absolute zero. The crate also serves as a template for strict Rust projects:
clippy's `pedantic` and `nursery` groups are set to deny, and so are lints
for panics, unchecked arithmetic and `as` casts.

## Install

### Dependencies

- Rust with Cargo, rustfmt and clippy. `rustfmt.toml` turns on unstable
  options, so formatting needs a nightly rustfmt.
- `clang` and `mold`. On x86_64 Linux, [`.cargo/config.toml`](.cargo/config.toml)
  links with them.
- [cargo-nextest](https://nexte.st/) (optional), for the `watcher` script.

[devenv](https://devenv.sh/) provides all of these. Enter its shell:

```sh
devenv shell
```

## Usage

```rust
use temp_rs::{Celsius, Fahrenheit, Kelvin};

let boiling = Celsius::new(100.0);
let f = Fahrenheit::from(boiling);
assert!(Kelvin::from(f).valid());
```

Run the tests:

```sh
cargo test
```

In the devenv shell, `watcher` runs `cargo clippy` and `cargo nextest run`
each time a `.rs` file changes. The git hooks run `typos` and `clippy`.

## Known Issues

- `cargo clippy --all-targets` fails. A test uses `-17.77777777777778`, and
  clippy's `unreadable_literal` lint wants digit separators.
- The `valid_fahrenheit` test checks `459.67`, not the minimum `-459.67`.
- `MAX` itself counts as invalid, because `valid()` uses the half-open range
  `MIN..MAX`.

## License

[MIT](../LICENSE) © 2024-2026 Rakhat Yskak
