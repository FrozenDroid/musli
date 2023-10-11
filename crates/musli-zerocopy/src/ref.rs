use core::marker::PhantomData;

use crate::offset::{DefaultTargetSize, Offset, TargetSize};
use crate::ZeroCopy;

/// A sized reference.
///
/// This is used to type a pointer with a [`ZeroCopy`] parameter so that it can
/// be used in combination with [`Buf`] to load the value from a buffer.
///
/// Note that the constructor is safe, because alignment and validation checks
/// happens whenever a value is loaded from a bare buffer.
///
/// [`Buf`]: crate::buf::Buf
///
/// # Examples
///
/// ```
/// use core::mem::align_of;
/// use musli_zerocopy::{AlignedBuf, Ref, Offset};
///
/// let mut buf = AlignedBuf::with_alignment(align_of::<u32>());
/// buf.extend_from_slice(&[1, 2, 3, 4]);
///
/// let buf = buf.as_ref()?;
///
/// let number = Ref::<u32>::new(Offset::ZERO);
/// assert_eq!(*buf.load(number)?, u32::from_ne_bytes([1, 2, 3, 4]));
/// # Ok::<_, musli_zerocopy::Error>(())
/// ```
#[derive(Debug, ZeroCopy)]
#[repr(C)]
#[zero_copy(crate)]
pub struct Ref<T, O: TargetSize = DefaultTargetSize> {
    ptr: Offset<O>,
    #[zero_copy(ignore)]
    _marker: PhantomData<T>,
}

impl<T> Ref<T>
where
    T: ZeroCopy,
{
    /// Construct a typed reference to the first position in a buffer.
    pub const fn zero() -> Self {
        Self::new(Offset::ZERO)
    }
}

impl<T, O: TargetSize> Ref<T, O>
where
    T: ZeroCopy,
{
    /// Construct a reference wrapping the given type at the specified address.
    pub const fn new(ptr: Offset<O>) -> Self {
        Self {
            ptr,
            _marker: PhantomData,
        }
    }

    #[inline]
    pub(crate) fn ptr(&self) -> Offset<O> {
        self.ptr
    }
}

impl<T, O: TargetSize> Clone for Ref<T, O> {
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}

impl<T, O: TargetSize> Copy for Ref<T, O> {}
