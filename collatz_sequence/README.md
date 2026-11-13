# Collatz Sequence

Counts the terms in the [Collatz sequence](https://en.wikipedia.org/wiki/Collatz_conjecture) that starts at a given number.

**Status: runs.**

The sequence starts at `n`. If `n` is even, the next term is `n / 2`. If `n`
is odd, it is `3n + 1`. The sequence stops at 1. `collatz_length` counts every
term, including the first and the last.

## Usage

```sh
cargo run
```

The program computes the length for 11 and checks it with `assert!`:

```text
Length: 15
```

With devenv, run `devenv shell` first. It provides the Rust toolchain.

## Known Issues

- The start value, 11, is hard-coded in `main`.
- `collatz_length` takes an `i32`. A large start value makes `3n + 1`
  overflow, which panics in a debug build.

## License

[MIT](../LICENSE) © 2024-2026 Rakhat Yskak
