// Async fn in traits is intentional here — we follow the same pattern as
// embedded-io-async itself and do not require Send bounds.
#![allow(async_fn_in_trait)]

use embedded_io::ReadExactError;
use embedded_io_async::{Read, Write};

use crate::ByteOrder;

/// Extends async [`Read`] with methods for reading numbers. (For `embedded-io-async`.)
///
/// Most of the methods defined here have an unconstrained type parameter that
/// must be explicitly instantiated. Typically, it is instantiated with either
/// the [`BigEndian`] or [`LittleEndian`] types defined in this crate.
///
/// # Examples
///
/// Read unsigned 16 bit big-endian integers from an async [`Read`]:
///
/// ```rust,no_run
/// # async fn example() {
/// use byteorder_embedded_io::BigEndian;
/// use byteorder_embedded_io::eio_async::ReadBytesExt;
///
/// let mut rdr = &[2u8, 5, 3, 0][..];
/// assert_eq!(517, rdr.read_u16::<BigEndian>().await.unwrap());
/// assert_eq!(768, rdr.read_u16::<BigEndian>().await.unwrap());
/// # }
/// ```
///
/// [`BigEndian`]: enum.BigEndian.html
/// [`LittleEndian`]: enum.LittleEndian.html
/// [`Read`]: https://docs.rs/embedded-io-async/0.7/embedded_io_async/trait.Read.html
pub trait ReadBytesExt: Read {
    /// Reads an unsigned 8 bit integer from the underlying reader.
    ///
    /// Note that since this reads a single byte, no byte order conversions
    /// are used. It is included for completeness.
    ///
    /// # Errors
    ///
    /// This method returns the same errors as [`Read::read_exact`].
    ///
    /// [`Read::read_exact`]: https://docs.rs/embedded-io-async/0.7/embedded_io_async/trait.Read.html#method.read_exact
    ///
    /// # Examples
    ///
    /// Read unsigned 8 bit integers from an async `Read`:
    ///
    /// ```rust,no_run
    /// # async fn example() {
    /// use byteorder_embedded_io::eio_async::ReadBytesExt;
    ///
    /// let mut rdr = &[2u8, 5][..];
    /// assert_eq!(2, rdr.read_u8().await.unwrap());
    /// assert_eq!(5, rdr.read_u8().await.unwrap());
    /// # }
    /// ```
    async fn read_u8(&mut self) -> Result<u8, ReadExactError<Self::Error>> {
        let mut buf = [0; 1];
        self.read_exact(&mut buf).await?;
        Ok(buf[0])
    }

    /// Reads a signed 8 bit integer from the underlying reader.
    ///
    /// Note that since this reads a single byte, no byte order conversions
    /// are used. It is included for completeness.
    ///
    /// # Errors
    ///
    /// This method returns the same errors as [`Read::read_exact`].
    ///
    /// [`Read::read_exact`]: https://docs.rs/embedded-io-async/0.7/embedded_io_async/trait.Read.html#method.read_exact
    ///
    /// # Examples
    ///
    /// Read signed 8 bit integers from an async `Read`:
    ///
    /// ```rust,no_run
    /// # async fn example() {
    /// use byteorder_embedded_io::eio_async::ReadBytesExt;
    ///
    /// let mut rdr = &[0x02u8, 0xfb][..];
    /// assert_eq!(2, rdr.read_i8().await.unwrap());
    /// assert_eq!(-5, rdr.read_i8().await.unwrap());
    /// # }
    /// ```
    #[allow(clippy::cast_possible_wrap)]
    async fn read_i8(&mut self) -> Result<i8, ReadExactError<Self::Error>> {
        let mut buf = [0; 1];
        self.read_exact(&mut buf).await?;
        Ok(buf[0] as i8)
    }

