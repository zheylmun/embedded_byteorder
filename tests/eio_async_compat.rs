//! Compatibility tests: verify that embedded-io-async `ReadBytesExt` / `WriteBytesExt`
//! produce byte-for-byte identical results to the std::io versions by running
//! both paths through `embedded-io-adapters::FromTokio`.

#![cfg(all(feature = "std", feature = "embedded-io-async"))]

use std::io::Cursor;

use embedded_io_adapters::tokio_1::FromTokio;

use byteorder_embedded_io::{BigEndian, LittleEndian, NativeEndian};

use byteorder_embedded_io::eio_async::ReadBytesExt as AsyncRead;
use byteorder_embedded_io::eio_async::WriteBytesExt as AsyncWrite;
use byteorder_embedded_io::io::ReadBytesExt as StdRead;
use byteorder_embedded_io::io::WriteBytesExt as StdWrite;

// -----------------------------------------------------------------------
// Helpers
// -----------------------------------------------------------------------

fn qc<A: quickcheck::Arbitrary + Clone + std::fmt::Debug>(f: fn(A) -> bool) {
    quickcheck::quickcheck(f);
}

fn block_on<F: std::future::Future>(f: F) -> F::Output {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(f)
}

// -----------------------------------------------------------------------
// Macros – reads
// -----------------------------------------------------------------------

macro_rules! qc_read {
    ($mod:ident, $method:ident, $size:expr) => {
        mod $mod {
            use super::*;

            #[cfg(not(miri))]
            #[test]
            fn big_endian() {
                fn prop(data: Vec<u8>) -> bool {
                    if data.len() < $size {
                        return true;
                    }
                    let d = &data[..$size];
                    let std_val =
                        Cursor::new(d).$method::<BigEndian>().unwrap();
                    let async_val = block_on(async {
                        FromTokio::new(Cursor::new(d))
                            .$method::<BigEndian>()
                            .await
                            .unwrap()
                    });
                    std_val == async_val
                }
                qc(prop as fn(Vec<u8>) -> bool);
            }

            #[cfg(not(miri))]
            #[test]
            fn little_endian() {
                fn prop(data: Vec<u8>) -> bool {
                    if data.len() < $size {
                        return true;
                    }
                    let d = &data[..$size];
                    let std_val =
                        Cursor::new(d).$method::<LittleEndian>().unwrap();
                    let async_val = block_on(async {
                        FromTokio::new(Cursor::new(d))
                            .$method::<LittleEndian>()
                            .await
                            .unwrap()
                    });
                    std_val == async_val
                }
                qc(prop as fn(Vec<u8>) -> bool);
            }

            #[cfg(not(miri))]
            #[test]
            fn native_endian() {
                fn prop(data: Vec<u8>) -> bool {
                    if data.len() < $size {
                        return true;
                    }
                    let d = &data[..$size];
                    let std_val =
                        Cursor::new(d).$method::<NativeEndian>().unwrap();
                    let async_val = block_on(async {
                        FromTokio::new(Cursor::new(d))
                            .$method::<NativeEndian>()
                            .await
                            .unwrap()
                    });
                    std_val == async_val
                }
                qc(prop as fn(Vec<u8>) -> bool);
            }
        }
    };
}

macro_rules! qc_read_float {
    ($mod:ident, $method:ident, $size:expr, $ty:ty) => {
        mod $mod {
            use super::*;

            #[cfg(not(miri))]
            #[test]
            fn big_endian() {
                fn prop(data: Vec<u8>) -> bool {
                    if data.len() < $size {
                        return true;
                    }
                    let d = &data[..$size];
                    let a: $ty =
                        Cursor::new(d).$method::<BigEndian>().unwrap();
                    let b: $ty = block_on(async {
                        FromTokio::new(Cursor::new(d))
                            .$method::<BigEndian>()
                            .await
                            .unwrap()
                    });
                    a.to_bits() == b.to_bits()
                }
                qc(prop as fn(Vec<u8>) -> bool);
            }

            #[cfg(not(miri))]
            #[test]
            fn little_endian() {
                fn prop(data: Vec<u8>) -> bool {
                    if data.len() < $size {
                        return true;
                    }
                    let d = &data[..$size];
                    let a: $ty =
                        Cursor::new(d).$method::<LittleEndian>().unwrap();
                    let b: $ty = block_on(async {
                        FromTokio::new(Cursor::new(d))
                            .$method::<LittleEndian>()
                            .await
                            .unwrap()
                    });
                    a.to_bits() == b.to_bits()
                }
                qc(prop as fn(Vec<u8>) -> bool);
            }
        }
    };
}

