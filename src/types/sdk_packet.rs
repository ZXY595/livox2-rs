use macro_rules_attribute::apply;
use zerocopy::{Immutable, IntoBytes, KnownLayout, TryFromBytes, Unaligned};

use crate::{
    DstData,
    dst::{DstRef, data::DstData, header::DstHeader, length},
};

pub type SdkPacketRef<'a> = DstRef<'a, &'a SdkPacketHeader, CommandDataRef<'a>>;

pub type SdkPacketOwned<'a> = DstRef<'a, SdkPacketHeader, CommandDataRef<'a>>;

#[derive(Debug, KnownLayout, Immutable, Unaligned, TryFromBytes, IntoBytes)]
#[repr(C, packed)]
pub struct SdkPacketHeader {
    /// 0xAA
    pub sof: u8,
    /// 0
    pub version: u8,
    pub length: u16,
    pub seq_num: u32,
    pub cmd_id: CommandID,
    pub cmd_type: CommandType,
    pub sender_type: SendType,
    pub rsvd: [u8; 6],
    /// for first 18 bytes
    pub crc16_h: u16,
    /// for data bytes
    pub crc32_d: u32,
}

impl DstHeader for SdkPacketHeader {
    type Tag = CommandID;
    type LengthMeta = length::UntypedLen<Self>;

    #[inline]
    fn data_len(&self) -> usize {
        self.length as usize
    }

    #[inline]
    fn data_tag(&self) -> Self::Tag {
        self.cmd_id
    }
}

#[apply(DstData!)]
pub enum CommandData {
    #[derive(Debug)]
    ref_type = CommandDataRef,
    #[derive(Debug)]
    mut_type = CommandDataMut,
    #[repr(u16)]
    tag = CommandID,
    #[tag = 0x0000]
    LidarSearch = DetectionData,
}

impl CommandID {
    pub const COUNT: usize = 256;
}

#[derive(Debug, Clone, Copy, KnownLayout, Immutable, TryFromBytes, IntoBytes)]
#[repr(u8)]
pub enum CommandType {
    /// command type, which requires response from the receiver.
    Cmd = 0,
    /// acknowledge type, which is the response of command type.
    Ack = 1,
}

#[derive(Debug, Clone, Copy, KnownLayout, Immutable, TryFromBytes, IntoBytes)]
#[repr(u8)]
pub enum SendType {
    /// command type, which requires response from the receiver.
    HostSend = 0,
    /// acknowledge type, which is the response of command type.
    LidarSend = 1,
}

#[derive(Debug, KnownLayout, Immutable, Unaligned, TryFromBytes, IntoBytes)]
#[repr(C, packed)]
pub struct DetectionData {
    pub ret_code: u8,
    pub dev_type: LivoxLidarDeviceType,
    pub sn: [u8; 16],
    pub lidar_ip: [u8; 4],
    pub cmd_port: u16,
}

#[derive(Debug, Clone, Copy, KnownLayout, Immutable, Unaligned, TryFromBytes, IntoBytes)]
#[repr(u8)]
pub enum LivoxLidarDeviceType {
    Hub = 0,
    Mid40 = 1,
    Tele = 2,
    Horizon = 3,
    Mid70 = 6,
    Avia = 7,
    Mid360 = 9,
    IndustrialHAP = 10,
    HAP = 15,
    PA = 16,
}

impl DetectionData {
    #[inline]
    pub fn is_valid(&self) -> bool {
        self.ret_code == 0
    }
}

impl<'a> SdkPacketOwned<'a> {
    pub const HEADER_SIZE: usize = std::mem::size_of::<SdkPacketHeader>();

    pub fn new_with_data(
        cmd_type: CommandType,
        sender_type: SendType,
        data: CommandDataRef<'a>,
    ) -> Self {
        let (cmd_id, bytes) = data.as_tag_and_bytes();
        let mut header = SdkPacketHeader {
            sof: 0xAA,
            version: 0,
            length: (Self::HEADER_SIZE + bytes.len()) as u16,
            seq_num: crate::seq::next_seq(),
            cmd_id,
            cmd_type,
            sender_type,
            rsvd: [0; 6],
            crc16_h: 0,
            crc32_d: 0,
        };
        header.crc16_h = crate::crc::CRC16.checksum(&header.as_bytes()[0..18]);
        header.crc32_d = crate::crc::CRC32.checksum(bytes);

        Self::new(header, data)
    }
}
