//! This module contains primitives for generating CSPRNG data using the chacha20 algorithm

#[cfg(feature = "simd")]
pub mod simd;
#[cfg(not(any(feature = "asm", feature = "simd")))]
pub mod trivial;

#[cfg(feature = "simd")]
pub use simd as implementation;
#[cfg(not(any(feature = "asm", feature = "simd")))]
pub use trivial as implementation;

pub use implementation::chacha20;

/// SIGMA constant used to expand a 32-byte key
/// Reads "expand 32-byte k"
pub const SIGMA: [u32; 4] = [0x6170_7865, 0x3320_646E, 0x7962_2d32, 0x6B20_6574];

/// XChacha20 structure. 32-byte key, 16-byte sigma, 12-byte nonce, 4-byte counter
#[derive(Clone)]
pub struct XChacha20 {
    key: [u32; 8],
    nonce: [u32; 2],
    ctr: u64,
}

impl XChacha20 {
    /// Creates a new XChacha20 struct. You need to provide a key, nonce and starting CTR
    pub fn new(key: [u8; 32], nonce: [u8; 8], ctr: u64) -> XChacha20 {
        XChacha20 {
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
            SIGMA[1],
            SIGMA[2],
            SIGMA[3],
            self.key[0],
            self.key[1],
            self.key[2],
            self.key[3],
            self.key[4],
            self.key[5],
            self.key[6],
            self.key[7],
            self.ctr as u32,
            (self.ctr >> 32) as u32,
            self.nonce[0],
            self.nonce[1],
        ];
        let output = chacha20(input);
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
        let mut cipher = XChacha20::new(
            [
                0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22,
                23, 24, 25, 26, 27, 28, 29, 30, 31,
            ],
            [0, 0, 0, 0x4a, 0, 0, 0, 0],
            1,
        );
        let mut text = b"Ladies and Gentlemen of the class of '99: If I could offer you only one tip for the future, sunscreen would be it.".clone();
        cipher.crypt(&mut text[..]);
        assert_eq!(
            &text[..],
            &[
                0x6e, 0x2e, 0x35, 0x9a, 0x25, 0x68, 0xf9, 0x80, 0x41, 0xba, 0x07, 0x28, 0xdd, 0x0d,
                0x69, 0x81, 0xe9, 0x7e, 0x7a, 0xec, 0x1d, 0x43, 0x60, 0xc2, 0x0a, 0x27, 0xaf, 0xcc,
                0xfd, 0x9f, 0xae, 0x0b, 0xf9, 0x1b, 0x65, 0xc5, 0x52, 0x47, 0x33, 0xab, 0x8f, 0x59,
                0x3d, 0xab, 0xcd, 0x62, 0xb3, 0x57, 0x16, 0x39, 0xd6, 0x24, 0xe6, 0x51, 0x52, 0xab,
                0x8f, 0x53, 0x0c, 0x35, 0x9f, 0x08, 0x61, 0xd8, 0x07, 0xca, 0x0d, 0xbf, 0x50, 0x0d,
                0x6a, 0x61, 0x56, 0xa3, 0x8e, 0x08, 0x8a, 0x22, 0xb6, 0x5e, 0x52, 0xbc, 0x51, 0x4d,
                0x16, 0xcc, 0xf8, 0x06, 0x81, 0x8c, 0xe9, 0x1a, 0xb7, 0x79, 0x37, 0x36, 0x5a, 0xf9,
                0x0b, 0xbf, 0x74, 0xa3, 0x5b, 0xe6, 0xb4, 0x0b, 0x8e, 0xed, 0xf2, 0x78, 0x5e, 0x42,
                0x87, 0x4d
            ][..]
        );
    }
}
