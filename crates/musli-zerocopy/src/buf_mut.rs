use crate::error::Error;
use crate::offset::TargetSize;
use crate::store_struct::StoreStruct;
use crate::zero_copy::ZeroCopy;

/// A mutable buffer to store zero copy types to.
///
/// This is implemented by [`AlignedBuf`].
///
/// [`AlignedBuf`]: crate::AlignedBuf
pub trait BufMut {
    /// Target size buffer is configured to use.
    type TargetSize: TargetSize;

    /// Interior mutable buffer.
    type StoreStruct<'a, T>: StoreStruct<T, Self::TargetSize>
    where
        Self: 'a,
        T: ZeroCopy;

    /// Extend the current buffer from the given slice.
    fn extend_from_slice(&mut self, bytes: &[u8]) -> Result<(), Error>;

    /// Write the given zero copy type to the buffer.
    fn store<T>(&mut self, value: &T) -> Result<(), Error>
    where
        T: ZeroCopy;

    /// Setup a writer for the given type.
    ///
    /// This API stores the type directly using an unaligned pointer store and
    /// just ensures that any padding is zeroed.
    ///
    /// # Safety
    ///
    /// While calling just this function is not unsafe, finishing writing with
    /// [`StoreStruct::finish`] is unsafe.
    ///
    /// # Examples
    ///
    /// ```
    /// use musli_zerocopy::{AlignedBuf, BufMut, StoreStruct, ZeroCopy};
    ///
    /// #[derive(Debug, PartialEq, Eq, ZeroCopy)]
    /// #[repr(C)]
    /// struct ZeroPadded {
    ///     a: u8,
    ///     b: u64,
    ///     c: u16,
    ///     d: u32,
    /// }
    ///
    /// let mut buf = AlignedBuf::new();
    ///
    /// let padded = ZeroPadded {
    ///     a: 0x01u8.to_be(),
    ///     b: 0x0203_0405_0607_0809u64.to_be(),
    ///     c: 0x0a0bu16.to_be(),
    ///     d: 0x0c0d_0e0fu32.to_be(),
    /// };
    ///
    /// let mut w = buf.store_struct(&padded);
    /// w.pad::<u8>();
    /// w.pad::<u64>();
    /// w.pad::<u16>();
    /// w.pad::<u32>();
    ///
    /// // SAFETY: We've asserted that the struct fields have been correctly padded.
    /// let ptr = unsafe { w.finish()? };
    ///
    /// // Note: The bytes are explicitly convert to big-endian encoding above.
    /// assert_eq!(buf.as_slice(), &[1, 0, 0, 0, 0, 0, 0, 0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 0, 0, 12, 13, 14, 15]);
    ///
    /// let buf = buf.as_aligned();
    ///
    /// assert_eq!(buf.load(ptr)?, &padded);
    /// # Ok::<_, musli_zerocopy::Error>(())
    /// ```
    fn store_struct<T>(&mut self, value: &T) -> Self::StoreStruct<'_, T>
    where
        T: ZeroCopy;
}

impl<B: ?Sized> BufMut for &mut B
where
    B: BufMut,
{
    type TargetSize = B::TargetSize;
    type StoreStruct<'a, T> = B::StoreStruct<'a, T> where Self: 'a, T: ZeroCopy;

    #[inline]
    fn extend_from_slice(&mut self, bytes: &[u8]) -> Result<(), Error> {
        (**self).extend_from_slice(bytes)
    }

    #[inline]
    fn store<T>(&mut self, value: &T) -> Result<(), Error>
    where
        T: ZeroCopy,
    {
        (**self).store(value)
    }

    #[inline]
    fn store_struct<T>(&mut self, value: &T) -> Self::StoreStruct<'_, T>
    where
        T: ZeroCopy,
    {
        (**self).store_struct(value)
    }
}
