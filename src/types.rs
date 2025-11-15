//! Many data structures used to communicate with the livox lidar device.
pub mod ethernet;
pub mod sdk_packet;

use core::ffi;

use zerocopy::{Immutable, KnownLayout};

#[derive(KnownLayout, Immutable)]
#[repr(u8)]
pub enum ProtocolType {
    LidarSdk,
    Rsvd1,
    ProtocolUndef,
}

#[repr(C)]
pub enum LogType {
    RealTimeLog = 0,
    ExceptionLog = 0x01,
}

#[repr(C)]
pub enum LidarStatus {
    /// Command send failed.
    SendFailed = -9,
    /// Handler implementation not exist.
    HandlerImplNotExist = -8,
    /// Device handle invalid.
    InvalidHandle = -7,
    /// Command channel not exist.
    ChannelNotExist = -6,
    /// No enough memory.
    NotEnoughMemory = -5,
    /// Operation timeouts.
    Timeout = -4,
    /// Operation is not supported on this device.
    NotSupported = -3,
    /// Requested device is not connected.
    NotConnected = -2,
    /// Failure.
    Failure = -1,
    /// Success.
    Success = 0,
}

#[repr(C)]
pub struct AsyncControlResponse {
    pub ret_code: u8,
    pub error_key: u16,
}

#[repr(C)]
pub struct InfoResponse {
    pub ret_code: u8,
    pub lidar_info: *const ffi::c_char,
}

#[repr(C)]
pub enum WorkMode {
    Normal = 0x01,
    WakeUp = 0x02,
    Sleep = 0x03,
    Error = 0x04,
    PowerOnSelfTest = 0x05,
    MotorStarting = 0x06,
    MotorStoping = 0x07,
    Upgrade = 0x08,
}

#[repr(C)]
pub enum WorkModeAfterBoot {
    AfterBootDefault = 0x00,
    AfterBootNormal = 0x01,
    AfterBootWakeUp = 0x02,
}


pub struct DirectLidarStateInfo {
    pub pcl_data_type: u8,                      // 0x0000
    pub pattern_mode: u8,                       // 0x0001
    pub dual_emit_en: u8,                       // 0x0002
    pub point_send_en: u8,                      // 0x0003
    pub lidar_ipcfg: LivoxLidarIpInfo,          // 0x0004
    pub host_state_info: HostStateInfoIpInfo,   // 0x0005
    pub pointcloud_host_ipcfg: HostPointIPInfo, // 0x0006
    pub imu_host_ipcfg: HostImuDataIPInfo,      // 0x0007
    pub ctl_host_ipcfg: LivoxIpCfg,             // 0x0008
    pub log_host_ipcfg: LivoxIpCfg,             // 0x0009

    pub vehicle_speed: i32,                          // 0x0010
    pub environment_temp: i32,                       // 0x0011
    pub install_attitude: LivoxLidarInstallAttitude, // 0x0012
    pub blind_spot_set: u32,                         // 0x0013
    pub frame_rate: u8,                              // 0x0014
    pub fov_cfg0: FovCfg,                            // 0x0015
    pub fov_cfg1: FovCfg,                            // 0x0016
    pub fov_cfg_en: u8,                              // 0x0017
    pub detect_mode: u8,                             // 0x0018
    pub func_io_cfg: [u8; 4],                        // 0x0019
    pub work_tgt_mode: u8,                           // 0x001A
    pub glass_heat: u8,                              // 0x001B
    pub imu_data_en: u8,                             // 0x001C
    pub fusa_en: u8,                                 // 0x001D

    pub sn: [u8; 16],              // 0x8000
    pub product_info: [u8; 64],    // 0x8001
    pub version_app: [u8; 4],      // 0x8002
    pub version_loader: [u8; 4],   // 0x8003
    pub version_hardware: [u8; 4], // 0x8004
    pub mac: [u8; 6],              // 0x8005
    pub cur_work_state: u8,        // 0x8006
    pub core_temp: i32,            // 0x8007
    pub powerup_cnt: u32,          // 0x8008
    pub local_time_now: u64,       // 0x8009
    pub last_sync_time: u64,       // 0x800A
    pub time_offset: i64,          // 0x800B
    pub time_sync_type: u8,        // 0x800C
    pub status_code: [u8; 32],     // 0x800D
    pub lidar_diag_status: u16,    // 0x800E
    pub lidar_flash_status: u8,    // 0x800F
    pub fw_type: u8,               // 0x8010
    pub hms_code: u32,             // 0x8011
    pub roi_mode: u8,              // 0xFFFE
}

pub struct LivoxLidarIpInfo {
    /// IP address.
    pub ip_addr: [u8; 16],
    /// Subnet mask.
    pub net_mask: [u8; 16],
    /// Gateway address.
    pub gw_addr: [u8; 16],
}

pub struct HostStateInfoIpInfo {
    /// IP address.
    pub host_ip_addr: [u8; 16],
    pub host_state_info_port: u16,
    pub lidar_state_info_port: u16,
}

pub struct HostPointIPInfo {
    /// IP address.
    pub host_ip_addr: [u8; 16],
    pub host_point_data_port: u16,
    pub lidar_point_data_port: u16,
}

pub struct HostImuDataIPInfo {
    /// IP address.
    pub host_ip_addr: [u8; 16],
    /// resv
    pub host_imu_data_port: u16,
    /// resv
    pub lidar_imu_data_port: u16,
}

pub struct LivoxIpCfg {
    /// IP address.
    pub ip_addr: [u8; 16],
    pub dst_port: u16,
    pub src_port: u16,
}

pub struct LivoxLidarStateInfo {
    pub pcl_data_type: u8,
    pub pattern_mode: u8,
    pub dual_emit_en: u8,
    pub point_send_en: u8,
    pub lidar_ip_info: LivoxLidarIpInfo,
    pub host_point_ip_info: HostPointIPInfo,
    pub host_imu_ip_info: HostImuDataIPInfo,
    pub install_attitude: LivoxLidarInstallAttitude,
    pub blind_spot_set: u32,
    pub work_mode: u8,
    pub glass_heat: u8,
    pub imu_data_en: u8,
    pub fusa_en: u8,
    pub sn: [u8; 16],
    pub product_info: [u8; 64],
    pub version_app: [u8; 4],
    pub version_load: [u8; 4],
    pub version_hardware: [u8; 4],
    pub mac: [u8; 6],
    pub cur_work_state: u8,
    pub status_code: u64,
}

pub struct LivoxLidarInstallAttitude {
    pub roll_deg: f32,
    pub pitch_deg: f32,
    pub yaw_deg: f32,
    /// mm
    pub x: i32,
    /// mm
    pub y: i32,
    /// mm
    pub z: i32,
}

pub struct FovCfg {
    pub yaw_start: i32,
    pub yaw_stop: i32,
    pub pitch_start: i32,
    pub pitch_stop: i32,
    pub rsvd: u32,
}
