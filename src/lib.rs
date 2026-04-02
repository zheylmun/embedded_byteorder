/*!
Extension of the [`byteorder`] crate with [`embedded-io`] support for
`no_std` environments.

This crate re-exports the core [`ByteOrder`] trait, [`BigEndian`],
[`LittleEndian`], and related types from [`byteorder`], and adds
`ReadBytesExt` and `WriteBytesExt` implementations for
[`embedded_io::Read`] / [`embedded_io::Write`] and their async
counterparts.

# Organization

| Module | Feature required | Trait source |
|---|---|---|
| crate root (re-exports) | *(none)* | `byteorder` core types |
| `io` | `std` | `byteorder::ReadBytesExt` / `WriteBytesExt` for `std::io` |
| `eio` | `embedded-io` | `ReadBytesExt` / `WriteBytesExt` for `embedded_io` |
| `eio_async` | `embedded-io-async` | async `ReadBytesExt` / `WriteBytesExt` for `embedded_io_async` |
| `adapters` | `adapters` | `FromStd` / `ToStd` bridging `std::io` ↔ `embedded-io` |

When the `embedded-io` feature is active, the `eio` traits and the core
`embedded-io` types (`Read`, `Write`, `ErrorType`, `ReadExactError`)
are re-exported at the crate root for convenience. When `embedded-io-async`
is active, `AsyncRead` and `AsyncWrite` are also re-exported.

# Examples

Write and read `u32` numbers in little endian order (always available,
`no_std`):

```rust
use byteorder_embedded_io::{ByteOrder, LittleEndian};

let mut buf = [0; 4];
LittleEndian::write_u32(&mut buf, 1_000_000);
assert_eq!(1_000_000, LittleEndian::read_u32(&buf));
```

# Minimum Supported Rust Version (MSRV)

The base crate (no features) requires **Rust 1.60**. Enabling
`embedded-io`, `embedded-io-async`, or `adapters` requires **Rust 1.81**
due to the `embedded-io` dependency.

[`byteorder`]: https://crates.io/crates/byteorder
[`embedded-io`]: https://crates.io/crates/embedded-io
[`embedded_io::Read`]: https://docs.rs/embedded-io/0.7/embedded_io/trait.Read.html
[`embedded_io::Write`]: https://docs.rs/embedded-io/0.7/embedded_io/trait.Write.html
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

/// Adapters between `std::io` and `embedded-io` traits.
///
/// Re-exports [`FromStd`](adapters::FromStd) and [`ToStd`](adapters::ToStd)
/// from [`embedded-io-adapters`](https://crates.io/crates/embedded-io-adapters),
/// making it easy to bridge `std::io` and `embedded-io` in projects that
/// target both desktop and embedded.
///
/// # Examples
///
/// Wrap a `std::io::Cursor` so it can be used with [`ReadBytesExt`]:
///
/// ```rust
/// use byteorder_embedded_io::{BigEndian, ReadBytesExt};
/// use byteorder_embedded_io::adapters::FromStd;
///
/// let mut rdr = FromStd::new(std::io::Cursor::new(vec![0, 0, 1, 0]));
/// assert_eq!(256, rdr.read_u32::<BigEndian>().unwrap());
/// ```
#[cfg(feature = "adapters")]
#[cfg_attr(docsrs, doc(cfg(feature = "adapters")))]
pub mod adapters {
    pub use embedded_io_adapters::std::{FromStd, ToStd};
}

// When embedded-io is active, re-export its traits at the crate root for
// convenience (mirrors how upstream byteorder puts std::io traits at root).
#[cfg(feature = "embedded-io")]
#[cfg_attr(docsrs, doc(cfg(feature = "embedded-io")))]
pub use crate::eio::{ReadBytesExt, WriteBytesExt};

#[cfg(feature = "embedded-io")]
#[cfg_attr(docsrs, doc(cfg(feature = "embedded-io")))]
pub use embedded_io::{ErrorType, Read, ReadExactError, Write};

#[cfg(feature = "embedded-io-async")]
#[cfg_attr(docsrs, doc(cfg(feature = "embedded-io-async")))]
pub use embedded_io_async::{Read as AsyncRead, Write as AsyncWrite};
