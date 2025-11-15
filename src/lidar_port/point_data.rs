use std::io;

use async_net::{AsyncToSocketAddrs, UdpSocket};
use zerocopy::TryFromBytes;

use crate::{
    lidar_port::SocketPortConfig,
    types::ethernet::{
        CartesianHighPoint, CartesianLowPoint, EthernetPacketHeader, PointDataType, SphericalPoint,
    },
};

pub struct PointDataPort {
    socket: UdpSocket,
    buffer: Vec<u8>,
}

impl PointDataPort {
    ///  1024 * 1024 * 200 in orginal livox sdk
    const DEFAULT_BUFFER_INIT_SIZE: usize = 1380;

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

    /// # Error
    ///
    /// Fail if the socket is not connected.
    pub async fn next_packet_ref(&mut self) -> Result<PointPacketRef<'_>, io::Error> {
        let buffer = self.buffer.as_mut();
        let len = self.socket.recv(buffer).await?;
        PointPacketRef::try_from_bytes(&buffer[..len]).map_err(From::from)
    }
}

impl SocketPortConfig {
    pub const fn new_point_data_config() -> Self {
        Self {
            local: 56301,
            lidar: 56300,
        }
    }
}

impl super::IpConfig {
    pub async fn new_point_data_port(
        &self,
        point_data_port: &SocketPortConfig,
        buffer_init_size: usize,
    ) -> Result<PointDataPort, io::Error> {
        PointDataPort::new(
            (self.local, point_data_port.local),
            (self.lidar, point_data_port.lidar),
            buffer_init_size,
        )
        .await
    }
    pub async fn new_default_point_data_port(&self) -> Result<PointDataPort, io::Error> {
        self.new_point_data_port(
            &SocketPortConfig::new_imu_port_config(),
            PointDataPort::DEFAULT_BUFFER_INIT_SIZE,
        )
        .await
    }
}

impl super::LidarPortConfig {
    pub async fn new_point_data_port(&self) -> Result<PointDataPort, io::Error> {
        self.ip
            .new_point_data_port(&self.port, self.buffer_init_size)
            .await
    }
}

pub struct PointPacketRef<'a> {
    pub header: &'a EthernetPacketHeader,
    pub data: CoordinateData<'a>,
}

#[derive(Debug)]
pub enum CoordinateData<'a> {
    CartesianHigh(&'a [CartesianHighPoint]),
    CartesianLow(&'a [CartesianLowPoint]),
    Spherical(&'a [SphericalPoint]),
}

impl<'a> PointPacketRef<'a> {
    pub fn try_from_bytes(source: &'a [u8]) -> Result<Self, crate::Error> {
        let (header, data) = EthernetPacketHeader::try_ref_from_prefix(source)?;
        let dot_num = header.dot_num as usize;
        let data = CoordinateData::try_from_bytes_with_elems(data, header.data_type, dot_num)?;
        Ok(Self { header, data })
    }
}

impl<'a> CoordinateData<'a> {
    pub fn try_from_bytes_with_elems(
        source: &'a [u8],
        data_type: PointDataType,
        count: usize,
    ) -> Result<Self, crate::Error> {
        let data = match data_type {
            PointDataType::CartesianCoordinateHighData => {
                <[CartesianHighPoint]>::try_ref_from_bytes_with_elems(source, count)
                    .map(CoordinateData::CartesianHigh)?
            }
            PointDataType::CartesianCoordinateLowData => {
                <[CartesianLowPoint]>::try_ref_from_bytes_with_elems(source, count)
                    .map(CoordinateData::CartesianLow)?
            }
            PointDataType::SphericalCoordinateData => {
                <[SphericalPoint]>::try_ref_from_bytes_with_elems(source, count)
                    .map(CoordinateData::Spherical)?
            }
            PointDataType::ImuData => {
                return Err(crate::Error::unknown_type(
                    format!(
                        "{} | {} | {}",
                        PointDataType::CartesianCoordinateHighData,
                        PointDataType::CartesianCoordinateLowData,
                        PointDataType::SphericalCoordinateData
                    ),
                    PointDataType::ImuData,
                ));
            }
        };
        Ok(data)
    }
}
