use crc::Crc;

/// `CRC16-IBM-3740`, same as `false CCITT`.
pub const CRC16: Crc<u16> = Crc::<u16>::new(&crc::CRC_16_IBM_3740);
/// `CRC32-ISO-HDLC``, same as `Ethernet`.
pub const CRC32: Crc<u32> = Crc::<u32>::new(&crc::CRC_32_ISO_HDLC);