macro_rules! qc_read_nbytes {
    ($mod:ident, $method:ident, $max_bytes:expr) => {
        mod $mod {
            use super::*;

            #[cfg(not(miri))]
            #[test]
            fn all_lengths() {
                fn prop(data: Vec<u8>) -> bool {
                    for nbytes in 1..=$max_bytes {
                        if data.len() < nbytes {
                            continue;
                        }
                        let d = &data[..nbytes];
                        let std_val = Cursor::new(d)
                            .$method::<BigEndian>(nbytes)
                            .unwrap();
                        let async_val = block_on(async {
                            FromTokio::new(Cursor::new(d))
                                .$method::<BigEndian>(nbytes)
                                .await
                                .unwrap()
                        });
                        if std_val != async_val {
                            return false;
                        }
                        let std_val = Cursor::new(d)
                            .$method::<LittleEndian>(nbytes)
                            .unwrap();
                        let async_val = block_on(async {
                            FromTokio::new(Cursor::new(d))
                                .$method::<LittleEndian>(nbytes)
                                .await
                                .unwrap()
                        });
                        if std_val != async_val {
                            return false;
                        }
                    }
                    true
                }
                qc(prop as fn(Vec<u8>) -> bool);
            }
        }
    };
}

// -----------------------------------------------------------------------
// Macros – writes
// -----------------------------------------------------------------------

macro_rules! qc_write {
    ($mod:ident, $ty:ty, $method:ident) => {
        mod $mod {
            use super::*;

            #[cfg(not(miri))]
            #[test]
            fn big_endian() {
                fn prop(n: $ty) -> bool {
                    let mut std_buf = vec![];
                    StdWrite::$method::<BigEndian>(&mut std_buf, n).unwrap();
                    let async_buf = block_on(async {
                        let mut w = FromTokio::new(Cursor::new(Vec::new()));
                        w.$method::<BigEndian>(n).await.unwrap();
                        w.into_inner().into_inner()
                    });
                    std_buf == async_buf
                }
                qc(prop as fn($ty) -> bool);
            }

            #[cfg(not(miri))]
            #[test]
            fn little_endian() {
                fn prop(n: $ty) -> bool {
                    let mut std_buf = vec![];
                    StdWrite::$method::<LittleEndian>(&mut std_buf, n)
                        .unwrap();
                    let async_buf = block_on(async {
                        let mut w = FromTokio::new(Cursor::new(Vec::new()));
                        w.$method::<LittleEndian>(n).await.unwrap();
                        w.into_inner().into_inner()
                    });
                    std_buf == async_buf
                }
                qc(prop as fn($ty) -> bool);
            }

            #[cfg(not(miri))]
            #[test]
            fn native_endian() {
                fn prop(n: $ty) -> bool {
                    let mut std_buf = vec![];
                    StdWrite::$method::<NativeEndian>(&mut std_buf, n)
                        .unwrap();
                    let async_buf = block_on(async {
                        let mut w = FromTokio::new(Cursor::new(Vec::new()));
                        w.$method::<NativeEndian>(n).await.unwrap();
                        w.into_inner().into_inner()
                    });
                    std_buf == async_buf
                }
                qc(prop as fn($ty) -> bool);
            }
        }
    };
}

macro_rules! qc_write_float {
    ($mod:ident, $ty:ty, $method:ident) => {
        qc_write!($mod, $ty, $method);
    };
}

