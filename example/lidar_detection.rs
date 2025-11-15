use livox2::lidar_port::IpConfig;
fn main() -> Result<(), std::io::Error> {
    smol::block_on(async {
        let mut detection_port = IpConfig::new([192, 168, 1, 100], [192, 168, 1, 101])
            .new_default_detection_port()
            .await?;
        let ack = detection_port.next_packet_ref().await?;
        dbg!(ack.header);
        dbg!(ack.data);
        Ok(())
    })
}
