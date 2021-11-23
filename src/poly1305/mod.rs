//! The Poly1305 message authentication algorithm
use crate::U256;
use std::cmp::min;

/// This function clamps r to be in the correct format
fn clamp(input: u128) -> u128 {
    input & 0xffffffc0ffffffc0ffffffc0fffffff
}

pub const PRIME: U256 = U256([0xffff_ffff_ffff_fffb, 0xffff_ffff_ffff_ffff, 3, 0]);

/// Poly1305 structure
pub struct Poly1305 {
    acc: U256,
    r: u128,
    s: u128,
}

impl Poly1305 {
    pub fn new(r: u128, s: u128) -> Poly1305 {
        Poly1305 {
            acc: U256::zero(),
            r: clamp(r),
            s,
        }
    }
    /// Reads one block. Panics if the size is larger than 16
    pub fn read_block(&mut self, x: &[u8]) {
        assert!(x.len() <= 16);
        let mut block = Vec::new();
        block.extend_from_slice(x);
        block.push(1);
        while block.len() != 32 {
            block.push(0);
        }
        let b = U256::from_little_endian(&block);
        self.acc = self
            .acc
            .overflowing_add(b)
            .0
            .overflowing_mul(U256::from(self.r))
            .0
            % PRIME;
    }
    /// Returns the finalized hash. This struct can still be used to extend the message if necessary
    /// (which it shouldn't be)
    pub fn finalize(&self) -> u128 {
        (self.acc + U256::from(self.s)).low_u128()
    }
    /// Hashes a message, one block at a time, then finalizes the output
    pub fn hash(&mut self, data: &[u8]) -> u128 {
        for i in 0..((data.len() + 15) / 16) {
            self.read_block(&data[(i * 16)..min((i + 1) * 16, data.len())]);
        }
        self.finalize()
    }
    /// Verifies a message based on a certain hash
    pub fn verify(&mut self, data: &[u8], expected: u128) -> bool {
        self.hash(data) == expected
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn ietf_test_vector() {
        let s = 0x1bf54941aff6bf4afdb20dfb8a800301;
        let r = 0x0806d5400e52447c036d555408bed685;
        let message = b"Cryptographic Forum Research Group";
        let mut hasher = Poly1305::new(r, s);
        assert_eq!(
            hasher.hash(&message[..]),
            0xa927010caf8b2bc2c6365130c11d06a8
        );
    }
}