macro_rules! qc_write_nbytes {
    ($mod:ident, $ty:ty, $method:ident, $max_bytes:expr) => {
        mod $mod {
            use super::*;

            #[cfg(not(miri))]
            #[test]
            fn all_lengths() {
                fn prop(n: $ty) -> bool {
                    for nbytes in 1..=$max_bytes {
                        let mask: $ty = if nbytes >= std::mem::size_of::<$ty>()
                        {
                            <$ty>::MAX
                        } else {
                            (1 << (nbytes * 8)) - 1
                        };
                        let val = n & mask;

                        let mut std_buf = vec![];
                        StdWrite::$method::<BigEndian>(
                            &mut std_buf,
                            val,
                            nbytes,
                        )
                        .unwrap();
                        let async_buf = block_on(async {
                            let mut w =
                                FromTokio::new(Cursor::new(Vec::new()));
                            w.$method::<BigEndian>(val, nbytes).await.unwrap();
                            w.into_inner().into_inner()
                        });
                        if std_buf != async_buf {
                            return false;
                        }

                        let mut std_buf = vec![];
                        StdWrite::$method::<LittleEndian>(
                            &mut std_buf,
                            val,
                            nbytes,
                        )
                        .unwrap();
                        let async_buf = block_on(async {
                            let mut w =
                                FromTokio::new(Cursor::new(Vec::new()));
                            w.$method::<LittleEndian>(val, nbytes)
                                .await
                                .unwrap();
                            w.into_inner().into_inner()
                        });
                        if std_buf != async_buf {
                            return false;
                        }
                    }
                    true
                }
                qc(prop as fn($ty) -> bool);
            }
        }
    };
}

// -----------------------------------------------------------------------
// Macros – read_into
// -----------------------------------------------------------------------

macro_rules! test_read_into {
    ($mod:ident, $method:ident, $elem_ty:ty, $count:expr, $data:expr) => {
        mod $mod {
            use super::*;

            #[test]
            fn big_endian() {
                let data: &[u8] = &$data;
                let mut std_dst = [<$elem_ty>::default(); $count];
                let mut async_dst = [<$elem_ty>::default(); $count];
                Cursor::new(data).$method::<BigEndian>(&mut std_dst).unwrap();
                block_on(async {
                    FromTokio::new(Cursor::new(data))
                        .$method::<BigEndian>(&mut async_dst)
                        .await
                        .unwrap();
                });
                assert_eq!(std_dst, async_dst);
            }

            #[test]
            fn little_endian() {
                let data: &[u8] = &$data;
                let mut std_dst = [<$elem_ty>::default(); $count];
                let mut async_dst = [<$elem_ty>::default(); $count];
                Cursor::new(data)
                    .$method::<LittleEndian>(&mut std_dst)
                    .unwrap();
                block_on(async {
                    FromTokio::new(Cursor::new(data))
                        .$method::<LittleEndian>(&mut async_dst)
                        .await
                        .unwrap();
                });
                assert_eq!(std_dst, async_dst);
            }
        }
    };
}

macro_rules! test_read_into_no_endian {
    ($mod:ident, $method:ident, $elem_ty:ty, $count:expr, $data:expr) => {
        mod $mod {
            use super::*;

            #[test]
            fn compat() {
                let data: &[u8] = &$data;
                let mut std_dst = [<$elem_ty>::default(); $count];
                let mut async_dst = [<$elem_ty>::default(); $count];
                Cursor::new(data).$method(&mut std_dst).unwrap();
                block_on(async {
                    FromTokio::new(Cursor::new(data))
                        .$method(&mut async_dst)
                        .await
                        .unwrap();
                });
                assert_eq!(std_dst, async_dst);
            }
        }
    };
}

// -----------------------------------------------------------------------
// Macros – 128-bit read/write with fixed values
// -----------------------------------------------------------------------

macro_rules! test_read_128 {
    ($mod:ident, $method:ident, $data:expr) => {
        mod $mod {
            use super::*;

            #[test]
            fn big_endian() {
                let d: &[u8] = &$data;
                let std_val = Cursor::new(d).$method::<BigEndian>().unwrap();
                let async_val = block_on(async {
                    FromTokio::new(Cursor::new(d))
                        .$method::<BigEndian>()
                        .await
                        .unwrap()
                });
                assert_eq!(std_val, async_val);
            }

            #[test]
            fn little_endian() {
                let d: &[u8] = &$data;
                let std_val =
                    Cursor::new(d).$method::<LittleEndian>().unwrap();
                let async_val = block_on(async {
                    FromTokio::new(Cursor::new(d))
                        .$method::<LittleEndian>()
                        .await
                        .unwrap()
                });
                assert_eq!(std_val, async_val);
            }
        }
    };
}

