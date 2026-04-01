# byteorder-embedded-io

[![CI](https://github.com/zheylmun/byteorder-embedded-io/actions/workflows/ci.yml/badge.svg)](https://github.com/zheylmun/byteorder-embedded-io/actions/workflows/ci.yml)
[![codecov](https://codecov.io/gh/zheylmun/byteorder-embedded-io/graph/badge.svg)](https://codecov.io/gh/zheylmun/byteorder-embedded-io)
[![crates.io](https://img.shields.io/crates/v/byteorder-embedded-io.svg)](https://crates.io/crates/byteorder-embedded-io)
[![docs.rs](https://docs.rs/byteorder-embedded-io/badge.svg)](https://docs.rs/byteorder-embedded-io)

Extension of [byteorder](https://crates.io/crates/byteorder) with
[embedded-io](https://crates.io/crates/embedded-io) `Read`/`Write` trait
support for `no_std` environments.

The upstream byteorder crate has
[suggested that there might be an issue with](https://github.com/BurntSushi/byteorder/issues/213)
embedded-io support.
This crate re-exports the core `byteorder` types (`ByteOrder`, `BigEndian`,
`LittleEndian`, `NativeEndian`, `NetworkEndian`) and adds `ReadBytesExt` /
`WriteBytesExt` implementations for `embedded_io::Read` / `Write` and their
async counterparts.

## Features

| Feature | Default | What it enables |
|---|---|---|
| *(none)* | yes | Re-exports `ByteOrder`, `BigEndian`, `LittleEndian`, `NativeEndian`, `NetworkEndian` from `byteorder` (always `no_std`) |
| `std` | no | `ReadBytesExt` / `WriteBytesExt` for `std::io::Read` / `Write` (via `byteorder/std`) |
| `embedded-io` | no | `ReadBytesExt` / `WriteBytesExt` for `embedded_io::Read` / `Write` |
| `embedded-io-async` | no | Async `ReadBytesExt` / `WriteBytesExt` for `embedded_io_async::Read` / `Write` (implies `embedded-io`) |

All features can be enabled simultaneously; they live in separate modules
(`byteorder_embedded_io::io`, `byteorder_embedded_io::eio`,
and `byteorder_embedded_io::eio_async`). When `embedded-io` is enabled, its traits
are also re-exported at the crate root.

## Usage

### `ByteOrder` (always available, `no_std`)

```rust
use byteorder_embedded_io::{ByteOrder, LittleEndian};

let mut buf = [0u8; 4];
LittleEndian::write_u32(&mut buf, 1_000_000);
assert_eq!(1_000_000, LittleEndian::read_u32(&buf));
```

### `embedded-io` (requires `embedded-io` feature)

```rust,ignore
use byteorder_embedded_io::{BigEndian, ReadBytesExt, WriteBytesExt};

// Read from any embedded_io::Read
let mut rdr = &[0x00u8, 0x03, 0x43, 0x95][..];
let val = rdr.read_u32::<BigEndian>().unwrap();

// Write to any embedded_io::Write
let mut buf = [0u8; 4];
let mut wtr = &mut buf[..];
wtr.write_u32::<BigEndian>(267).unwrap();
```

### `embedded-io-async` (requires `embedded-io-async` feature)

```rust,ignore
use byteorder_embedded_io::BigEndian;
use byteorder_embedded_io::eio_async::ReadBytesExt;

async fn read_value(rdr: &mut &[u8]) -> u32 {
    rdr.read_u32::<BigEndian>().await.unwrap()
}
```

### `std::io` (requires `std` feature)

```rust,ignore
use std::io::Cursor;
use byteorder_embedded_io::{BigEndian, io::ReadBytesExt};

let mut rdr = Cursor::new(vec![2, 5, 3, 0]);
assert_eq!(517, rdr.read_u16::<BigEndian>().unwrap());
```

## MSRV

- Base crate (no features): **Rust 1.60** (matches `byteorder`)
- `embedded-io` / `embedded-io-async` features: **Rust 1.81** (required by `embedded-io 0.7`)

## License

This project is licensed under either of

- The Unlicense ([UNLICENSE](UNLICENSE))
- MIT License ([LICENSE-MIT](LICENSE-MIT))

at your option.
