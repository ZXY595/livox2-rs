# Livox2

# Overview
livox2 is a pure Rust library for interfacing with Livox LiDAR devices.
This crate provides the [`lidar_port`] API, and each port is actually a lidar data flow, like the [point cloud port](lidar_port::PointDataPort).

# Usage
To initialize the lidar data ports, see [`lidar_port::LidarPortConfig`] new methods to help you build the
data ports.

Or you can create the device port instances by yourself, see also [`lidar_port`] sub-modules.

Once you have the device port instances, you can call [`next_packet_ref`](lidar_port::PointDataPort::next_packet_ref) method to
receive the corresponding data packets.

> [!NOTE]
> This crate doens't provide stream methods by default, because the [`futures_core::Stream`] trait doesn't
> guarantee the unique mutable access to the socket buffer between each yield
> [`futures_core::Stream::Item`], and can't even return a reference.
> 
> Instead, to get iterate the lidar data streams, you must match the packet data first, like:
> [`CoordinateData`](lidar_port::point_data::CoordinateData), take the inner array slice, then
> iterate this array slice to read the data.

# Example
See also [example](https://github.com/ZXY595/livox2-rs/tree/main/example)
