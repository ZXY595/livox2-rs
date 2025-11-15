use livox2::lidar_port::{IpConfig, point_data::CoordinateData};

fn main() -> Result<(), std::io::Error> {
    smol::block_on(async {
        let mut point_port = IpConfig::new([192, 168, 1, 100], [192, 168, 1, 101])
            .new_default_point_data_port()
            .await?;
        let packet = point_port.next_packet_ref().await?;
        dbg!(packet.header);
        match packet.data {
            CoordinateData::CartesianHigh(cartesian_high_points) => {
                dbg!(&cartesian_high_points[..5]);
            }
            CoordinateData::CartesianLow(cartesian_low_points) => {
                dbg!(&cartesian_low_points[..5]);
            }
            CoordinateData::Spherical(spherical_points) => {
                dbg!(&spherical_points[..5]);
            }
        }
        Ok(())
    })
}
