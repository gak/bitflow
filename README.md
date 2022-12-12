# fleabit

**fleabit** is a Rust crate that allows reading and writing unaligned bytes. It provides a simple API
and is built on top of bitvec.

It is intended for a multiplayer game library that's in development.

Note that this crate is probably not production ready. Use at your own discretion.

## Example

```rust
use fleabit::{FleaBitReader, FleaBitWriter};

let mut writer = FleaBitWriter::new();

writer.bool(false);
assert_eq!(writer.to_string(), ".......0");

writer.u8(135);
assert_eq!(writer.to_string(), "00001110_.......1");

let bytes = writer.into_vec();
assert_eq!(bytes, vec![0x0e, 0x01]);

let mut reader = FleaBitReader::from_slice(&bytes);
assert_eq!(reader.bool(), false);
assert_eq!(reader.u8(), 135);
```

## License

**fleabit** is licensed under either the [MIT license](LICENSE-MIT) or
the [Apache-2.0 license](LICENSE-APACHE), at your discretion.