macro_rules! test_write_128 {
    ($mod:ident, $ty:ty, $method:ident, $val:expr) => {
        mod $mod {
            use super::*;

            #[test]
            fn big_endian() {
                let n: $ty = $val;
                let mut std_buf = vec![];
                StdWrite::$method::<BigEndian>(&mut std_buf, n).unwrap();
                let async_buf = block_on(async {
                    let mut w = FromTokio::new(Cursor::new(Vec::new()));
                    w.$method::<BigEndian>(n).await.unwrap();
                    w.into_inner().into_inner()
                });
                assert_eq!(std_buf, async_buf);
            }

            #[test]
            fn little_endian() {
                let n: $ty = $val;
                let mut std_buf = vec![];
                StdWrite::$method::<LittleEndian>(&mut std_buf, n).unwrap();
                let async_buf = block_on(async {
                    let mut w = FromTokio::new(Cursor::new(Vec::new()));
                    w.$method::<LittleEndian>(n).await.unwrap();
                    w.into_inner().into_inner()
                });
                assert_eq!(std_buf, async_buf);
            }
        }
    };
}

// =======================================================================
// Read tests
// =======================================================================

mod read {
    use super::*;

    #[cfg(not(miri))]
    #[test]
    fn read_u8() {
        fn prop(data: Vec<u8>) -> bool {
            if data.is_empty() {
                return true;
            }
            let d = &data[..1];
            let std_val = StdRead::read_u8(&mut Cursor::new(d)).unwrap();
            let async_val = block_on(async {
                AsyncRead::read_u8(&mut FromTokio::new(Cursor::new(d)))
                    .await
                    .unwrap()
            });
            std_val == async_val
        }
        qc(prop as fn(Vec<u8>) -> bool);
    }

    #[cfg(not(miri))]
    #[test]
    fn read_i8() {
        fn prop(data: Vec<u8>) -> bool {
            if data.is_empty() {
                return true;
            }
            let d = &data[..1];
            let std_val = StdRead::read_i8(&mut Cursor::new(d)).unwrap();
            let async_val = block_on(async {
                AsyncRead::read_i8(&mut FromTokio::new(Cursor::new(d)))
                    .await
                    .unwrap()
            });
            std_val == async_val
        }
        qc(prop as fn(Vec<u8>) -> bool);
    }

    qc_read!(read_u16, read_u16, 2);
    qc_read!(read_i16, read_i16, 2);
    qc_read!(read_u24, read_u24, 3);
    qc_read!(read_i24, read_i24, 3);
    qc_read!(read_u32, read_u32, 4);
    qc_read!(read_i32, read_i32, 4);
    qc_read!(read_u48, read_u48, 6);
    qc_read!(read_i48, read_i48, 6);
    qc_read!(read_u64, read_u64, 8);
    qc_read!(read_i64, read_i64, 8);

    test_read_128!(
        read_u128,
        read_u128,
        [
            0x00, 0x03, 0x43, 0x95, 0x4d, 0x60, 0x86, 0x83, 0x00, 0x03, 0x43,
            0x95, 0x4d, 0x60, 0x86, 0x83
        ]
    );
    test_read_128!(
        read_i128,
        read_i128,
        [
            0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x01
        ]
    );

    qc_read_float!(read_f32, read_f32, 4, f32);
    qc_read_float!(read_f64, read_f64, 8, f64);

    qc_read_nbytes!(read_uint, read_uint, 8);
    qc_read_nbytes!(read_int, read_int, 8);

    mod read_uint128 {
        use super::*;

        #[test]
        fn selected_lengths() {
            let data: &[u8] = &[
                0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a,
                0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10,
            ];
            for nbytes in 1..=16 {
                let d = &data[..nbytes];
                let std_val =
                    Cursor::new(d).read_uint128::<BigEndian>(nbytes).unwrap();
                let async_val = block_on(async {
                    FromTokio::new(Cursor::new(d))
                        .read_uint128::<BigEndian>(nbytes)
                        .await
                        .unwrap()
                });
                assert_eq!(
                    std_val, async_val,
                    "read_uint128 BE nbytes={nbytes}"
                );

                let std_val = Cursor::new(d)
                    .read_int128::<LittleEndian>(nbytes)
                    .unwrap();
                let async_val = block_on(async {
                    FromTokio::new(Cursor::new(d))
                        .read_int128::<LittleEndian>(nbytes)
                        .await
                        .unwrap()
                });
                assert_eq!(
                    std_val, async_val,
                    "read_int128 LE nbytes={nbytes}"
                );
            }
        }
    }

