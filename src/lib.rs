//! # Overview
//! livox2 is a pure Rust library for interfacing with Livox LiDAR devices.
//! This crate provides the [`lidar_port`] API, and each port is actually a lidar data flow, like the [point cloud port](lidar_port::PointDataPort).
//!
//! # Usage
//! To initialize the lidar data ports, see [`lidar_port::LidarPortConfig`] new methods to help you build the
//! data ports.
//!
//! Or you can create the device port instances by yourself, see also [`lidar_port`] sub-modules.
//!
//! Once you have the device port instances, you can call [`next_packet_ref`](lidar_port::PointDataPort::next_packet_ref) method to
//! receive the corresponding data packets.
//!
//! # Example
//! See also [example](https://github.com/ZXY595/livox2-rs/tree/main/example)

mod crc;
pub mod error;
pub mod lidar_port;
mod seq;
pub mod types;

pub use error::Error;
