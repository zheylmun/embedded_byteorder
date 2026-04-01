/*!
Extension of the [`byteorder`] crate with [`embedded-io`] support for
`no_std` environments.

This crate re-exports the core [`ByteOrder`] trait, [`BigEndian`],
[`LittleEndian`], and related types from [`byteorder`], and adds
[`ReadBytesExt`] and [`WriteBytesExt`] implementations for
[`embedded_io::Read`] / [`embedded_io::Write`] and their async
counterparts.

# Organization

| Module | Feature required | Trait source |
|---|---|---|
| crate root (re-exports) | *(none)* | `byteorder` core types |
| [`io`] | `std` | `byteorder::ReadBytesExt` / `WriteBytesExt` for `std::io` |
| [`eio`] | `embedded-io` | `ReadBytesExt` / `WriteBytesExt` for `embedded_io` |
| [`eio_async`] | `embedded-io-async` | async `ReadBytesExt` / `WriteBytesExt` for `embedded_io_async` |

When the `embedded-io` feature is active, the `eio` traits are also
re-exported at the crate root for convenience.

# Examples

Write and read `u32` numbers in little endian order (always available,
`no_std`):

```rust
use byteorder_embedded_io::{ByteOrder, LittleEndian};

let mut buf = [0; 4];
LittleEndian::write_u32(&mut buf, 1_000_000);
assert_eq!(1_000_000, LittleEndian::read_u32(&buf));
```

[`byteorder`]: https://crates.io/crates/byteorder
[`embedded-io`]: https://crates.io/crates/embedded-io
[`ByteOrder`]: trait.ByteOrder.html
[`BigEndian`]: enum.BigEndian.html
[`LittleEndian`]: enum.LittleEndian.html
[`ReadBytesExt`]: trait.ReadBytesExt.html
[`WriteBytesExt`]: trait.WriteBytesExt.html
*/

#![deny(missing_docs)]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg))]

// Re-export the core byteorder types so users only need one crate.
pub use byteorder::{
    BigEndian, ByteOrder, LittleEndian, NativeEndian, NetworkEndian,
};

/// `ReadBytesExt` and `WriteBytesExt` for `std::io::Read` and `std::io::Write`.
///
/// These are re-exported directly from the [`byteorder`] crate.
#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
pub mod io {
    pub use byteorder::{ReadBytesExt, WriteBytesExt};
}

/// `ReadBytesExt` and `WriteBytesExt` for `embedded_io::Read` and `embedded_io::Write`.
#[cfg(feature = "embedded-io")]
#[cfg_attr(docsrs, doc(cfg(feature = "embedded-io")))]
pub mod eio;

/// Async `ReadBytesExt` and `WriteBytesExt` for `embedded_io_async::Read` and `embedded_io_async::Write`.
#[cfg(feature = "embedded-io-async")]
#[cfg_attr(docsrs, doc(cfg(feature = "embedded-io-async")))]
pub mod eio_async;

// When embedded-io is active, re-export its traits at the crate root for
// convenience (mirrors how upstream byteorder puts std::io traits at root).
#[cfg(feature = "embedded-io")]
#[cfg_attr(docsrs, doc(cfg(feature = "embedded-io")))]
pub use crate::eio::{ReadBytesExt, WriteBytesExt};

#[cfg(feature = "embedded-io")]
#[cfg_attr(docsrs, doc(cfg(feature = "embedded-io")))]
pub use embedded_io::ReadExactError;