    test_read_into!(
        read_u16_into,
        read_u16_into,
        u16,
        4,
        [0, 1, 2, 3, 4, 5, 6, 7]
    );
    test_read_into!(
        read_u32_into,
        read_u32_into,
        u32,
        2,
        [0, 1, 2, 3, 4, 5, 6, 7]
    );
    test_read_into!(
        read_u64_into,
        read_u64_into,
        u64,
        2,
        [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]
    );
    test_read_into!(
        read_u128_into,
        read_u128_into,
        u128,
        1,
        [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]
    );
    test_read_into_no_endian!(
        read_i8_into,
        read_i8_into,
        i8,
        4,
        [0x01, 0x80, 0xff, 0x7f]
    );
    test_read_into!(
        read_i16_into,
        read_i16_into,
        i16,
        4,
        [0, 1, 0x80, 3, 0xff, 5, 6, 7]
    );
    test_read_into!(
        read_i32_into,
        read_i32_into,
        i32,
        2,
        [0, 1, 2, 3, 0x80, 5, 6, 7]
    );
    test_read_into!(
        read_i64_into,
        read_i64_into,
        i64,
        2,
        [0, 1, 2, 3, 4, 5, 6, 7, 0x80, 9, 10, 11, 12, 13, 14, 15]
    );
    test_read_into!(
        read_i128_into,
        read_i128_into,
        i128,
        1,
        [0x80, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]
    );

    mod read_f32_into {
        use super::*;

        #[test]
        fn big_endian() {
            let data: &[u8] =
                &[0x40, 0x49, 0x0f, 0xdb, 0x3f, 0x80, 0x00, 0x00];
            let mut std_dst = [0.0f32; 2];
            let mut async_dst = [0.0f32; 2];
            Cursor::new(data)
                .read_f32_into::<BigEndian>(&mut std_dst)
                .unwrap();
            block_on(async {
                FromTokio::new(Cursor::new(data))
                    .read_f32_into::<BigEndian>(&mut async_dst)
                    .await
                    .unwrap();
            });
            assert_eq!(std_dst.map(f32::to_bits), async_dst.map(f32::to_bits));
        }
    }

    mod read_f64_into {
        use super::*;

        #[test]
        fn big_endian() {
            let data: &[u8] = &[
                0x40, 0x09, 0x21, 0xfb, 0x54, 0x44, 0x2d, 0x18, 0x3f, 0xf0,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            ];
            let mut std_dst = [0.0f64; 2];
            let mut async_dst = [0.0f64; 2];
            Cursor::new(data)
                .read_f64_into::<BigEndian>(&mut std_dst)
                .unwrap();
            block_on(async {
                FromTokio::new(Cursor::new(data))
                    .read_f64_into::<BigEndian>(&mut async_dst)
                    .await
                    .unwrap();
            });
            assert_eq!(std_dst.map(f64::to_bits), async_dst.map(f64::to_bits));
        }
    }
}

// =======================================================================
// Write tests
// =======================================================================

mod write {
    use super::*;

    #[cfg(not(miri))]
    #[test]
    fn write_u8() {
        fn prop(n: u8) -> bool {
            let mut std_buf = vec![];
            StdWrite::write_u8(&mut std_buf, n).unwrap();
            let async_buf = block_on(async {
                let mut w = FromTokio::new(Cursor::new(Vec::new()));
                w.write_u8(n).await.unwrap();
                w.into_inner().into_inner()
            });
            std_buf == async_buf
        }
        qc(prop as fn(u8) -> bool);
    }

    #[cfg(not(miri))]
    #[test]
    fn write_i8() {
        fn prop(n: i8) -> bool {
            let mut std_buf = vec![];
            StdWrite::write_i8(&mut std_buf, n).unwrap();
            let async_buf = block_on(async {
                let mut w = FromTokio::new(Cursor::new(Vec::new()));
                w.write_i8(n).await.unwrap();
                w.into_inner().into_inner()
            });
            std_buf == async_buf
        }
        qc(prop as fn(i8) -> bool);
    }

    qc_write!(write_u16, u16, write_u16);
    qc_write!(write_i16, i16, write_i16);
    qc_write!(write_u32, u32, write_u32);
    qc_write!(write_i32, i32, write_i32);
    qc_write!(write_u64, u64, write_u64);
    qc_write!(write_i64, i64, write_i64);
    qc_write!(write_u24, u32, write_u24);
    qc_write!(write_i24, i32, write_i24);
    qc_write!(write_u48, u64, write_u48);
    qc_write!(write_i48, i64, write_i48);

