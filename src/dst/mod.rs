pub mod data;
pub mod header;
pub mod length;
#[macro_use]
pub mod macros;

use core::marker::PhantomData;
use data::{DstData, DstDataMut, DstDataRef};
use header::DstHeader;
use zerocopy::{Immutable, IntoBytes, KnownLayout, TryFromBytes};
mod impl_convert;
mod impl_io;

/// A type containing a header with a reference
/// to its dynamically sized data.
///
/// The header must implement the [`DstHeader`] to get the expected size of the data.
///
/// Thanks to the rust generics, the mutability of the reference is generic.
///
/// Use this type to avoid casting between raw Dst types and buffers using the zerocopy crate.
/// Which is hard to cast between them in one step because we don't know the size of the data
/// before the header is parsed.
#[derive(Debug, KnownLayout, Immutable)]
pub struct DstRef<'a, H, D>
where
    H: 'a,
    D: 'a,
{
    lifetime: PhantomData<&'a ()>,
    pub header: H,
    pub data: D,
}

impl<'a, H, D> DstRef<'a, &'a H, D>
where
    H: DstHeader + TryFromBytes,
    D: DstDataRef<'a, Tag = H::Tag>,
{
    pub fn try_ref_from_bytes(bytes: &'a [u8]) -> Result<Self, crate::Error> {
        Self::try_from(bytes)
    }
}

impl<'a, H, D> DstRef<'a, H, D>
where
    H: DstHeader,
    D: DstData<'a, Tag = H::Tag>,
{
    pub fn new(header: H, data: D) -> Self {
        Self {
            lifetime: PhantomData,
            header,
            data,
        }
    }
}

impl<'a, H, D> DstRef<'a, &'a mut H, D>
where
    H: DstHeader + TryFromBytes + IntoBytes,
    D: DstDataMut<'a, Tag = H::Tag>,
{
    pub fn try_mut_from_bytes(bytes: &'a mut [u8]) -> Result<Self, crate::Error> {
        Self::try_from(bytes)
    }
}
