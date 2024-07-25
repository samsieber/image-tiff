use std::io::Write;
use std::io;

use fax::{Color, VecWriter};

use crate::tags::CompressionMethod;

use super::{Compression, CompressionAlgorithm, Compressor};

/// The LZW algorithm used to compress image data in TIFF files.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct CcittG4{
    bits_per_row: usize,
}


impl CcittG4 {
    pub fn with_width(samples_per_row: usize) -> CcittG4 {
        CcittG4{
            bits_per_row: samples_per_row,
        }
    }
}
impl Compression for CcittG4 {
    const COMPRESSION_METHOD: CompressionMethod = CompressionMethod::Fax4;

    fn get_algorithm(&self, width: usize) -> Compressor {
        Compressor::CcittG4(CcittG4::with_width(width))
    }
}

fn to_pels(v: u8) -> [Color; 8] {
    [
        if v & 0b1000_0000 > 0 { Color::White } else { Color::Black },
        if v & 0b0100_0000 > 0 { Color::White } else { Color::Black },
        if v & 0b0010_0000 > 0 { Color::White } else { Color::Black },
        if v & 0b0001_0000 > 0 { Color::White } else { Color::Black },
        if v & 0b0000_1000 > 0 { Color::White } else { Color::Black },
        if v & 0b0000_0100 > 0 { Color::White } else { Color::Black },
        if v & 0b0000_0010 > 0 { Color::White } else { Color::Black },
        if v & 0b0000_0001 > 0 { Color::White } else { Color::Black },
    ]
}

impl CompressionAlgorithm for CcittG4 {
    fn write_to<W: Write>(&mut self, writer: &mut W, bytes: &[u8]) -> Result<u64, io::Error> {
        let mut encoder = ccitt_t4_t6::g42d::encode::Encoder::new(self.bits_per_row, bytes);
        encoder.skip_tail = 8 - (self.bits_per_row % 8);
        let buf = encoder.encode();
        writer.write_all(&buf)?;
        Ok(buf.len() as u64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_fax() {
        // use crate::upser::Fax4Fax4::with_width(100);
        // todo!();
    }

    #[test]
    fn test_pels() {
        assert_eq!([
            Color::White, Color::White, Color::White, Color::White, Color::White, Color::White, Color::White, Color::White
        ], to_pels(255u8))
    }
}
