use std::io;

use async_net::{AsyncToSocketAddrs, UdpSocket};
use zerocopy::TryFromBytes;

use crate::types::ethernet::{EthernetPacketHeader, ImuData, PointDataType};

use super::SocketPortConfig;

pub struct ImuPort {
    socket: UdpSocket,
    buffer: Vec<u8>,
}

impl ImuPort {
    const DEFAULT_BUFFER_INIT_SIZE: usize = 60;

    pub async fn new(
        local_addr: impl AsyncToSocketAddrs,
        lidar_addr: impl AsyncToSocketAddrs,
        buffer_init_size: usize,
    ) -> Result<Self, io::Error> {
        let socket = UdpSocket::bind(local_addr).await?;
        socket.connect(lidar_addr).await?;

        Ok(Self {
            socket,
            buffer: vec![0; buffer_init_size],
        })
    }

    pub async fn next_packet_ref(&mut self) -> Result<ImuPacketRef<'_>, io::Error> {
        let buffer = self.buffer.as_mut();
        let len = self.socket.recv(buffer).await?;
        ImuPacketRef::try_from_bytes(&buffer[..len]).map_err(From::from)
    }
}

impl SocketPortConfig {
    pub const fn new_imu_port_config() -> Self {
        Self {
            local: 56401,
            lidar: 56400,
        }
    }
}

impl super::IpConfig {
    pub async fn new_imu_port(
        &self,
        imu_port: &SocketPortConfig,
        buffer_init_size: usize,
    ) -> Result<ImuPort, io::Error> {
        ImuPort::new(
            (self.local, imu_port.local),
            (self.lidar, imu_port.lidar),
            buffer_init_size,
        )
        .await
    }
    pub async fn new_default_imu_port(&self) -> Result<ImuPort, io::Error> {
        self.new_imu_port(
            &SocketPortConfig::new_imu_port_config(),
            ImuPort::DEFAULT_BUFFER_INIT_SIZE,
        )
        .await
    }
}

impl super::LidarPortConfig {
    pub async fn new_imu_port(&self) -> Result<ImuPort, io::Error> {
        self.ip
            .new_imu_port(&self.port, self.buffer_init_size)
            .await
    }
}

pub struct ImuPacketRef<'a> {
    pub header: &'a EthernetPacketHeader,
    pub data: &'a ImuData,
}

impl<'a> ImuPacketRef<'a> {
    pub fn try_from_bytes(source: &'a [u8]) -> Result<Self, crate::Error> {
        let (header, data) = EthernetPacketHeader::try_ref_from_prefix(source)?;

        let PointDataType::ImuData = header.data_type else {
            return Err(crate::Error::unknown_type(
                PointDataType::ImuData,
                header.data_type,
            ));
        };
        let data = ImuData::try_ref_from_bytes(data)?;
        Ok(Self { header, data })
    }
}