    test_write_128!(
        write_u128,
        u128,
        write_u128,
        0x0003_4395_4d60_8683_0003_4395_4d60_8683
    );
    test_write_128!(write_i128, i128, write_i128, i128::MIN + 1);

    qc_write_float!(write_f32, f32, write_f32);
    qc_write_float!(write_f64, f64, write_f64);

    qc_write_nbytes!(write_uint, u64, write_uint, 8);
    qc_write_nbytes!(write_int, i64, write_int, 8);

    mod write_uint128 {
        use super::*;

        #[test]
        fn selected_lengths() {
            let val: u128 = 0x0102_0304_0506_0708_090a_0b0c_0d0e_0f10;
            for nbytes in 1..=16 {
                let mask: u128 = if nbytes >= 16 {
                    u128::MAX
                } else {
                    (1u128 << (nbytes * 8)) - 1
                };
                let n = val & mask;

                let mut std_buf = vec![];
                StdWrite::write_uint128::<BigEndian>(&mut std_buf, n, nbytes)
                    .unwrap();
                let async_buf = block_on(async {
                    let mut w = FromTokio::new(Cursor::new(Vec::new()));
                    w.write_uint128::<BigEndian>(n, nbytes).await.unwrap();
                    w.into_inner().into_inner()
                });
                assert_eq!(
                    std_buf, async_buf,
                    "write_uint128 BE nbytes={nbytes}"
                );
            }
        }
    }

    mod write_int128 {
        use super::*;

        #[test]
        fn selected_lengths() {
            for nbytes in 1..=16 {
                let n: i128 = -1;

                let mut std_buf = vec![];
                StdWrite::write_int128::<BigEndian>(&mut std_buf, n, nbytes)
                    .unwrap();
                let async_buf = block_on(async {
                    let mut w = FromTokio::new(Cursor::new(Vec::new()));
                    w.write_int128::<BigEndian>(n, nbytes).await.unwrap();
                    w.into_inner().into_inner()
                });
                assert_eq!(
                    std_buf, async_buf,
                    "write_int128 BE nbytes={nbytes}"
                );
            }
        }
    }
}

// =======================================================================
// Round-trip: write via std, read via async (and vice versa)
// =======================================================================

mod round_trip {
    use super::*;

    macro_rules! qc_round_trip {
        ($mod:ident, $ty:ty, $write:ident, $read:ident, $size:expr) => {
            mod $mod {
                use super::*;

                #[cfg(not(miri))]
                #[test]
                fn std_write_async_read_be() {
                    fn prop(n: $ty) -> bool {
                        let mut buf = vec![];
                        StdWrite::$write::<BigEndian>(&mut buf, n).unwrap();
                        let got = block_on(async {
                            FromTokio::new(Cursor::new(&buf[..]))
                                .$read::<BigEndian>()
                                .await
                                .unwrap()
                        });
                        n == got
                    }
                    qc(prop as fn($ty) -> bool);
                }

                #[cfg(not(miri))]
                #[test]
                fn async_write_std_read_be() {
                    fn prop(n: $ty) -> bool {
                        let buf = block_on(async {
                            let mut w =
                                FromTokio::new(Cursor::new(Vec::new()));
                            w.$write::<BigEndian>(n).await.unwrap();
                            w.into_inner().into_inner()
                        });
                        let got = Cursor::new(&buf[..])
                            .$read::<BigEndian>()
                            .unwrap();
                        n == got
                    }
                    qc(prop as fn($ty) -> bool);
                }

                #[cfg(not(miri))]
                #[test]
                fn std_write_async_read_le() {
                    fn prop(n: $ty) -> bool {
                        let mut buf = vec![];
                        StdWrite::$write::<LittleEndian>(&mut buf, n).unwrap();
                        let got = block_on(async {
                            FromTokio::new(Cursor::new(&buf[..]))
                                .$read::<LittleEndian>()
                                .await
                                .unwrap()
                        });
                        n == got
                    }
                    qc(prop as fn($ty) -> bool);
                }
            }
        };
    }

    qc_round_trip!(u16, u16, write_u16, read_u16, 2);
    qc_round_trip!(i16, i16, write_i16, read_i16, 2);
    qc_round_trip!(u32, u32, write_u32, read_u32, 4);
    qc_round_trip!(i32, i32, write_i32, read_i32, 4);
    qc_round_trip!(u64, u64, write_u64, read_u64, 8);
    qc_round_trip!(i64, i64, write_i64, read_i64, 8);
}
