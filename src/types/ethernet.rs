use std::fmt::Display;

use zerocopy::{Immutable, IntoBytes, KnownLayout, TryFromBytes, Unaligned};

/// see also: [`Livox Ethernet Protocol`](https://livox-wiki-en.readthedocs.io/en/latest/tutorials/new_product/mid360/livox_eth_protocol_mid360.html#point-cloud-imu-data-protocol)
#[derive(Debug, KnownLayout, Immutable, Unaligned, TryFromBytes)]
#[repr(C, packed)]
pub struct EthernetPacketHeader {
    /// Package protocol version: currently 0.
    pub version: u8,
    /// The length of the entire UDP data segment starting from [`version`](Self::version).
    pub length: u16,
    /// Intra-frame point cloud sampling time (Unit: 0.1us);
    /// In this frame of point cloud data, the time of the last point minus the time of the first point.
    pub time_interval: u16,
    /// The current UDP packet data field contains the number of points.
    pub dot_num: u16,
    /// Point cloud UDP packet count, each UDP packet is incremented by 1 in turn,
    /// and cleared to 0 at the beginning of the point cloud frame.
    pub udp_cnt: u16,
    /// Point cloud frame count, plus 1 for each frame of point cloud (10Hz/15Hz, etc.);
    /// For non-repeating scans, this field is invalid.
    pub frame_cnt: u8,
    /// Data type. For details, see [`2.3 Data Types`](https://livox-wiki-en.readthedocs.io/en/latest/tutorials/new_product/mid360/livox_eth_protocol_mid360.html#data-types).
    pub data_type: PointDataType,
    /// Timestamp type. For details, see [`2.2 Timestamp`](https://livox-wiki-en.readthedocs.io/en/latest/tutorials/new_product/mid360/livox_eth_protocol_mid360.html#timestamp).
    pub time_type: u8,
    /// Reserved.
    pub reserved: [u8; 12],
    /// Timestamp + data segment check code, using CRC-32 algorithm.
    /// For details, see [`6 CRC Algorithm`](https://livox-wiki-en.readthedocs.io/en/latest/tutorials/new_product/mid360/livox_eth_protocol_mid360.html#crc-algorithm).
    pub crc32: u32,
    /// Point cloud timestamp. For details, see [`2.2 Timestamp`](https://livox-wiki-en.readthedocs.io/en/latest/tutorials/new_product/mid360/livox_eth_protocol_mid360.html#timestamp).
    pub timestamp: [u8; 8],
}

/// see also [`Data Types`](https://livox-wiki-en.readthedocs.io/en/latest/tutorials/new_product/mid360/livox_eth_protocol_mid360.html#data-types)
#[derive(Debug, Copy, Clone, Immutable, TryFromBytes)]
#[repr(u8)]
pub enum PointDataType {
    ImuData = 0,
    /// the default data type
    /// 96 samples per packet.
    CartesianCoordinateHighData = 1,
    /// 96 samples per packet.
    CartesianCoordinateLowData = 2,
    /// 96 samples per packet.
    SphericalCoordinateData = 3,
}

impl Display for PointDataType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PointDataType::ImuData => write!(f, "ImuData"),
            PointDataType::CartesianCoordinateHighData => write!(f, "CartesianCoordinateHighData"),
            PointDataType::CartesianCoordinateLowData => write!(f, "CartesianCoordinateLowData"),
            PointDataType::SphericalCoordinateData => write!(f, "SphericalCoordinateData"),
        }
    }
}

#[derive(Debug, Clone, KnownLayout, Immutable, Unaligned, TryFromBytes, IntoBytes)]
#[repr(C, packed)]
pub struct ImuData {
    /// Unit: rad/s
    pub gyro_x: f32,
    /// Unit: rad/s
    pub gyro_y: f32,
    /// Unit: rad/s
    pub gyro_z: f32,
    /// Unit: g
    pub acc_x: f32,
    /// Unit: g
    pub acc_y: f32,
    /// Unit: g
    pub acc_z: f32,
}

/// Cartesian coordinate data with high precision.
#[derive(Debug, Clone, KnownLayout, Immutable, Unaligned, TryFromBytes, IntoBytes)]
#[repr(C, packed)]
pub struct CartesianHighPoint {
    /// X axis, Unit:mm
    pub x: i32,
    /// Y axis, Unit:mm
    pub y: i32,
    /// Z axis, Unit:mm
    pub z: i32,
    pub reflectivity: u8,
    /// According to the point cloud frame header `pack_info.tag_type` field, which is in [`reserved`](EthernetPacketHeader::reserved) field, match the specific tag type.
    /// For details, see [`2.4 Tag Information`](https://livox-wiki-en.readthedocs.io/en/latest/tutorials/new_product/mid360/livox_eth_protocol_mid360.html#tag-information).
    pub tag: u8,
}

/// Cartesian coordinate data with low precision.
#[derive(Debug, Clone, KnownLayout, Immutable, Unaligned, TryFromBytes, IntoBytes)]
#[repr(C, packed)]
pub struct CartesianLowPoint {
    /// X axis, Unit:cm
    pub x: i16,
    /// Y axis, Unit:cm
    pub y: i16,
    /// Z axis, Unit:cm
    pub z: i16,
    pub reflectivity: u8,
    /// According to the point cloud frame header `pack_info.tag_type` field, which is in [`reserved`](EthernetPacketHeader::reserved) field, match the specific tag type.
    /// For details, see [`2.4 Tag Information`](https://livox-wiki-en.readthedocs.io/en/latest/tutorials/new_product/mid360/livox_eth_protocol_mid360.html#tag-information).
    pub tag: u8,
}

/// Spherical coordinate data.
#[derive(Debug, Clone, KnownLayout, Immutable, Unaligned, TryFromBytes, IntoBytes)]
#[repr(C, packed)]
pub struct SphericalPoint {
    /// Unit: mm
    pub depth: u32,
    /// Zenith angle[0, 18000], Unit: 0.01 degree
    pub theta: u16,
    /// Azimuth[0, 36000], Unit: 0.01 degree
    pub phi: u16,
    pub reflectivity: u8,
    /// According to the point cloud frame header `pack_info.tag_type` field, which is in [`reserved`](EthernetPacketHeader::reserved) field, match the specific tag type.
    /// For details, see [`2.4 Tag Information`](https://livox-wiki-en.readthedocs.io/en/latest/tutorials/new_product/mid360/livox_eth_protocol_mid360.html#tag-information).
    pub tag: u8,
}
