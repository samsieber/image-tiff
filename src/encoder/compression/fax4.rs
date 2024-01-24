use std::io::Write;
use std::io;

use fax::{Color, VecWriter};

use crate::tags::CompressionMethod;

use super::{Compression, CompressionAlgorithm, Compressor};

/// The LZW algorithm used to compress image data in TIFF files.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Fax4{
    bits_per_row: usize,
}


impl Fax4 {
    pub fn with_width(samples_per_row: usize) -> Fax4 {
        Fax4{
            bits_per_row: samples_per_row,
        }
    }
}
impl Compression for Fax4 {
    const COMPRESSION_METHOD: CompressionMethod = CompressionMethod::Fax4;

    fn get_algorithm(&self, width: usize) -> Compressor {
        Compressor::Fax4(Fax4::with_width(width))
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

impl CompressionAlgorithm for Fax4 {
    fn write_to<W: Write>(&mut self, writer: &mut W, bytes: &[u8]) -> Result<u64, io::Error> {
        let vec_writer = VecWriter::new();
        let mut encoder = fax::encoder::Encoder::new(vec_writer);
        let bytes_per_row = crate::util::usize_div_ceil(self.bits_per_row, 8);
        let rows = bytes.chunks(bytes_per_row);
        let mut first = true;
        for row in rows {
            let pels = row.into_iter()
                .map(|byte| u8::MAX - byte) // upstream assumes that min is white, so invert it here...
                .flat_map(to_pels).take(self.bits_per_row);
            if first {
                first = false;
            }
            encoder.encode_line(pels, self.bits_per_row as u16) // TODO: return error instead? We might want to error on dimensions instead...
        }
        let vec_writer = encoder.finish();
        let buf = vec_writer.finish();
        writer.write_all(&buf)?;
        Ok(buf.len() as u64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_fax() {
        Fax4::with_width(100);
        todo!();
    }

    #[test]
    fn test_pels() {
        assert_eq!([
            Color::White, Color::White, Color::White, Color::White, Color::White, Color::White, Color::White, Color::White
        ], to_pels(255u8))
    }
}
