use std::{
    io::{self, Read},
    net::Ipv4Addr,
};

use async_net::UdpSocket;

use crate::types::sdk_packet::{
    CommandDataRef, DetectionData, CommandType, SdkPacketOwned,
    SdkPacketRef, SendType,
};

pub struct DetectionStream {
    socket: UdpSocket,
    broadcast_socket: UdpSocket,
    buffer: Vec<u8>,
}

impl DetectionStream {
    const DETECTION_BUFFER_SIZE: usize = 48;

    pub async fn new(
        local_ip: impl Into<Ipv4Addr>,
        lidar_ip: impl Into<Ipv4Addr>,
        detection_port: u16,
    ) -> Result<Self, io::Error> {
        let local_ip = local_ip.into();
        let lidar_ip = lidar_ip.into();

        let broadcast_socket =
            UdpSocket::bind((Ipv4Addr::new(255, 255, 255, 255), detection_port)).await?;

        let socket = UdpSocket::bind((local_ip, detection_port)).await?;
        socket.set_broadcast(true)?;
        socket.connect((lidar_ip, detection_port)).await?;

        Ok(Self {
            socket,
            broadcast_socket,
            buffer: vec![0; Self::DETECTION_BUFFER_SIZE],
        })
    }
}

impl super::LidarStream for DetectionStream {
    type Packet<'a> = &'a DetectionData;
    async fn from_config(config: &super::LidarConfig) -> Result<Self, io::Error> {
        Self::new(config.local_ip, config.lidar_ip, config.detection_port).await
    }

    async fn next_packet(&mut self) -> Result<Self::Packet<'_>, io::Error> {
        let buf = self.buffer.as_mut();

        let mut packet = SdkPacketOwned::new_with_data(
            CommandType::Cmd,
            SendType::HostSend,
            CommandDataRef::LidarSearch(&[]),
        );

        let len = packet.read(buf)?;

        self.socket.send(&buf[..len]).await?;
        let dst_addr = self.socket.peer_addr()?;

        // TODO: use timeout
        let len = loop {
            let (len, src) = self.broadcast_socket.recv_from(buf).await?;
            if src == dst_addr {
                break len;
            }
        };

        let packet = SdkPacketRef::try_ref_from_bytes(&buf[..len])?;

        #[expect(irrefutable_let_patterns)]
        if let CommandDataRef::LidarSearch(data) = packet.data {
            data.first().filter(|x| x.is_valid())
        } else {
            None
        }
        .ok_or(io::ErrorKind::InvalidData.into())
    }
}