    /// Reads an unsigned 16 bit integer from the underlying reader.
    ///
    /// # Errors
    ///
    /// This method returns the same errors as [`Read::read_exact`].
    ///
    /// [`Read::read_exact`]: https://docs.rs/embedded-io-async/0.7/embedded_io_async/trait.Read.html#method.read_exact
    ///
    /// # Examples
    ///
    /// Read unsigned 16 bit big-endian integers from an async `Read`:
    ///
    /// ```rust,no_run
    /// # async fn example() {
    /// use byteorder_embedded_io::{BigEndian, eio_async::ReadBytesExt};
    ///
    /// let mut rdr = &[2u8, 5, 3, 0][..];
    /// assert_eq!(517, rdr.read_u16::<BigEndian>().await.unwrap());
    /// assert_eq!(768, rdr.read_u16::<BigEndian>().await.unwrap());
    /// # }
    /// ```
    async fn read_u16<T: ByteOrder>(
        &mut self,
    ) -> Result<u16, ReadExactError<Self::Error>> {
        let mut buf = [0; 2];
        self.read_exact(&mut buf).await?;
        Ok(T::read_u16(&buf))
    }

    /// Reads a signed 16 bit integer from the underlying reader.
    ///
    /// # Errors
    ///
    /// This method returns the same errors as [`Read::read_exact`].
    ///
    /// [`Read::read_exact`]: https://docs.rs/embedded-io-async/0.7/embedded_io_async/trait.Read.html#method.read_exact
    ///
    /// # Examples
    ///
    /// Read signed 16 bit big-endian integers from an async `Read`:
    ///
    /// ```rust,no_run
    /// # async fn example() {
    /// use byteorder_embedded_io::{BigEndian, eio_async::ReadBytesExt};
    ///
    /// let mut rdr = &[0x00u8, 0xc1, 0xff, 0x7c][..];
    /// assert_eq!(193, rdr.read_i16::<BigEndian>().await.unwrap());
    /// assert_eq!(-132, rdr.read_i16::<BigEndian>().await.unwrap());
    /// # }
    /// ```
    async fn read_i16<T: ByteOrder>(
        &mut self,
    ) -> Result<i16, ReadExactError<Self::Error>> {
        let mut buf = [0; 2];
        self.read_exact(&mut buf).await?;
        Ok(T::read_i16(&buf))
    }

    /// Reads an unsigned 24 bit integer from the underlying reader.
    async fn read_u24<T: ByteOrder>(
        &mut self,
    ) -> Result<u32, ReadExactError<Self::Error>> {
        let mut buf = [0; 3];
        self.read_exact(&mut buf).await?;
        Ok(T::read_u24(&buf))
    }

    /// Reads a signed 24 bit integer from the underlying reader.
    async fn read_i24<T: ByteOrder>(
        &mut self,
    ) -> Result<i32, ReadExactError<Self::Error>> {
        let mut buf = [0; 3];
        self.read_exact(&mut buf).await?;
        Ok(T::read_i24(&buf))
    }

    /// Reads an unsigned 32 bit integer from the underlying reader.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # async fn example() {
    /// use byteorder_embedded_io::{BigEndian, eio_async::ReadBytesExt};
    ///
    /// let mut rdr = &[0x00u8, 0x00, 0x01, 0x0b][..];
    /// assert_eq!(267, rdr.read_u32::<BigEndian>().await.unwrap());
    /// # }
    /// ```
    async fn read_u32<T: ByteOrder>(
        &mut self,
    ) -> Result<u32, ReadExactError<Self::Error>> {
        let mut buf = [0; 4];
        self.read_exact(&mut buf).await?;
        Ok(T::read_u32(&buf))
    }

    /// Reads a signed 32 bit integer from the underlying reader.
    async fn read_i32<T: ByteOrder>(
        &mut self,
    ) -> Result<i32, ReadExactError<Self::Error>> {
        let mut buf = [0; 4];
        self.read_exact(&mut buf).await?;
        Ok(T::read_i32(&buf))
    }

    /// Reads an unsigned 48 bit integer from the underlying reader.
    async fn read_u48<T: ByteOrder>(
        &mut self,
    ) -> Result<u64, ReadExactError<Self::Error>> {
        let mut buf = [0; 6];
        self.read_exact(&mut buf).await?;
        Ok(T::read_u48(&buf))
    }

    /// Reads a signed 48 bit integer from the underlying reader.
    async fn read_i48<T: ByteOrder>(
        &mut self,
    ) -> Result<i64, ReadExactError<Self::Error>> {
        let mut buf = [0; 6];
        self.read_exact(&mut buf).await?;
        Ok(T::read_i48(&buf))
    }

