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
    ret_code: u8,
    error_key: u16,
}

#[repr(C)]
pub struct InfoResponse {
    ret_code: u8,
    lidar_info: *const ffi::c_char,
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


#[expect(unused)]
pub struct DirectLidarStateInfo {
    pcl_data_type: u8,                      // 0x0000
    pattern_mode: u8,                       // 0x0001
    dual_emit_en: u8,                       // 0x0002
    point_send_en: u8,                      // 0x0003
    lidar_ipcfg: LivoxLidarIpInfo,          // 0x0004
    host_state_info: HostStateInfoIpInfo,   // 0x0005
    pointcloud_host_ipcfg: HostPointIPInfo, // 0x0006
    imu_host_ipcfg: HostImuDataIPInfo,      // 0x0007
    ctl_host_ipcfg: LivoxIpCfg,             // 0x0008
    log_host_ipcfg: LivoxIpCfg,             // 0x0009

    vehicle_speed: i32,                          // 0x0010
    environment_temp: i32,                       // 0x0011
    install_attitude: LivoxLidarInstallAttitude, // 0x0012
    blind_spot_set: u32,                         // 0x0013
    frame_rate: u8,                              // 0x0014
    fov_cfg0: FovCfg,                            // 0x0015
    fov_cfg1: FovCfg,                            // 0x0016
    fov_cfg_en: u8,                              // 0x0017
    detect_mode: u8,                             // 0x0018
    func_io_cfg: [u8; 4],                        // 0x0019
    work_tgt_mode: u8,                           // 0x001A
    glass_heat: u8,                              // 0x001B
    imu_data_en: u8,                             // 0x001C
    fusa_en: u8,                                 // 0x001D

    sn: [u8; 16],              // 0x8000
    product_info: [u8; 64],    // 0x8001
    version_app: [u8; 4],      // 0x8002
    version_loader: [u8; 4],   // 0x8003
    version_hardware: [u8; 4], // 0x8004
    mac: [u8; 6],              // 0x8005
    cur_work_state: u8,        // 0x8006
    core_temp: i32,            // 0x8007
    powerup_cnt: u32,          // 0x8008
    local_time_now: u64,       // 0x8009
    last_sync_time: u64,       // 0x800A
    time_offset: i64,          // 0x800B
    time_sync_type: u8,        // 0x800C
    status_code: [u8; 32],     // 0x800D
    lidar_diag_status: u16,    // 0x800E
    lidar_flash_status: u8,    // 0x800F
    fw_type: u8,               // 0x8010
    hms_code: u32,             // 0x8011
    roi_mode: u8,              // 0xFFFE
}

#[expect(unused)]
pub struct LivoxLidarIpInfo {
    /// IP address.
    ip_addr: [u8; 16],
    /// Subnet mask.
    net_mask: [u8; 16],
    /// Gateway address.
    gw_addr: [u8; 16],
}

#[expect(unused)]
pub struct HostStateInfoIpInfo {
    /// IP address.
    host_ip_addr: [u8; 16],
    host_state_info_port: u16,
    lidar_state_info_port: u16,
}

#[expect(unused)]
pub struct HostPointIPInfo {
    /// IP address.
    host_ip_addr: [u8; 16],
    host_point_data_port: u16,
    lidar_point_data_port: u16,
}

#[expect(unused)]
pub struct HostImuDataIPInfo {
    /// IP address.
    host_ip_addr: [u8; 16],
    /// resv
    host_imu_data_port: u16,
    /// resv
    lidar_imu_data_port: u16,
}

#[expect(unused)]
pub struct LivoxIpCfg {
    /// IP address.
    ip_addr: [u8; 16],
    dst_port: u16,
    src_port: u16,
}

#[expect(unused)]
pub struct LivoxLidarStateInfo {
    pcl_data_type: u8,
    pattern_mode: u8,
    dual_emit_en: u8,
    point_send_en: u8,
    lidar_ip_info: LivoxLidarIpInfo,
    host_point_ip_info: HostPointIPInfo,
    host_imu_ip_info: HostImuDataIPInfo,
    install_attitude: LivoxLidarInstallAttitude,
    blind_spot_set: u32,
    work_mode: u8,
    glass_heat: u8,
    imu_data_en: u8,
    fusa_en: u8,
    sn: [u8; 16],
    product_info: [u8; 64],
    version_app: [u8; 4],
    version_load: [u8; 4],
    version_hardware: [u8; 4],
    mac: [u8; 6],
    cur_work_state: u8,
    status_code: u64,
}

#[expect(unused)]
pub struct LivoxLidarInstallAttitude {
    roll_deg: f32,
    pitch_deg: f32,
    yaw_deg: f32,
    /// mm
    x: i32,
    /// mm
    y: i32,
    /// mm
    z: i32,
}

#[expect(unused)]
pub struct FovCfg {
    yaw_start: i32,
    yaw_stop: i32,
    pitch_start: i32,
    pitch_stop: i32,
    rsvd: u32,
}
