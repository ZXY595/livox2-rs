use super::header::DstHeader;

pub trait DstData<'a>: Sized {
    type Tag: Copy;
    fn as_tag_and_bytes(&'a self) -> (Self::Tag, &'a [u8]);
}

pub trait DstDataRef<'a>: DstData<'a> {
    fn ref_from_header_and_bytes<H>(header: H, bytes: &'a [u8]) -> Result<Self, crate::Error>
    where
        H: DstHeader<Tag = Self::Tag>;
}

pub trait DstDataMut<'a>: DstData<'a> {
    fn mut_from_header_and_bytes<H>(header: H, bytes: &'a mut [u8]) -> Result<Self, crate::Error>
    where
        H: DstHeader<Tag = Self::Tag>;
}