    /// Reads an unsigned 64 bit integer from the underlying reader.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # async fn example() {
    /// use byteorder_embedded_io::{BigEndian, eio_async::ReadBytesExt};
    ///
    /// let mut rdr = &[0x00u8, 0x03, 0x43, 0x95, 0x4d, 0x60, 0x86, 0x83][..];
    /// assert_eq!(918733457491587, rdr.read_u64::<BigEndian>().await.unwrap());
    /// # }
    /// ```
    async fn read_u64<T: ByteOrder>(
        &mut self,
    ) -> Result<u64, ReadExactError<Self::Error>> {
        let mut buf = [0; 8];
        self.read_exact(&mut buf).await?;
        Ok(T::read_u64(&buf))
    }

    /// Reads a signed 64 bit integer from the underlying reader.
    async fn read_i64<T: ByteOrder>(
        &mut self,
    ) -> Result<i64, ReadExactError<Self::Error>> {
        let mut buf = [0; 8];
        self.read_exact(&mut buf).await?;
        Ok(T::read_i64(&buf))
    }

    /// Reads an unsigned 128 bit integer from the underlying reader.
    async fn read_u128<T: ByteOrder>(
        &mut self,
    ) -> Result<u128, ReadExactError<Self::Error>> {
        let mut buf = [0; 16];
        self.read_exact(&mut buf).await?;
        Ok(T::read_u128(&buf))
    }

    /// Reads a signed 128 bit integer from the underlying reader.
    async fn read_i128<T: ByteOrder>(
        &mut self,
    ) -> Result<i128, ReadExactError<Self::Error>> {
        let mut buf = [0; 16];
        self.read_exact(&mut buf).await?;
        Ok(T::read_i128(&buf))
    }

    /// Reads an unsigned n-bytes integer from the underlying reader.
    async fn read_uint<T: ByteOrder>(
        &mut self,
        nbytes: usize,
    ) -> Result<u64, ReadExactError<Self::Error>> {
        let mut buf = [0; 8];
        self.read_exact(&mut buf[..nbytes]).await?;
        Ok(T::read_uint(&buf[..nbytes], nbytes))
    }

    /// Reads a signed n-bytes integer from the underlying reader.
    async fn read_int<T: ByteOrder>(
        &mut self,
        nbytes: usize,
    ) -> Result<i64, ReadExactError<Self::Error>> {
        let mut buf = [0; 8];
        self.read_exact(&mut buf[..nbytes]).await?;
        Ok(T::read_int(&buf[..nbytes], nbytes))
    }

    /// Reads an unsigned n-bytes integer from the underlying reader.
    async fn read_uint128<T: ByteOrder>(
        &mut self,
        nbytes: usize,
    ) -> Result<u128, ReadExactError<Self::Error>> {
        let mut buf = [0; 16];
        self.read_exact(&mut buf[..nbytes]).await?;
        Ok(T::read_uint128(&buf[..nbytes], nbytes))
    }

    /// Reads a signed n-bytes integer from the underlying reader.
    async fn read_int128<T: ByteOrder>(
        &mut self,
        nbytes: usize,
    ) -> Result<i128, ReadExactError<Self::Error>> {
        let mut buf = [0; 16];
        self.read_exact(&mut buf[..nbytes]).await?;
        Ok(T::read_int128(&buf[..nbytes], nbytes))
    }

    /// Reads a IEEE754 single-precision (4 bytes) floating point number.
    async fn read_f32<T: ByteOrder>(
        &mut self,
    ) -> Result<f32, ReadExactError<Self::Error>> {
        let mut buf = [0; 4];
        self.read_exact(&mut buf).await?;
        Ok(T::read_f32(&buf))
    }

    /// Reads a IEEE754 double-precision (8 bytes) floating point number.
    async fn read_f64<T: ByteOrder>(
        &mut self,
    ) -> Result<f64, ReadExactError<Self::Error>> {
        let mut buf = [0; 8];
        self.read_exact(&mut buf).await?;
        Ok(T::read_f64(&buf))
    }

    /// Reads a sequence of unsigned 16 bit integers.
    async fn read_u16_into<T: ByteOrder>(
        &mut self,
        dst: &mut [u16],
    ) -> Result<(), ReadExactError<Self::Error>> {
        {
            let buf = unsafe { slice_to_u8_mut(dst) };
            self.read_exact(buf).await?;
        }
        T::from_slice_u16(dst);
        Ok(())
    }

