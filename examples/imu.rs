use livox2::lidar_port::IpConfig;

fn main() -> Result<(), std::io::Error> {
    smol::block_on(async {
        let mut imu_port = IpConfig::new([192, 168, 1, 100], [192, 168, 1, 101])
            .new_default_imu_port()
            .await?;
        let packet = imu_port.next_packet_ref().await?;
        dbg!(packet.header);
        dbg!(&packet.data);
        Ok(())
    })
}
