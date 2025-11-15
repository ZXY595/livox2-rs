use std::{
    io::{self, Write},
    net::Ipv4Addr,
};

use async_net::UdpSocket;
use zerocopy::{Immutable, IntoBytes, TryFromBytes};

use crate::types::sdk_packet::{CommandID, CommandType, QueryDeviceTypeAck, SdkPacketHeader, SendType};

use super::SocketPortConfig;

pub struct DetectionPort {
    socket: UdpSocket,
    broadcast_socket: UdpSocket,
    buffer: Vec<u8>,
}

impl DetectionPort {
    const DEFAULT_BUFFER_INIT_SIZE: usize = 48;

    pub async fn new(
        local_ip: impl Into<Ipv4Addr>,
        lidar_ip: impl Into<Ipv4Addr>,
        detection_port: u16,
        buffer_init_size: usize,
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
            buffer: vec![0; buffer_init_size],
        })
    }
}

impl SocketPortConfig {
    const DEFAULT_DETECTION_PORT: u16 = 56000;

    pub const fn new_detection_port_config() -> Self {
        Self {
            local: Self::DEFAULT_DETECTION_PORT,
            lidar: Self::DEFAULT_DETECTION_PORT,
        }
    }
}

impl super::IpConfig {
    pub async fn new_detection_port(
        &self,
        imu_port: u16,
        buffer_init_size: usize,
    ) -> Result<DetectionPort, io::Error> {
        DetectionPort::new(self.local, self.lidar, imu_port, buffer_init_size).await
    }
    pub async fn new_default_detection_port(&self) -> Result<DetectionPort, io::Error> {
        self.new_detection_port(
            SocketPortConfig::DEFAULT_DETECTION_PORT,
            DetectionPort::DEFAULT_BUFFER_INIT_SIZE,
        )
        .await
    }
}

impl super::LidarPortConfig {
    pub async fn new_detection_port(&self) -> Result<DetectionPort, io::Error> {
        self.ip
            .new_detection_port(self.port.lidar, self.buffer_init_size)
            .await
    }
}

impl DetectionPort {
    pub async fn next_packet_ref(&mut self) -> Result<LidarSearchAckRef<'_>, io::Error> {
        let buf = &mut self.buffer;

        let packet = LidarSearchCmdPacket::new(CommandType::Cmd, SendType::HostSend);

        let len = buf.write(packet.as_bytes())?;

        self.socket.send(&buf[..len]).await?;
        let dst_addr = self.socket.peer_addr()?;

        // TODO: use timeout
        let len = loop {
            let (len, src) = self.broadcast_socket.recv_from(buf).await?;
            if src == dst_addr {
                break len;
            }
        };

        LidarSearchAckRef::try_from_bytes(&buf[..len]).map_err(From::from)
    }
}

#[derive(Immutable, IntoBytes)]
struct LidarSearchCmdPacket {
    #[expect(unused)]
    header: SdkPacketHeader,
}

impl LidarSearchCmdPacket {
    pub fn new(cmd_type: CommandType, sender_type: SendType) -> Self {
        let cmd_id = CommandID::QueryDeviceType;

        let mut header = SdkPacketHeader::new(0, cmd_id, cmd_type, sender_type);
        header.crc16_h = crate::crc::CRC16.checksum(header.as_bytes());
        header.crc32_d = crate::crc::CRC32.checksum(header.as_bytes());

        Self { header }
    }
}

pub struct LidarSearchAckRef<'a> {
    pub header: &'a SdkPacketHeader,
    pub data: &'a QueryDeviceTypeAck,
}

impl<'a> LidarSearchAckRef<'a> {
    pub fn try_from_bytes(source: &'a [u8]) -> Result<Self, crate::Error> {
        let (header, data) = SdkPacketHeader::try_ref_from_prefix(source)?;
        let cmd_id = header.cmd_id;

        let CommandID::QueryDeviceType = cmd_id else {
            return Err(crate::Error::unknown_type(
                CommandID::QueryDeviceType,
                cmd_id,
            ));
        };
        let data = QueryDeviceTypeAck::try_ref_from_bytes(&data[..header.data_len()])?;
        Ok(Self { header, data })
    }
}
