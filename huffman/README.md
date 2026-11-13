# Huffman Coding

Canonical Huffman encoding and decoding of strings.

**Status: library with one test.** It encodes and decodes the test sentence,
but some inputs fail. See [Known Issues](#known-issues).

Huffman coding gives short bit codes to common bytes and long codes to rare
ones. This crate builds the Huffman tree from byte counts, then turns the codes
into canonical form. Canonical codes can be rebuilt from just two things: the
number of codes of each length, and the symbols in order. So the decoder needs
only that small table, not the whole tree.

## Usage

The crate has no binary. Run the test to see it work:

```sh
cargo test -- --nocapture
```

The test encodes "the quick brown fox jumps over the lazy dog". The 43 bytes
become 192 bits.

```rust
use huffman::{decode, encode};

let (bits, table) = encode("hello");
let bytes = decode(&bits, table);
assert_eq!(bytes.as_deref(), Some("hello".as_bytes()));
```

## API

| Item | Role |
| --- | --- |
| `encode(&str) -> (String, HuffmanTable)` | Returns the code as a string of `'0'` and `'1'` characters, plus the table. |
| `decode(&str, HuffmanTable) -> Option<Vec<u8>>` | Rebuilds the codes from the table and decodes the bit string. |
| `HuffmanNode::tree` | Builds the Huffman tree from byte counts. |
| `HuffmanCode::as_canonical` | Turns tree codes into canonical codes. |
| `HuffmanTable`, `HuffmanCode::describe` | Print the table or the codes, for debugging. |

## Known Issues

- `decode` is wrong when the code lengths skip a value. For example,
  `"aaaabcde"` gives codes of length 1, 3, 3, 3 and 3. It does not decode back
  to the input. The cause is `HuffmanCode::from_table`, which shifts the code
  one bit per new length instead of one bit per length step.
- The output is a `String` of `'0'` and `'1'` characters, not packed bits. Each
  bit takes a full byte.
- Codes live in a `u128`, so no code can be longer than 128 bits.
- `decode` formats every code as a string and compares it to the bits read so
  far. That is slow on long inputs.
- The test reports a 14% compression rate. It assumes 32 bits per input
  character. At 8 bits per byte, the true rate is 192 / 344, about 56%.

## License

[MIT](../LICENSE) © 2024-2026 Rakhat Yskak
