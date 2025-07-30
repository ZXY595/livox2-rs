use super::length::DstLength; 
use zerocopy::{Immutable, KnownLayout}; 

/// Sized header describing the len and tag of the corresponding dynamic sized data.
pub trait DstHeader: KnownLayout + Immutable + Sized {
    type Tag;
    type LengthMeta: DstLength;

    fn data_len(&self) -> usize;
    fn data_tag(&self) -> Self::Tag;
}

impl<T> DstHeader for &T
where
    T: DstHeader,
{
    type Tag = T::Tag;
    type LengthMeta = T::LengthMeta;

    fn data_len(&self) -> usize {
        T::data_len(self)
    }

    fn data_tag(&self) -> Self::Tag {
        T::data_tag(self)
    }
}

impl<T> DstHeader for &mut T
where
    T: DstHeader,
{
    type Tag = T::Tag;
    type LengthMeta = T::LengthMeta;

    fn data_len(&self) -> usize {
        T::data_len(self)
    }

    fn data_tag(&self) -> Self::Tag {
        T::data_tag(self)
    }
}

