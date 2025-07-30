//! This module contains the [`LidarStream`] trait with various implementors
//! and the related LidarConfig struct.
//!
//! The Lidarstreams can be created by calling their new method
//! or calling the [`LidarStream::from_config`] method.
use std::{io, net::Ipv4Addr};

mod detection;
mod imu;
mod point_data;
#[macro_use]
mod macros;
use macro_rules_attribute::derive;

pub use detection::DetectionStream;
pub use imu::ImuStream;
pub use point_data::PointDataStream;

pub trait LidarStream: Sized {
    type Packet<'a>
    where
        Self: 'a;
    fn from_config(config: &LidarConfig) -> impl Future<Output = Result<Self, io::Error>>;
    fn next_packet(&mut self) -> impl Future<Output = Result<Self::Packet<'_>, io::Error>>;
}

#[derive(Debug, BuilderMethods!)]
pub struct LidarConfig {
    pub lidar_ip: Ipv4Addr,
    /// or called `host_ip`
    pub local_ip: Ipv4Addr,
    pub detection_port: u16,
    pub lidar_point_data_port: u16,
    /// or called `host_point_data_port`
    pub local_point_data_port: u16,
    pub lidar_imu_port: u16,
    /// or called `host_imu_port`
    pub local_imu_port: u16,
}

impl LidarConfig {
    pub const fn new(local_ip: Ipv4Addr, lidar_ip: Ipv4Addr) -> Self {
        Self {
            local_ip,
            lidar_ip,
            detection_port: 56000,
            lidar_point_data_port: 56300,
            local_point_data_port: 56301,
            lidar_imu_port: 56400,
            local_imu_port: 56401,
        }
    }
}
