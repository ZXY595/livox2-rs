use std::io;

use async_net::{AsyncToSocketAddrs, UdpSocket};

use crate::types::ethernet::EthernetPacketRef;

pub struct ImuStream {
    socket: UdpSocket,
    buffer: Vec<u8>,
}

impl ImuStream {
    const INIT_BUFFER_SIZE: usize = 60;

    pub async fn new(
        local_addr: impl AsyncToSocketAddrs,
        lidar_addr: impl AsyncToSocketAddrs,
    ) -> Result<Self, io::Error> {
        let socket = UdpSocket::bind(local_addr).await?;
        socket.connect(lidar_addr).await?;

        Ok(Self {
            socket,
            buffer: vec![0; Self::INIT_BUFFER_SIZE],
        })
    }
}

impl super::LidarStream for ImuStream {
    type Packet<'a> = EthernetPacketRef<'a>;
    async fn from_config(config: &super::LidarConfig) -> Result<Self, io::Error> {
        Self::new(
            (config.local_ip, config.local_imu_port),
            (config.lidar_ip, config.lidar_imu_port),
        )
        .await
    }

    async fn next_packet(&mut self) -> Result<Self::Packet<'_>, io::Error> {
        let buffer = self.buffer.as_mut();
        let len = self.socket.recv(buffer).await?;
        let buffer = &buffer[..len];
        EthernetPacketRef::try_ref_from_bytes(buffer).map_err(From::from)
    }
}
