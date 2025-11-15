use std::fmt::Display;

use zerocopy::{Immutable, IntoBytes, KnownLayout, TryFromBytes, Unaligned};

#[derive(Debug, Clone, Copy, Immutable, TryFromBytes, IntoBytes)]
#[repr(u8)]
pub enum StartOfFrame {
    AA = 0xAA,
}

#[derive(Debug, Clone, Copy, Immutable, TryFromBytes, IntoBytes)]
#[repr(u8)]
pub enum Version {
    V0 = 0,
}

#[derive(Debug, KnownLayout, Immutable, Unaligned, TryFromBytes, IntoBytes)]
#[repr(C, packed)]
pub struct SdkPacketHeader {
    /// Starting byte, fixed to be `0xAA`.
    pub sof: StartOfFrame,
    /// Protocol version, `0` for current version.
    pub version: Version,
    /// Length of frame;
    /// The number of bytes from beginning of [`sof`](Self::sof) to end of entire data segment. Max value: 1400.
    pub length: u16,
    /// This field is incremented by 1 for each new REQ request message;
    /// This field of ACK message is the same as REQ and can be used for message matching.
    pub seq_num: u32,
    /// Different types of messages are distinguished by this field,
    /// For details, see [`5 Return Code Description`](https://livox-wiki-en.readthedocs.io/en/latest/tutorials/new_product/mid360/livox_eth_protocol_mid360.html#return-code-description).
    pub cmd_id: CommandID,
    /// Command Type:
    /// 0x00: REQ, actively send data request;
    /// 0x01: ACK, response to REQ data.
    pub cmd_type: CommandType,
    /// The sender’s device type:
    /// 0x00: Host computer,
    /// 0x01: LiDAR.
    pub sender_type: SendType,
    pub reserved: [u8; 6],
    /// Frame header checksum, check data starts from sof to crc16 (not included),
    /// 18 bytes in total. For details, see [`6 CRC Algorithm`](https://livox-wiki-en.readthedocs.io/en/latest/tutorials/new_product/mid360/livox_eth_protocol_mid360.html#CRC-Algorithm).
    pub crc16_h: u16,
    /// Frame data checksum. For details, see [`6 CRC Algorithm`](https://livox-wiki-en.readthedocs.io/en/latest/tutorials/new_product/mid360/livox_eth_protocol_mid360.html#CRC-Algorithm);
    /// When length of data field is 0, CRC32 needs to be padded with 0.
    pub crc32_d: u32,
}

impl SdkPacketHeader {
    pub const SIZE: usize = std::mem::size_of::<Self>();
    pub fn new(
        data_length: usize,
        cmd_id: CommandID,
        cmd_type: CommandType,
        sender_type: SendType,
    ) -> Self {
        let length = (Self::SIZE + data_length) as u16;
        Self {
            sof: StartOfFrame::AA,
            version: Version::V0,
            length,
            seq_num: crate::seq::next_seq(),
            cmd_id,
            cmd_type,
            sender_type,
            reserved: [0; 6],
            crc16_h: 0,
            crc32_d: 0,
        }
    }
    pub fn data_len(&self) -> usize {
        self.length as usize - Self::SIZE
    }
}

/// see also [`Command ID`](https://livox-wiki-en.readthedocs.io/en/latest/tutorials/new_product/mid360/livox_eth_protocol_mid360.html#command-id)
#[derive(Debug, Clone, Copy, Immutable, TryFromBytes, IntoBytes)]
#[repr(u16)]
pub enum CommandID {
    /* Device Type Query */
    /// Discovery by broadcasting
    QueryDeviceType = 0x0000,

    /* LiDAR Information */
    /// Parameter information configuration
    ConfigParamInfo = 0x0001,
    /// Inquire LiDAR information
    InquireLidarInfo = 0x0101,
    /// Push LiDAR information
    PushLidarInfo = 0x0102,

    /* Control CMD */
    /// Request reboot device
    RequestRebootDevice = 0x0200,
    /// Restore factory settings
    RestoreFactorySettings = 0x0201,
    /// Set GPS timestamp
    SetGpsTimestamp = 0x0202,

    /* log CMD */
    /// Log file push
    PushLogFile = 0x0300,
    /// Log collection configuration
    LogCollectionConfig = 0x0301,
    /// Log system time synchronization
    SyncLogSystemTime = 0x0302,
    /// Debug raw data collection configuration
    DebugRawDataCollectionConfig = 0x0303,

    /* General Uprade CMD */
    /// Request to start upgrade
    RequestStartUpgrade = 0x0400,
    /// Firmware data transfer
    TransferFirmwareData = 0x0401,
    /// Firmware transfer complete
    FirmwareTransferComplete = 0x0402,
    /// Get firmware upgrade status
    GetFirmwareUpgradeStatus = 0x0403,
}

impl Display for CommandID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::QueryDeviceType => write!(f, "Discovery device type by broadcasting"),
            Self::ConfigParamInfo => write!(f, "Parameter information configuration"),
            Self::InquireLidarInfo => write!(f, "Inquire LiDAR information"),
            Self::PushLidarInfo => write!(f, "Push LiDAR information"),
            Self::RequestRebootDevice => write!(f, "Request reboot device"),
            Self::RestoreFactorySettings => write!(f, "Restore factory settings"),
            Self::SetGpsTimestamp => write!(f, "Set GPS timestamp"),
            Self::PushLogFile => write!(f, "Log file push"),
            Self::LogCollectionConfig => write!(f, "Log collection configuration"),
            Self::SyncLogSystemTime => write!(f, "Log system time synchronization"),
            Self::DebugRawDataCollectionConfig => {
                write!(f, "Debug raw data collection configuration")
            }
            Self::RequestStartUpgrade => write!(f, "Request to start upgrade"),
            Self::TransferFirmwareData => write!(f, "Firmware data transfer"),
            Self::FirmwareTransferComplete => write!(f, "Firmware transfer complete"),
            Self::GetFirmwareUpgradeStatus => write!(f, "Get firmware upgrade status"),
        }
    }
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
pub struct QueryDeviceTypeAck {
    /// Return code
    /// For details, see [`5 Return Code Description`](https://livox-wiki-en.readthedocs.io/en/latest/tutorials/new_product/mid360/livox_eth_protocol_mid360.html#5-Return-Code-Description)
    pub ret_code: u8,
    /// LiDAR type
    pub dev_type: LivoxLidarDeviceType,
    /// LiDAR SN
    pub sn: [u8; 16],
    /// LiDAR IP address
    ///
    /// E.g: AA.BB.CC.DD
    /// ```text
    /// lidar_ip[0] = AA
    /// lidar_ip[1] = BB
    /// lidar_ip[2] = CC
    /// lidar_ip[3] = DD
    /// ```
    pub lidar_ip: [u8; 4],
    /// LiDAR command port
    pub cmd_port: u16,
}

#[derive(Debug, Clone, Copy, Immutable, TryFromBytes, IntoBytes)]
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

impl QueryDeviceTypeAck {
    #[inline]
    pub fn is_valid(&self) -> bool {
        self.ret_code == 0
    }
}