    /// Reads a sequence of unsigned 32 bit integers.
    async fn read_u32_into<T: ByteOrder>(
        &mut self,
        dst: &mut [u32],
    ) -> Result<(), ReadExactError<Self::Error>> {
        {
            let buf = unsafe { slice_to_u8_mut(dst) };
            self.read_exact(buf).await?;
        }
        T::from_slice_u32(dst);
        Ok(())
    }

    /// Reads a sequence of unsigned 64 bit integers.
    async fn read_u64_into<T: ByteOrder>(
        &mut self,
        dst: &mut [u64],
    ) -> Result<(), ReadExactError<Self::Error>> {
        {
            let buf = unsafe { slice_to_u8_mut(dst) };
            self.read_exact(buf).await?;
        }
        T::from_slice_u64(dst);
        Ok(())
    }

    /// Reads a sequence of unsigned 128 bit integers.
    async fn read_u128_into<T: ByteOrder>(
        &mut self,
        dst: &mut [u128],
    ) -> Result<(), ReadExactError<Self::Error>> {
        {
            let buf = unsafe { slice_to_u8_mut(dst) };
            self.read_exact(buf).await?;
        }
        T::from_slice_u128(dst);
        Ok(())
    }

    /// Reads a sequence of signed 8 bit integers.
    async fn read_i8_into(
        &mut self,
        dst: &mut [i8],
    ) -> Result<(), ReadExactError<Self::Error>> {
        let buf = unsafe { slice_to_u8_mut(dst) };
        self.read_exact(buf).await
    }

    /// Reads a sequence of signed 16 bit integers.
    async fn read_i16_into<T: ByteOrder>(
        &mut self,
        dst: &mut [i16],
    ) -> Result<(), ReadExactError<Self::Error>> {
        {
            let buf = unsafe { slice_to_u8_mut(dst) };
            self.read_exact(buf).await?;
        }
        T::from_slice_i16(dst);
        Ok(())
    }

    /// Reads a sequence of signed 32 bit integers.
    async fn read_i32_into<T: ByteOrder>(
        &mut self,
        dst: &mut [i32],
    ) -> Result<(), ReadExactError<Self::Error>> {
        {
            let buf = unsafe { slice_to_u8_mut(dst) };
            self.read_exact(buf).await?;
        }
        T::from_slice_i32(dst);
        Ok(())
    }

    /// Reads a sequence of signed 64 bit integers.
    async fn read_i64_into<T: ByteOrder>(
        &mut self,
        dst: &mut [i64],
    ) -> Result<(), ReadExactError<Self::Error>> {
        {
            let buf = unsafe { slice_to_u8_mut(dst) };
            self.read_exact(buf).await?;
        }
        T::from_slice_i64(dst);
        Ok(())
    }

    /// Reads a sequence of signed 128 bit integers.
    async fn read_i128_into<T: ByteOrder>(
        &mut self,
        dst: &mut [i128],
    ) -> Result<(), ReadExactError<Self::Error>> {
        {
            let buf = unsafe { slice_to_u8_mut(dst) };
            self.read_exact(buf).await?;
        }
        T::from_slice_i128(dst);
        Ok(())
    }

    /// Reads a sequence of IEEE754 single-precision floating point numbers.
    async fn read_f32_into<T: ByteOrder>(
        &mut self,
        dst: &mut [f32],
    ) -> Result<(), ReadExactError<Self::Error>> {
        {
            let buf = unsafe { slice_to_u8_mut(dst) };
            self.read_exact(buf).await?;
        }
        T::from_slice_f32(dst);
        Ok(())
    }

    /// Reads a sequence of IEEE754 double-precision floating point numbers.
    async fn read_f64_into<T: ByteOrder>(
        &mut self,
        dst: &mut [f64],
    ) -> Result<(), ReadExactError<Self::Error>> {
        {
            let buf = unsafe { slice_to_u8_mut(dst) };
            self.read_exact(buf).await?;
        }
        T::from_slice_f64(dst);
        Ok(())
    }
}

/// All types that implement async `Read` get methods defined in
/// `ReadBytesExt` for free.
impl<R: Read + ?Sized> ReadBytesExt for R {}

