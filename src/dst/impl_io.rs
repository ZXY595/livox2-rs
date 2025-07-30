use super::{DstHeader, DstRef};
use std::io::{Read, Write};
use zerocopy::IntoBytes;

impl<'a, H, D> std::io::Read for DstRef<'a, H, D>
where
    H: DstHeader + IntoBytes,
    D: Read,
{
    fn read(&mut self, mut buf: &mut [u8]) -> std::io::Result<usize> {
        let len = buf.write(self.header.as_bytes())? + self.data.read(buf)?;
        Ok(len)
    }
}

