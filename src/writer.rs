use crate::{debug, FleaBitReader};
use bitvec::vec::BitVec;
use std::fmt::{Debug, Formatter};

#[derive(Clone)]
pub struct FleaBitWriter(BitVec<u8>);

impl FleaBitWriter {
    pub fn new() -> Self {
        Self(BitVec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn bool(&mut self, v: bool) {
        self.0.push(v);
    }

    pub fn set_bool(&mut self, pos: usize, v: bool) {
        self.0.set(pos, v);
    }

    pub fn u8(&mut self, v: u8) {
        let b = BitVec::<u8>::from_element(v);
        self.0.extend_from_bitslice(&b);
    }

    pub fn u8_part(&mut self, v: u8, bits: usize) {
        let b = BitVec::<u8>::from_element(v);
        let b = &b[..bits];
        self.0.extend_from_bitslice(b);
    }

    pub fn u16(&mut self, v: u16) {
        let v = v.to_le_bytes();
        let b = BitVec::<u8>::from_slice(&v);
        self.0.extend_from_bitslice(&b);
    }

    pub fn u16_part(&mut self, v: u16, bits: usize) {
        let v = v.to_le_bytes();
        let b = BitVec::<u8>::from_slice(&v);
        let b = &b[..bits];
        self.0.extend_from_bitslice(&b);
    }

    pub fn u32(&mut self, v: u32) {
        let v = v.to_le_bytes();
        let b = BitVec::<u8>::from_slice(&v);
        self.0.extend_from_bitslice(&b);
    }

    pub fn u32_part(&mut self, v: u32, bits: usize) {
        let v = v.to_le_bytes();
        let b = BitVec::<u8>::from_slice(&v);
        let b = &b[..bits];
        self.0.extend_from_bitslice(&b);
    }

    pub fn usize(&mut self, v: usize) {
        let v = v.to_le_bytes();
        let b = BitVec::<u8>::from_slice(&v);
        self.0.extend_from_bitslice(&b);
    }

    pub fn usize_part(&mut self, v: usize, bits: usize) {
        let v = v.to_le_bytes();
        let b = BitVec::<u8>::from_slice(&v);
        let b = &b[..bits];
        self.0.extend_from_bitslice(b);
    }

    pub fn bytes(&mut self, v: &[u8]) {
        let b = BitVec::<u8>::from_slice(v);
        self.0.extend_from_bitslice(&b);
    }

    /// Extend bits until there is a whole byte.
    pub fn pad(&mut self) {
        let len = self.0.len();
        let rem = len % 8;
        if rem == 0 {
            return;
        }

        let pad = 8 - rem;
        for _ in 0..pad {
            self.bool(false);
        }
    }

    /// Warning: This will use the whole set of data, not using pos.
    pub fn extend_all(&mut self, v: &FleaBitReader) {
        self.0.extend_from_bitslice(&v.bits);
    }

    pub fn into_vec(self) -> Vec<u8> {
        self.0.into_vec()
    }

    pub fn to_string(&self) -> String {
        let reader = self.clone().into_reader();
        debug(&reader)
    }

    pub fn into_reader(self) -> FleaBitReader {
        FleaBitReader {
            bits: self.0,
            pos: 0,
        }
    }
}

impl Debug for FleaBitWriter {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FleaBitWriter")
            .field("bits", &self.to_string())
            .field("len", &self.len())
            .finish()
    }
}
