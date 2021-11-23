//! This module contains primitives for generating CSPRNG data using the salsa20 algorithm

#[cfg(feature = "simd")]
pub mod simd;
#[cfg(not(any(feature = "asm", feature = "simd")))]
pub mod trivial;

#[cfg(feature = "simd")]
pub use simd as implementation;
#[cfg(not(any(feature = "asm", feature = "simd")))]
pub use trivial as implementation;

pub use implementation::salsa20;

/// SIGMA constant used to expand a 32-byte key
/// Reads "expand 32-byte k"
pub const SIGMA: [u32; 4] = [0x61707865, 0x3320646e, 0x79622d32, 0x6b206574];

/// XSalsa20 structure. 32-byte key, 16-byte sigma, 12-byte nonce, 4-byte counter
#[derive(Clone)]
pub struct XSalsa20 {
    key: [u32; 8],
    nonce: [u32; 2],
    ctr: u64,
}
impl XSalsa20 {
    /// Creates a new XSalsa20 struct. You need to provide a key, nonce and starting CTR
    pub fn new(key: [u8; 32], nonce: [u8; 8], ctr: u64) -> XSalsa20 {
        XSalsa20 {
            ctr,
            nonce: [
                u32::from_le_bytes([nonce[0], nonce[1], nonce[2], nonce[3]]),
                u32::from_le_bytes([nonce[4], nonce[5], nonce[6], nonce[7]]),
            ],
            key: [
                u32::from_le_bytes([key[0], key[1], key[2], key[3]]),
                u32::from_le_bytes([key[4], key[5], key[6], key[7]]),
                u32::from_le_bytes([key[8], key[9], key[10], key[11]]),
                u32::from_le_bytes([key[12], key[13], key[14], key[15]]),
                u32::from_le_bytes([key[16], key[17], key[18], key[19]]),
                u32::from_le_bytes([key[20], key[21], key[22], key[23]]),
                u32::from_le_bytes([key[24], key[25], key[26], key[27]]),
                u32::from_le_bytes([key[28], key[29], key[30], key[31]]),
            ],
        }
    }
    /// Generates a single block of Salsa20 random data
    pub fn generate_block(&mut self) -> Option<Vec<u8>> {
        let input = [
            SIGMA[0],
            self.key[0],
            self.key[1],
            self.key[2],
            self.key[3],
            SIGMA[1],
            self.nonce[0],
            self.nonce[1],
            self.ctr as u32,
            (self.ctr >> 32) as u32,
            SIGMA[2],
            self.key[4],
            self.key[5],
            self.key[6],
            self.key[7],
            SIGMA[3],
        ];
        let output = salsa20(input);
        self.ctr = self.ctr.checked_add(1)?;
        let mut out_bytes = Vec::new();
        for w in output.iter() {
            out_bytes.extend_from_slice(&w.to_le_bytes());
        }
        Some(out_bytes)
    }
    /// Generates a certain amount of Salsa20 random data
    pub fn generate(&mut self, mut amount: usize) -> Option<Vec<u8>> {
        let mut data = Vec::new();
        while amount > 64 {
            data.append(&mut self.generate_block()?);
            amount -= 64;
        }
        if amount == 0 {
            return Some(data);
        }
        data.extend_from_slice(&self.generate_block()?[..amount]);
        Some(data)
    }
    /// Encrypts/Decrypts a slice of data in-place
    pub fn crypt(&mut self, data: &mut [u8]) -> Option<()> {
        let mut size = data.len();
        let mut offset = 0;
        while size > 64 {
            let block = self.generate_block()?;
            for i in 0..64 {
                data[offset + i] ^= block[i];
            }
            size -= 64;
            offset += 64;
        }
        if size == 0 {
            return Some(());
        }
        let block = self.generate(size)?;
        for i in 0..block.len() {
            data[offset + i] ^= block[i];
        }
        Some(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_vectors() {
        let mut cipher1 = XSalsa20::new(
            [
                1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 201, 202, 203, 204, 205,
                206, 207, 208, 209, 210, 211, 212, 213, 214, 215, 216,
            ],
            [101, 102, 103, 104, 105, 106, 107, 108],
            0x74737271706f6e6d,
        );
        let received = cipher1.generate_block().unwrap();
        let received_slice = &received[..];
        let expected = &[
            69, 37, 68, 39, 41, 15, 107, 193, 255, 139, 122, 6, 170, 233, 217, 98, 89, 144, 182,
            106, 21, 51, 200, 65, 239, 49, 222, 34, 215, 114, 40, 126, 104, 197, 7, 225, 197, 153,
            31, 2, 102, 78, 76, 176, 84, 245, 246, 184, 177, 160, 133, 130, 6, 72, 149, 119, 192,
            195, 132, 236, 234, 103, 246, 74,
        ][..];
        assert_eq!(received_slice, expected);
    }
}