/// Extends async [`Write`] with methods for writing numbers. (For `embedded-io-async`.)
///
/// Most of the methods defined here have an unconstrained type parameter that
/// must be explicitly instantiated. Typically, it is instantiated with either
/// the [`BigEndian`] or [`LittleEndian`] types defined in this crate.
///
/// # Examples
///
/// Write unsigned 16 bit big-endian integers to an async [`Write`]:
///
/// ```rust,no_run
/// # async fn example() {
/// use byteorder_embedded_io::BigEndian;
/// use byteorder_embedded_io::eio_async::WriteBytesExt;
///
/// let mut buf = [0u8; 4];
/// {
///     let mut wtr = &mut buf[..];
///     wtr.write_u16::<BigEndian>(517).await.unwrap();
///     wtr.write_u16::<BigEndian>(768).await.unwrap();
/// }
/// assert_eq!(buf, [2, 5, 3, 0]);
/// # }
/// ```
///
/// [`BigEndian`]: enum.BigEndian.html
/// [`LittleEndian`]: enum.LittleEndian.html
/// [`Write`]: https://docs.rs/embedded-io-async/0.7/embedded_io_async/trait.Write.html
pub trait WriteBytesExt: Write {
    /// Writes an unsigned 8 bit integer to the underlying writer.
    async fn write_u8(&mut self, n: u8) -> Result<(), Self::Error> {
        self.write_all(&[n]).await
    }

    /// Writes a signed 8 bit integer to the underlying writer.
    #[allow(clippy::cast_sign_loss)]
    async fn write_i8(&mut self, n: i8) -> Result<(), Self::Error> {
        self.write_all(&[n as u8]).await
    }

    /// Writes an unsigned 16 bit integer to the underlying writer.
    async fn write_u16<T: ByteOrder>(
        &mut self,
        n: u16,
    ) -> Result<(), Self::Error> {
        let mut buf = [0; 2];
        T::write_u16(&mut buf, n);
        self.write_all(&buf).await
    }

    /// Writes a signed 16 bit integer to the underlying writer.
    async fn write_i16<T: ByteOrder>(
        &mut self,
        n: i16,
    ) -> Result<(), Self::Error> {
        let mut buf = [0; 2];
        T::write_i16(&mut buf, n);
        self.write_all(&buf).await
    }

    /// Writes an unsigned 24 bit integer to the underlying writer.
    async fn write_u24<T: ByteOrder>(
        &mut self,
        n: u32,
    ) -> Result<(), Self::Error> {
        let mut buf = [0; 3];
        T::write_u24(&mut buf, n);
        self.write_all(&buf).await
    }

    /// Writes a signed 24 bit integer to the underlying writer.
    async fn write_i24<T: ByteOrder>(
        &mut self,
        n: i32,
    ) -> Result<(), Self::Error> {
        let mut buf = [0; 3];
        T::write_i24(&mut buf, n);
        self.write_all(&buf).await
    }

    /// Writes an unsigned 32 bit integer to the underlying writer.
    async fn write_u32<T: ByteOrder>(
        &mut self,
        n: u32,
    ) -> Result<(), Self::Error> {
        let mut buf = [0; 4];
        T::write_u32(&mut buf, n);
        self.write_all(&buf).await
    }

    /// Writes a signed 32 bit integer to the underlying writer.
    async fn write_i32<T: ByteOrder>(
        &mut self,
        n: i32,
    ) -> Result<(), Self::Error> {
        let mut buf = [0; 4];
        T::write_i32(&mut buf, n);
        self.write_all(&buf).await
    }

    /// Writes an unsigned 48 bit integer to the underlying writer.
    async fn write_u48<T: ByteOrder>(
        &mut self,
        n: u64,
    ) -> Result<(), Self::Error> {
        let mut buf = [0; 6];
        T::write_u48(&mut buf, n);
        self.write_all(&buf).await
    }

    /// Writes a signed 48 bit integer to the underlying writer.
    async fn write_i48<T: ByteOrder>(
        &mut self,
        n: i64,
    ) -> Result<(), Self::Error> {
        let mut buf = [0; 6];
        T::write_i48(&mut buf, n);
        self.write_all(&buf).await
    }

    /// Writes an unsigned 64 bit integer to the underlying writer.
    async fn write_u64<T: ByteOrder>(
        &mut self,
        n: u64,
    ) -> Result<(), Self::Error> {
        let mut buf = [0; 8];
        T::write_u64(&mut buf, n);
        self.write_all(&buf).await
    }

