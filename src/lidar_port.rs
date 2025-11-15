//! This module contains the [`LidarStream`] trait with various implementors
//! and the related LidarConfig struct.
//!
//! The lidar streams can be created by calling their new method
//! or calling the [`LidarStream::from_config`] method.
use std::net::Ipv4Addr;

pub mod detection;
pub mod imu;
pub mod point_data;

pub use detection::DetectionPort;
pub use imu::ImuPort;
pub use point_data::PointDataPort;

#[derive(Debug, Clone)]
pub struct IpConfig {
    /// also known as `host_ip`
    pub local: Ipv4Addr,
    pub lidar: Ipv4Addr,
}

#[derive(Debug, Clone)]
pub struct SocketPortConfig {
    /// The port of the data on the `local` machine or the `host` machine.
    pub local: u16,
    /// The port of the data on the `Livox` lidar.
    pub lidar: u16,
}

#[derive(Debug)]
pub struct LidarPortConfig {
    pub ip: IpConfig,
    pub port: SocketPortConfig,
    pub buffer_init_size: usize,
}

impl IpConfig {
    pub fn new(local: impl Into<Ipv4Addr>, lidar: impl Into<Ipv4Addr>) -> Self {
        Self {
            local: local.into(),
            lidar: lidar.into(),
        }
    }

    pub const fn new_const(local: Ipv4Addr, lidar: Ipv4Addr) -> Self {
        Self { local, lidar }
    }
}

impl LidarPortConfig {
    pub const fn new(
        local_ip: Ipv4Addr,
        local_port: u16,
        lidar_ip: Ipv4Addr,
        lidar_port: u16,
        buffer_init_size: usize,
    ) -> Self {
        Self {
            ip: IpConfig {
                local: local_ip,
                lidar: lidar_ip,
            },
            port: SocketPortConfig {
                local: local_port,
                lidar: lidar_port,
            },
            buffer_init_size,
        }
    }
}
