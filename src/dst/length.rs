use std::marker::PhantomData;

use zerocopy::{Immutable, IntoBytes, KnownLayout, TryFromBytes};

pub trait DstLength {
    fn parse_ref_bytes<T>(bytes: &[u8], len: usize) -> Result<&[T], crate::Error>
    where
        T: Immutable + TryFromBytes;
    fn parse_mut_bytes<T>(bytes: &mut [u8], len: usize) -> Result<&mut [T], crate::Error>
    where
        T: KnownLayout + TryFromBytes + IntoBytes;
}

/// Indicates the [`DstHeader::data_len`] method
/// returns the length of the typed data.
pub enum TypedLen {}

/// Indicates the [`DstHeader::data_len`] method
/// returns the `[u8]` length of `(header + data)`,
///
/// H is the header type, use to calculate the header length.
pub struct UntypedLen<H>(PhantomData<H>);

impl DstLength for TypedLen {
    fn parse_ref_bytes<T>(bytes: &[u8], len: usize) -> Result<&[T], crate::Error>
    where
        T: Immutable + TryFromBytes,
    {
        <[T]>::try_ref_from_bytes_with_elems(bytes, len).map_err(From::from)
    }

    fn parse_mut_bytes<T>(bytes: &mut [u8], len: usize) -> Result<&mut [T], crate::Error>
    where
        T: KnownLayout + TryFromBytes + IntoBytes,
    {
        <[T]>::try_mut_from_bytes_with_elems(bytes, len).map_err(From::from)
    }
}

impl<H> DstLength for UntypedLen<H>
where
    H: Sized,
{
    fn parse_ref_bytes<T>(bytes: &[u8], len: usize) -> Result<&[T], crate::Error>
    where
        T: Immutable + TryFromBytes,
    {
        let len = len - std::mem::size_of::<H>();
        <[T]>::try_ref_from_bytes(&bytes[..len]).map_err(From::from)
    }

    fn parse_mut_bytes<T>(bytes: &mut [u8], len: usize) -> Result<&mut [T], crate::Error>
    where
        T: KnownLayout + TryFromBytes + IntoBytes,
    {
        let len = len - std::mem::size_of::<H>();
        <[T]>::try_mut_from_bytes(&mut bytes[..len]).map_err(From::from)
    }
}
