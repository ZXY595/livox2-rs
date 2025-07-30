use macro_rules_attribute::apply;
use zerocopy::{Immutable, IntoBytes, KnownLayout, TryFromBytes, Unaligned};

use crate::{
    DstData,
    dst::{DstRef, header::DstHeader, length},
};

pub type EthernetPacketRef<'a> = DstRef<'a, &'a EthernetPacketHeader, PointDataRef<'a>>;

#[derive(Debug, KnownLayout, Immutable, Unaligned, TryFromBytes)]
#[repr(C, packed)]
pub struct EthernetPacketHeader {
    pub version: u8,
    pub length: u16,
    /// unit: 0.1 us
    pub time_interval: u16,
    pub dot_num: u16,
    pub udp_cnt: u16,
    pub frame_cnt: u8,
    pub data_type: PointDataType,
    pub time_type: u8,
    pub rsvd: [u8; 12],
    pub crc32: u32,
    pub timestamp: [u8; 8],
}

impl DstHeader for EthernetPacketHeader {
    type Tag = PointDataType;
    type LengthMeta = length::TypedLen;

    #[inline]
    fn data_len(&self) -> usize {
        self.dot_num as usize
    }

    #[inline]
    fn data_tag(&self) -> Self::Tag {
        self.data_type
    }
}

#[apply(DstData!)]
pub enum PointData {
    #[derive(Debug)]
    ref_type = PointDataRef,
    mut_type = None,
    #[repr(u8)]
    tag = PointDataType,
    #[tag = 0]
    ImuData = ImuRawPoint,
    #[tag = 0x01]
    CartesianCoordinateHighData = CartesianHighRawPoint,
    #[tag = 0x02]
    CartesianCoordinateLowData = CartesianLowRawPoint,
    #[tag = 0x03]
    SphericalCoordinateData = SphericalRawPoint,
}

#[derive(Debug, KnownLayout, Immutable, Unaligned, TryFromBytes, IntoBytes)]
#[repr(C, packed)]
pub struct ImuRawPoint {
    pub(crate) gyro_x: f32,
    pub(crate) gyro_y: f32,
    pub(crate) gyro_z: f32,
    pub(crate) acc_x: f32,
    pub(crate) acc_y: f32,
    pub(crate) acc_z: f32,
}

/// Cartesian coordinate data with high precision.
#[derive(Debug, KnownLayout, Immutable, Unaligned, TryFromBytes, IntoBytes)]
#[repr(C, packed)]
pub struct CartesianHighRawPoint {
    /// X axis, Unit:mm
    pub(crate) x: i32,
    /// Y axis, Unit:mm
    pub(crate) y: i32,
    /// Z axis, Unit:mm
    pub(crate) z: i32,
    pub(crate) reflectivity: u8,
    pub(crate) tag: u8,
}

/// Cartesian coordinate data with low precision.
#[derive(Debug, KnownLayout, Immutable, Unaligned, TryFromBytes, IntoBytes)]
#[repr(C, packed)]
pub struct CartesianLowRawPoint {
    /// X axis, Unit:cm
    pub(crate) x: i16,
    /// Y axis, Unit:cm
    pub(crate) y: i16,
    /// Z axis, Unit:cm
    pub(crate) z: i16,
    pub(crate) reflectivity: u8,
    pub(crate) tag: u8,
}

/// Spherical coordinate data.
#[derive(Debug, KnownLayout, Immutable, Unaligned, TryFromBytes, IntoBytes)]
#[repr(C, packed)]
pub struct SphericalRawPoint {
    pub(crate) depth: u32,
    pub(crate) theta: u16,
    pub(crate) phi: u16,
    pub(crate) reflectivity: u8,
    pub(crate) tag: u8,
}
