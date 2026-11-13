# Matrix Transpose

Transposes a 3×3 matrix and prints it before and after.

**Status: does not compile.** `transpose` has an empty loop and returns
nothing.

The transpose of a matrix swaps its rows and columns: the value at row `i`,
column `j` moves to row `j`, column `i`.

## Usage

```sh
cargo run
```

The build fails right now with `error[E0308]: mismatched types`. `transpose`
must return a `[[i32; 3]; 3]`, but its body ends with a `for` loop, which
evaluates to `()`.

## Known Issues

- `transpose` is not written yet, as described above.
- `Cargo.toml` names the package `collatz_sequence`, which was copied from the
  other project. The binary therefore builds as `collatz_sequence`.
- The folder name has a typo: "transponse".

## License

[MIT](../LICENSE) © 2024-2026 Rakhat Yskak
