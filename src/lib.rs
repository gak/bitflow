#[doc = include_str!("../README.md")]
mod reader;
mod writer;

pub use reader::FleaBitReader;
pub use writer::FleaBitWriter;

/// Return a String that shows bits as 1/0's and is separated by underscores.
///
/// Use a `.` (full stop) to indicate there is no valid bit at the position.
pub(crate) fn debug(b: &FleaBitReader) -> String {
    let mut s = String::with_capacity(b.bits.len() * 8 + b.bits.len());
    let mut byte_bit = 0;
    loop {
        if byte_bit != 0 {
            s.push('_');
        }
        for bit in (0..8).rev() {
            let idx = byte_bit + bit;
            if idx >= b.bits.len() {
                s.push('.');
                continue;
            }

            if b.bits[idx] {
                s.push('1');
            } else {
                s.push('0');
            }
        }

        byte_bit += 8;
        if byte_bit >= b.bits.len() {
            break;
        }
    }

    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_string() {
        let mut writer = FleaBitWriter::new();
        writer.bool(false);
        assert_eq!(writer.to_string(), ".......0");

        writer.bool(true);
        assert_eq!(writer.to_string(), "......10");

        writer.u8_part(6, 6);
        assert_eq!(writer.to_string(), "00011010");

        writer.bool(true);
        assert_eq!(writer.to_string(), "00011010_.......1");
    }

    #[test]
    fn bool() {
        let mut writer = FleaBitWriter::new();
        writer.bool(false);
        assert_eq!(writer.to_string(), ".......0");

        let mut reader = writer.into_reader();
        assert_eq!(reader.bool(), false);
        assert!(reader.is_end())
    }

    #[test]
    fn basics() {
        let mut writer = FleaBitWriter::new();
        writer.bool(false);
        assert_eq!(writer.to_string(), ".......0");
        writer.u8(135);
        assert_eq!(writer.to_string(), "00001110_.......1");
        writer.u8_part(255, 2);
        assert_eq!(writer.to_string(), "00001110_.....111");
        writer.u8_part(33, 7);
        assert_eq!(writer.to_string(), "00001110_00001111_......01");

        let mut reader = writer.into_reader();
        assert_eq!(reader.bool(), false);
        assert_eq!(reader.u8(), 135);
        assert_eq!(reader.u8_part(2), 3);
        assert_eq!(reader.u8_part(7), 33);
        assert!(reader.is_end());
    }

    #[test]
    fn u16() {
        let mut writer = FleaBitWriter::new();
        writer.bool(false); // Just to be misaligned
        writer.u16(0xffff);
        assert_eq!(writer.to_string(), "11111110_11111111_.......1");

        let mut reader = writer.into_reader();
        assert_eq!(reader.bool(), false);
        assert_eq!(reader.u16(), 0xffff);
        assert!(reader.is_end());
    }

    #[test]
    fn usize() {
        let mut writer = FleaBitWriter::new();
        writer.bool(false); // Misaligned
        writer.usize(usize::MAX - 1);
        if usize::BITS == 64 {
            assert_eq!(
                writer.to_string(),
                "11111100_11111111_11111111_11111111_11111111_11111111_11111111_11111111_.......1"
            );
        } else {
            panic!("No test for 64bit usize.")
        }

        let mut reader = writer.into_reader();
        assert_eq!(reader.bool(), false);
        assert_eq!(reader.usize(), usize::MAX - 1);
        assert!(reader.is_end());
    }

    #[test]
    fn usize_sized() {
        let mut writer = FleaBitWriter::new();
        writer.bool(false); // Misaligned
        writer.usize_part(0x01ff0000ff, 33);
        writer.bool(false);
        assert_eq!(
            writer.to_string(),
            "11111110_00000001_00000000_11111110_.....011"
        );

        let mut reader = writer.into_reader();
        assert_eq!(reader.bool(), false);
        assert_eq!(reader.usize_part(33), 0x1ff0000ff);
        assert_eq!(reader.bool(), false);
        assert!(reader.is_end());
    }

    #[test]
    fn bytes_aligned() {
        let mut writer = FleaBitWriter::new();
        writer.bytes(&[255, 160]);
        assert_eq!(writer.to_string(), "11111111_10100000");

        let mut reader = writer.into_reader();
        assert_eq!(reader.bytes(2), vec![255, 160]);
    }

    #[test]
    fn bytes_misaligned() {
        let mut writer = FleaBitWriter::new();
        writer.bool(false);
        writer.bytes(&[255, 160]);
        assert_eq!(writer.to_string(), "11111110_01000001_.......1");

        let mut reader = writer.into_reader();
        assert_eq!(reader.bool(), false);
        assert_eq!(reader.bytes(2), vec![255, 160]);
    }

    #[test]
    fn pad() {
        let mut writer = FleaBitWriter::new();

        writer.pad();
        assert_eq!(writer.to_string(), "........");

        writer.bool(false);
        writer.bool(true);
        writer.pad();
        assert_eq!(writer.to_string(), "00000010");

        writer.pad();
        assert_eq!(writer.to_string(), "00000010");
    }
}
