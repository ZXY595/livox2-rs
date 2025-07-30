//! # Overview
//! livox2 is a pure Rust library for interfacing with Livox LiDAR devices.
//! This crate see lidar data streams as a [`streams::LidarStream`], just like the socket data
//! flows they are in low-level network communication.
//!
//! # Usage
//! To initialize the lidar data streams, use [`LivoxLidarBuilder`] to help you
//! create the device instances by forwarding the configuration.
//!
//! Or you can create the device instances by yourself, see also [`streams`] module.
//!
//! Once you have the device instances, you can call [`streams::LidarStream::next_packet`] to
//! receive the corresponding data packets.
//!
//! # Note
//! This crate doens't provide stream methods, because the [`futures_core::Stream`] trait doesn't
//! guarantee the unique mutable access to the socket buffer between each yiled
//! [`futures_core::Stream::Item`], and can't even return a reference.
//!
//! Instead, to get iterate the lidar data streams, you must match the enum
//! [`streams::LidarStream::Packet`] first, take the inner array slice, then
//! iterate this array slice to read the data.
//!
//! # Example
//! See tests in crate lib.rs

mod crc;
pub mod error;
mod seq;
pub mod streams;
pub mod types;

pub mod dst;

use std::{io, net::Ipv4Addr};

pub use error::Error;
use streams::{LidarConfig, LidarStream};
use tuple_list::TupleList;

/// A builder for creating [`LidarStream`] instances.
pub struct LivoxLidarBuilder<D> {
    config: LidarConfig,
    device: D,
}

impl<Ds> LivoxLidarBuilder<Ds>
where
    Ds: TupleList,
{
    pub fn new(
        local_ip: impl Into<Ipv4Addr>,
        lidar_ip: impl Into<Ipv4Addr>,
    ) -> LivoxLidarBuilder<()> {
        Self::new_const(local_ip.into(), lidar_ip.into())
    }
    pub const fn new_const(local_ip: Ipv4Addr, lidar_ip: Ipv4Addr) -> LivoxLidarBuilder<()> {
        Self::from_config(LidarConfig::new(local_ip, lidar_ip))
    }
    pub const fn from_config(config: LidarConfig) -> LivoxLidarBuilder<()> {
        LivoxLidarBuilder { config, device: () }
    }
    pub async fn add_stream<D>(self) -> Result<LivoxLidarBuilder<(D, Ds)>, io::Error>
    where
        D: LidarStream,
    {
        Ok(LivoxLidarBuilder {
            device: (D::from_config(&self.config).await?, self.device),
            config: self.config,
        })
    }
    pub fn build(self) -> Ds::Tuple {
        self.device.into_tuple()
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use zerocopy::*;

    const TEST_CONFIG: LivoxLidarBuilder<()> = LivoxLidarBuilder::new_const(
        Ipv4Addr::new(192, 168, 1, 50),
        Ipv4Addr::new(192, 168, 1, 3),
    );

    #[test]
    fn test_builder() {
        smol::block_on(async {
            let (detect, imu) = TEST_CONFIG
                .add_stream::<streams::PointDataStream>()
                .await
                .unwrap()
                .add_stream::<streams::ImuStream>()
                .await
                .unwrap()
                .build();
        });
    }

    #[test]
    fn test_detect_lidar() {
        smol::block_on(async {
            let mut detection = TEST_CONFIG
                .add_stream::<streams::DetectionStream>()
                .await
                .unwrap()
                .build()
                .0;
            let result = detection.next_packet().await.unwrap();
            dbg!(result);
        });
    }

    #[test]
    fn test_recv_packet() {
        smol::block_on(async {
            let mut lidar = TEST_CONFIG
                .add_stream::<streams::PointDataStream>()
                .await
                .unwrap()
                .build()
                .0;
            let packet = lidar.next_packet().await.unwrap();
            dbg!(packet.header);
            dbg!(&packet.data[..5]);
        });
    }
}