    /// Writes a signed 64 bit integer to the underlying writer.
    async fn write_i64<T: ByteOrder>(
        &mut self,
        n: i64,
    ) -> Result<(), Self::Error> {
        let mut buf = [0; 8];
        T::write_i64(&mut buf, n);
        self.write_all(&buf).await
    }

    /// Writes an unsigned 128 bit integer to the underlying writer.
    async fn write_u128<T: ByteOrder>(
        &mut self,
        n: u128,
    ) -> Result<(), Self::Error> {
        let mut buf = [0; 16];
        T::write_u128(&mut buf, n);
        self.write_all(&buf).await
    }

    /// Writes a signed 128 bit integer to the underlying writer.
    async fn write_i128<T: ByteOrder>(
        &mut self,
        n: i128,
    ) -> Result<(), Self::Error> {
        let mut buf = [0; 16];
        T::write_i128(&mut buf, n);
        self.write_all(&buf).await
    }

    /// Writes an unsigned n-bytes integer to the underlying writer.
    ///
    /// # Panics
    ///
    /// If the given integer is not representable in the given number of
    /// bytes, this method panics. If `nbytes > 8`, this method panics.
    async fn write_uint<T: ByteOrder>(
        &mut self,
        n: u64,
        nbytes: usize,
    ) -> Result<(), Self::Error> {
        let mut buf = [0; 8];
        T::write_uint(&mut buf, n, nbytes);
        self.write_all(&buf[0..nbytes]).await
    }

    /// Writes a signed n-bytes integer to the underlying writer.
    ///
    /// # Panics
    ///
    /// If the given integer is not representable in the given number of
    /// bytes, this method panics. If `nbytes > 8`, this method panics.
    async fn write_int<T: ByteOrder>(
        &mut self,
        n: i64,
        nbytes: usize,
    ) -> Result<(), Self::Error> {
        let mut buf = [0; 8];
        T::write_int(&mut buf, n, nbytes);
        self.write_all(&buf[0..nbytes]).await
    }

    /// Writes an unsigned n-bytes integer to the underlying writer.
    ///
    /// If `nbytes > 16`, this method panics.
    async fn write_uint128<T: ByteOrder>(
        &mut self,
        n: u128,
        nbytes: usize,
    ) -> Result<(), Self::Error> {
        let mut buf = [0; 16];
        T::write_uint128(&mut buf, n, nbytes);
        self.write_all(&buf[0..nbytes]).await
    }

    /// Writes a signed n-bytes integer to the underlying writer.
    ///
    /// If `nbytes > 16`, this method panics.
    async fn write_int128<T: ByteOrder>(
        &mut self,
        n: i128,
        nbytes: usize,
    ) -> Result<(), Self::Error> {
        let mut buf = [0; 16];
        T::write_int128(&mut buf, n, nbytes);
        self.write_all(&buf[0..nbytes]).await
    }

    /// Writes a IEEE754 single-precision (4 bytes) floating point number.
    async fn write_f32<T: ByteOrder>(
        &mut self,
        n: f32,
    ) -> Result<(), Self::Error> {
        let mut buf = [0; 4];
        T::write_f32(&mut buf, n);
        self.write_all(&buf).await
    }

    /// Writes a IEEE754 double-precision (8 bytes) floating point number.
    async fn write_f64<T: ByteOrder>(
        &mut self,
        n: f64,
    ) -> Result<(), Self::Error> {
        let mut buf = [0; 8];
        T::write_f64(&mut buf, n);
        self.write_all(&buf).await
    }
}

/// All types that implement async `Write` get methods defined in
/// `WriteBytesExt` for free.
impl<W: Write + ?Sized> WriteBytesExt for W {}

/// Convert a slice of T (where T is plain old data) to its mutable binary
/// representation.
///
/// This function is wildly unsafe because it permits arbitrary modification
/// of the binary representation of any `Copy` type. Use with care. It's
/// intended to be called only where `T` is a numeric type.
unsafe fn slice_to_u8_mut<T: Copy>(slice: &mut [T]) -> &mut [u8] {
    let len = core::mem::size_of_val(slice);
    core::slice::from_raw_parts_mut(slice.as_mut_ptr().cast::<u8>(), len)
}
