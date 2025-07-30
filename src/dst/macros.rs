/// Macro for defining DST data structures.
/// # Example:
/// ```rust
/// use macro_rules_attribute::apply;
///
/// #[apply(DstData!)]
/// enum TestData {
///     ref_type = TestDataRef, // ignore when ref_type = None
///     mut_type = None, // ignore when mut_type = None
///     #[repr(C)]
///     tag = TestTag,
///     #[tag = 0x01]
///     A = A,
///     #[tag = 0x02]
///     B = B,
/// }
///
/// #[derive(Debug, KnownLayout, Immutable, TryFromBytes, IntoBytes)]
/// struct A {}
///
/// #[derive(Debug, KnownLayout, Immutable, TryFromBytes)]
/// struct B {}
/// ```
#[macro_export]
macro_rules! DstData {
(
    $(#[$attr:meta])*
    $vis:vis enum $name:ident {
        $(#[$ref_attr:meta])*
        ref_type = $ref_name:ident,
        $(#[$mut_attr:meta])*
        mut_type = $mut_name:ident,
        #[repr($repr:meta)]
        tag = $tag_name:ident,
        $(
            $(#[tag = $tag:literal])?
            $field:ident = $type:ty
        ),* $(,)?
    }
) => {
        $(#[$attr])*
        #[derive(Debug, Copy, Clone,
            zerocopy::KnownLayout, zerocopy::Immutable,
            zerocopy::TryFromBytes, zerocopy::IntoBytes
        )]
        #[repr($repr)]
        $vis enum $tag_name {
            $($field $( = $tag)? ),*
        }

        $crate::ignore_when_none!($ref_name,

            $(#[$ref_attr])*
            $vis enum $ref_name<'a> {
                 $( $field(&'a [$type]) ),*
            }

            impl<'a> $crate::dst::data::DstData<'a> for $ref_name<'a> {
                type Tag = $tag_name;

                fn as_tag_and_bytes(
                    &'a self,
                ) -> (Self::Tag, &'a [u8]) {
                    match self {
                        $($ref_name::$field(data) => ($tag_name::$field, data.as_bytes())),*
                    }
                }
            }
            impl<'a> $crate::dst::data::DstDataRef<'a> for $ref_name<'a> {
                fn ref_from_header_and_bytes<H>(header: H, bytes: &'a [u8]) -> Result<Self, $crate::Error>
                where
                    H: $crate::dst::header::DstHeader<Tag = Self::Tag>
                {
                    let tag = header.data_tag();
                    let len = header.data_len();
                    let data = match tag {
                        $($tag_name::$field => Self::$field( <H::LengthMeta as $crate::dst::length::DstLength>::parse_ref_bytes(bytes, len)? )),*
                    };
                    Ok(data)
                }
            }

            impl<'a> std::io::Read for $ref_name<'a> {
                fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
                    match self {
                        $($ref_name::$field(data) => data.as_bytes().read(buf)),*
                    }
                }
            }
        );

        $crate::ignore_when_none!($mut_name,

            $(#[$mut_attr])*
            $vis enum $mut_name<'a> {
                 $( $field(&'a mut [$type]) ),*
            }

            impl<'a> $crate::dst::data::DstData<'a> for $mut_name<'a> {
                type Tag = $tag_name;

                fn as_tag_and_bytes(
                    &'a self,
                ) -> (Self::Tag, &'a [u8]) {
                    match self {
                        $($mut_name::$field(data) => ($tag_name::$field, data.as_bytes())),*
                    }
                }
            }

            impl<'a> $crate::dst::data::DstDataMut<'a> for $mut_name<'a> {
                fn mut_from_header_and_bytes<H>(header: H, bytes: &'a mut [u8]) -> Result<Self, $crate::Error>
                where
                    H: $crate::dst::header::DstHeader<Tag = Self::Tag>
                {
                    let tag = header.data_tag();
                    let len = header.data_len();
                    let data = match tag {
                        $($tag_name::$field => Self::$field( <H::LengthMeta as $crate::dst::length::DstLength>::parse_mut_bytes(bytes, len)? )),*
                    };
                    Ok(data)
                }
            }

            impl<'a> std::io::Read for $mut_name<'a> {
                fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
                    match self {
                        $($mut_name::$field(data) => data.as_bytes().read(buf)),*
                    }
                }
            }
        );
    };
}

#[macro_export]
macro_rules! ignore_when_none {
    (None, $($other:tt)*) => {};
    ($cond:ident, $($other:tt)*) => {
        $($other)*
    };
}

/// # Example:
/// ```rust
/// use macro_rules_attribute::derive;
/// use crate::types::dst::DstHeader;
///
/// #[derive(Debug, DstHeader!)]
/// struct TestHeader {
///     other: u8,
///     #[derive_args(dst_tag)]
///     tag: u8,
///     #[derive_args(dst_len)]
///     len: u16,
/// }
/// ```
#[expect(unused)]
macro_rules! DstHeader {
    (
        $(#[$attr:meta])*
        $vis:vis struct $name:ident {
            $(
                $(#[ $($field_attr:tt)* ])*
                $field_vis:vis $field:ident: $type:ty
            ),* $(,)?
        }
    ) => {
        impl $crate::dst::header::DstHeader for $name {
            $(
                $crate::DstHeader!($( $($field_attr)* )*,
                    $field_vis $field: $type,
                );
            )*
        }
    };
    (
        derive_args(dst_tag), $vis:vis $name:ident: $type:ty,
    ) => {
        type Tag = $type;

        #[inline]
        fn data_tag(&self) -> Self::Tag {
            self.$name
        }
    };
    (
        derive_args(dst_len), $vis:vis $name:ident: $type:ty,
    ) => {
        #[inline]
        fn data_len(&self) -> usize {
            self.$name as usize
        }
    };
    ($($other:tt)*) => {};
}

