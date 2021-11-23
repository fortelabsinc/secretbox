use crate::salsa20::implementation::salsa20_rounds;

pub const SIGMA: [u8; 16] = [
    0x65, 0x78, 0x70, 0x61, 0x6e, 0x64, 0x20, 0x33, 0x32, 0x2d, 0x62, 0x79, 0x74, 0x65, 0x20, 0x6B,
];

pub fn hsalsa20(data: [u8; 16], k: [u8; 32], c: [u8; 16]) -> [u8; 32] {
    let data = [
        u32::from_le_bytes([c[0], c[1], c[2], c[3]]),
        u32::from_le_bytes([k[0], k[1], k[2], k[3]]),
        u32::from_le_bytes([k[4], k[5], k[6], k[7]]),
        u32::from_le_bytes([k[8], k[9], k[10], k[11]]),
        u32::from_le_bytes([k[12], k[13], k[14], k[15]]),
        u32::from_le_bytes([c[4], c[5], c[6], c[7]]),
        u32::from_le_bytes([data[0], data[1], data[2], data[3]]),
        u32::from_le_bytes([data[4], data[5], data[6], data[7]]),
        u32::from_le_bytes([data[8], data[9], data[10], data[11]]),
        u32::from_le_bytes([data[12], data[13], data[14], data[15]]),
        u32::from_le_bytes([c[8], c[9], c[10], c[11]]),
        u32::from_le_bytes([k[16], k[17], k[18], k[19]]),
        u32::from_le_bytes([k[20], k[21], k[22], k[23]]),
        u32::from_le_bytes([k[24], k[25], k[26], k[27]]),
        u32::from_le_bytes([k[28], k[29], k[30], k[31]]),
        u32::from_le_bytes([c[12], c[13], c[14], c[15]]),
    ];
    let data = salsa20_rounds(data);
    let x0 = data[0].to_le_bytes();
    let x5 = data[5].to_le_bytes();
    let x6 = data[6].to_le_bytes();
    let x7 = data[7].to_le_bytes();
    let x8 = data[8].to_le_bytes();
    let x9 = data[9].to_le_bytes();
    let x10 = data[10].to_le_bytes();
    let x15 = data[15].to_le_bytes();
    [
        x0[0], x0[1], x0[2], x0[3], x5[0], x5[1], x5[2], x5[3], x10[0], x10[1], x10[2], x10[3],
        x15[0], x15[1], x15[2], x15[3], x6[0], x6[1], x6[2], x6[3], x7[0], x7[1], x7[2], x7[3],
        x8[0], x8[1], x8[2], x8[3], x9[0], x9[1], x9[2], x9[3],
    ]
}

pub fn generate_subkey(nonce: [u8; 24], key: [u8; 32]) -> ([u8; 32], [u8; 8]) {
    let hnonce = [
        nonce[0], nonce[1], nonce[2], nonce[3], nonce[4], nonce[5], nonce[6], nonce[7], nonce[8],
        nonce[9], nonce[10], nonce[11], nonce[12], nonce[13], nonce[14], nonce[15],
    ];
    let subkey = hsalsa20(hnonce, key, SIGMA);
    (
        subkey,
        [
            nonce[16], nonce[17], nonce[18], nonce[19], nonce[20], nonce[21], nonce[22], nonce[23],
        ],
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn core() {
        let expected = [
            0x1b, 0x27, 0x55, 0x64, 0x73, 0xe9, 0x85, 0xd4, 0x62, 0xcd, 0x51, 0x19, 0x7a, 0x9a,
            0x46, 0xc7, 0x60, 0x09, 0x54, 0x9e, 0xac, 0x64, 0x74, 0xf2, 0x06, 0xc4, 0xee, 0x08,
            0x44, 0xf6, 0x83, 0x89,
        ];
        let input = [0u8; 16];
        let shared = [
            0x4a, 0x5d, 0x9d, 0x5b, 0xa4, 0xce, 0x2d, 0xe1, 0x72, 0x8e, 0x3b, 0xf4, 0x80, 0x35,
            0x0f, 0x25, 0xe0, 0x7e, 0x21, 0xc9, 0x47, 0xd1, 0x9e, 0x33, 0x76, 0xf0, 0x9b, 0x3c,
            0x1e, 0x16, 0x17, 0x42,
        ];
        assert_eq!(hsalsa20(input, shared, SIGMA), expected);
    }
}
