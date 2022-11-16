/// Copyright (c) 2021 Marcos Pontes
/// MIT License
///
use serde::{Deserialize, Serialize};

use super::byte_rpr::{ByteRpr, FixedByteLen};

///
/// External memory block segment
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct FileSegment {
    /// Start offset for the segment in a given io device.
    pub start: u64,
    /// End offsets for the segment in a given io device.
    pub end: u64,
}

// Byte_rpr implementations
impl ByteRpr for FileSegment {
    fn as_byte_rpr(&self, buff: &mut dyn std::io::Write) -> usize {
        self.start.as_byte_rpr(buff) + self.end.as_byte_rpr(buff)
    }

    fn from_byte_rpr(bytes: &[u8]) -> Self {
        let (start_s, start_e) = (0, u64::segment_len());
        let (end_s, end_e) = (start_e, start_e + u64::segment_len());
        FileSegment {
            start: u64::from_byte_rpr(&bytes[start_s..start_e]),
            end: u64::from_byte_rpr(&bytes[end_s..end_e]),
        }
    }
}

impl FixedByteLen for FileSegment {
    fn segment_len() -> usize {
        2 * u64::segment_len()
    }
}

#[cfg(test)]
mod file_segment_test_serialization {
    use super::*;
    pub fn test_segments(len: usize) -> Vec<FileSegment> {
        let mut segments = Vec::with_capacity(len);
        for i in 0..len {
            segments.push(FileSegment {
                start: i as u64,
                end: i as u64,
            });
        }
        segments
    }
    #[test]
    fn serialize() {
        let fs = test_segments(1);
        assert_eq!(fs[0].alloc_byte_rpr().len(), FileSegment::segment_len());
        assert_eq!(fs[0].alloc_byte_rpr().len(), fs[0].as_byte_rpr(&mut vec![]));
        assert_eq!(FileSegment::from_byte_rpr(&fs[0].alloc_byte_rpr()), fs[0]);
    }
}
