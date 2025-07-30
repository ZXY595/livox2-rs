use std::marker::PhantomData;

use zerocopy::{IntoBytes, TryFromBytes};

use super::{DstDataMut, DstDataRef, DstHeader, DstRef};

impl<'a, H, D> TryFrom<&'a [u8]> for DstRef<'a, &'a H, D>
where
    H: DstHeader + TryFromBytes,
    D: DstDataRef<'a, Tag = H::Tag>,
{
    type Error = crate::Error;

    fn try_from(value: &'a [u8]) -> Result<Self, Self::Error> {
        let header_size = std::mem::size_of::<H>();

        let (header, tailing) = value
            .split_at_checked(header_size)
            .ok_or(crate::Error::invalid_size(header_size, value.len()))?;

        let header = H::try_ref_from_bytes(header)?;
        let data = D::ref_from_header_and_bytes(header, tailing)?;

        Ok(Self {
            header,
            data,
            lifetime: PhantomData,
        })
    }
}

impl<'a, H, D> TryFrom<&'a mut [u8]> for DstRef<'a, &'a mut H, D>
where
    H: DstHeader + TryFromBytes + IntoBytes,
    D: DstDataMut<'a, Tag = H::Tag>,
{
    type Error = crate::Error;

    fn try_from(value: &'a mut [u8]) -> Result<Self, Self::Error> {
        let header_size = std::mem::size_of::<H>();
        let bytes_len = value.len();

        let (header, tailing) = value
            .split_at_mut_checked(header_size)
            .ok_or(crate::Error::invalid_size(header_size, bytes_len))?;

        let header = H::try_mut_from_bytes(header)?;
        let data = D::mut_from_header_and_bytes(&header, tailing)?;

        Ok(Self {
            header,
            data,
            lifetime: PhantomData,
        })
    }
}
