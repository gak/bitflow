use crate::debug;
use bitvec::field::BitField;
use bitvec::vec::BitVec;
use std::fmt::{Debug, Formatter};

#[derive(Clone)]
pub struct FleaBitReader {
    pub(crate) bits: BitVec<u8>,
    pub(crate) pos: usize,
}

impl FleaBitReader {
    pub fn from_vec(v: Vec<u8>) -> Self {
        Self {
            bits: BitVec::from_vec(v),
            pos: 0,
        }
    }

    pub fn from_slice(v: &[u8]) -> Self {
        Self {
            bits: BitVec::from_slice(v),
            pos: 0,
        }
    }

    pub fn bool(&mut self) -> bool {
        let b = self.bits[self.pos];
        self.pos += 1;
        b
    }

    pub fn u8(&mut self) -> u8 {
        let b = self.bits[self.pos..self.pos + 8].load();
        self.pos += 8;
        b
    }

    pub fn u8_part(&mut self, bits: usize) -> u8 {
        let b = self.bits[self.pos..self.pos + bits].load();
        self.pos += bits;
        b
    }

    pub fn u16(&mut self) -> u16 {
        let b = self.bits[self.pos..self.pos + 16].load();
        self.pos += 16;
        b
    }

    pub fn u16_part(&mut self, bits: usize) -> u16 {
        let b = self.bits[self.pos..self.pos + bits].load();
        self.pos += bits;
        b
    }

    pub fn u32(&mut self) -> u32 {
        let b = self.bits[self.pos..self.pos + 32].load();
        self.pos += 32;
        b
    }

    pub fn usize(&mut self) -> usize {
        let bits = usize::BITS as usize;
        let b = self.bits[self.pos..self.pos + bits].load();
        self.pos += bits;
        b
    }

    pub fn usize_part(&mut self, bits: usize) -> usize {
        let b = self.bits[self.pos..self.pos + bits].load();
        self.pos += bits;
        b
    }

    pub fn bytes(&mut self, bytes_len: usize) -> Vec<u8> {
        let bits_len = bytes_len * 8;
        let mut b = self.bits[self.pos..self.pos + bits_len].to_bitvec();
        b.force_align();
        self.pos += bytes_len * 8;
        b.into_vec()
    }

    /// Create a new Reader with the remaining bits.
    pub fn crop_end(&mut self) -> Self {
        let bits = self.bits[self.pos..].to_bitvec();
        Self { bits, pos: 0 }
    }

    pub fn len(&self) -> usize {
        self.bits.len()
    }

    pub fn remaining_len(&self) -> usize {
        self.bits.len() - self.pos
    }

    pub fn is_end(&self) -> bool {
        self.pos >= self.bits.len()
    }

    pub fn to_string(&self) -> String {
        debug(&self)
    }
}

impl Debug for FleaBitReader {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FleaBitReader")
            .field("bits", &self.to_string())
            .field("pos", &self.pos)
            .field("len", &self.len())
            .finish()
    }
}

impl From<Vec<u8>> for FleaBitReader {
    fn from(v: Vec<u8>) -> Self {
        Self::from_vec(v)
    }
}

impl From<&[u8]> for FleaBitReader {
    fn from(v: &[u8]) -> Self {
        Self::from_slice(v)
    }
}
