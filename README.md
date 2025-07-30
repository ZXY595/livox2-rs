# livox2
A lightweight and pure Rust implementation of Livox SDK2, based on async-net and zerocopy.

# Overview
livox2 is a pure Rust library for interfacing with Livox LiDAR devices.
This crate see lidar data streams as a [`streams::LidarStream`], just like the socket data
flows they are in low-level network communication.

# Usage
To initialize the lidar data streams, use [`LivoxLidarBuilder`] to help you
create the device instances by forwarding the configuration.

Or you can create the device instances by yourself, see also [`streams`] module.

Once you have the device instances, you can call [`streams::LidarStream::next_packet`] to
receive the corresponding data packets.

> [!NOTE]
> This crate doens't provide stream methods, because the [`futures_core::Stream`] trait doesn't
> guarantee the unique mutable access to the socket buffer between each yiled
> [`futures_core::Stream::Item`], and can't even return a reference.
> 
> Instead, to get iterate the lidar data streams, you must match the enum
> [`streams::LidarStream::Packet`] first, take the inner array slice, then
> iterate this array slice to read the data.

 # Example
 See tests in crate lib.rs
