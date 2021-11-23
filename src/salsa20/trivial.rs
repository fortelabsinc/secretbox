#[inline(always)]
pub fn quarter_round(i: (u32, u32, u32, u32)) -> (u32, u32, u32, u32) {
    let z1 = i.1 ^ i.0.wrapping_add(i.3).rotate_left(7);
    let z2 = i.2 ^ z1.wrapping_add(i.0).rotate_left(9);
    let z3 = i.3 ^ z2.wrapping_add(z1).rotate_left(13);
    let z0 = i.0 ^ z3.wrapping_add(z2).rotate_left(18);
    (z0, z1, z2, z3)
}

#[inline(always)]
pub fn row_round(i: [u32; 16]) -> [u32; 16] {
    let q1 = quarter_round((i[0], i[1], i[2], i[3]));
    let q2 = quarter_round((i[5], i[6], i[7], i[4]));
    let q3 = quarter_round((i[10], i[11], i[8], i[9]));
    let q4 = quarter_round((i[15], i[12], i[13], i[14]));
    [
        q1.0, q1.1, q1.2, q1.3, q2.3, q2.0, q2.1, q2.2, q3.2, q3.3, q3.0, q3.1, q4.1, q4.2, q4.3,
        q4.0,
    ]
}

#[inline(always)]
pub fn column_round(i: [u32; 16]) -> [u32; 16] {
    let q0 = quarter_round((i[0], i[4], i[8], i[12]));
    let q1 = quarter_round((i[5], i[9], i[13], i[1]));
    let q2 = quarter_round((i[10], i[14], i[2], i[6]));
    let q3 = quarter_round((i[15], i[3], i[7], i[11]));
    [
        q0.0, q1.3, q2.2, q3.1, q0.1, q1.0, q2.3, q3.2, q0.2, q1.1, q2.0, q3.3, q0.3, q1.2, q2.1,
        q3.0,
    ]
}
#[inline(always)]
pub fn double_round(i: [u32; 16]) -> [u32; 16] {
    row_round(column_round(i))
}

/// This is the raw salsa20 hash function
pub fn salsa20(mut input: [u32; 16]) -> [u32; 16] {
    let mut buf = (0..10).fold(input, |out, _| double_round(out));
    for i in 0..16 {
        buf[i] = buf[i].wrapping_add(input[i]);
        input[i] = 0; // Clear cleartext data from stack
    }
    buf
}

#[inline(always)]
pub(crate) fn salsa20_rounds(input: [u32; 16]) -> [u32; 16] {
    (0..10).fold(input, |out, _| double_round(out))
}

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn test_quarter_round() {
        assert_eq!(quarter_round((0, 0, 0, 0)), (0, 0, 0, 0));
        assert_eq!(
            quarter_round((1, 0, 0, 0)),
            (0x8008145, 0x80, 0x10200, 0x20500000)
        );
        assert_eq!(
            quarter_round((0, 1, 0, 0)),
            (0x88000100, 1, 0x200, 0x402000)
        );
        assert_eq!(quarter_round((0, 0, 1, 0)), (0x80040000, 0, 1, 0x2000));
        assert_eq!(
            quarter_round((0, 0, 0, 1)),
            (0x48044, 0x80, 0x10000, 0x20100001)
        );
        assert_eq!(
            quarter_round((0xe7e8c006, 0xc4f9417d, 0x6479b4b2, 0x68c67137)),
            (0xe876d72b, 0x9361dfd5, 0xf1460244, 0x948541a3)
        );
        assert_eq!(
            quarter_round((0xd3917c5b, 0x55f1c407, 0x52a58a7a, 0x8f887a3b)),
            (0x3e2f308c, 0xd90a8f36, 0x6ab2a923, 0x2883524c)
        );
    }
    #[test]
    fn test_row_round() {
        assert_eq!(
            row_round([1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0]),
            [
                0x08008145, 0x80, 0x10200, 0x20500000, 0x20100001, 0x48044, 0x80, 0x10000, 1,
                0x2000, 0x80040000, 0, 1, 0x200, 0x402000, 0x88000100
            ]
        );
        assert_eq!(
            row_round([
                0x08521bd6, 0x1fe88837, 0xbb2aa576, 0x3aa26365, 0xc54c6a5b, 0x2fc74c2f, 0x6dd39cc3,
                0xda0a64f6, 0x90a2f23d, 0x067f95a6, 0x06b35f61, 0x41e4732e, 0xe859c100, 0xea4d84b7,
                0x0f619bff, 0xbc6e965a
            ]),
            [
                0xa890d39d, 0x65d71596, 0xe9487daa, 0xc8ca6a86, 0x949d2192, 0x764b7754, 0xe408d9b9,
                0x7a41b4d1, 0x3402e183, 0x3c3af432, 0x50669f96, 0xd89ef0a8, 0x0040ede5, 0xb545fbce,
                0xd257ed4f, 0x1818882d
            ]
        );
    }
    #[test]
    fn test_column_round() {
        assert_eq!(
            column_round([1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0]),
            [0x10090288, 0, 0, 0, 0x101, 0, 0, 0, 0x20401, 0, 0, 0, 0x40a04001, 0, 0, 0]
        );
        assert_eq!(
            column_round([
                0x08521bd6, 0x1fe88837, 0xbb2aa576, 0x3aa26365, 0xc54c6a5b, 0x2fc74c2f, 0x6dd39cc3,
                0xda0a64f6, 0x90a2f23d, 0x067f95a6, 0x06b35f61, 0x41e4732e, 0xe859c100, 0xea4d84b7,
                0x0f619bff, 0xbc6e965a
            ]),
            [
                0x8c9d190a, 0xce8e4c90, 0x1ef8e9d3, 0x1326a71a, 0x90a20123, 0xead3c4f3, 0x63a091a0,
                0xf0708d69, 0x789b010c, 0xd195a681, 0xeb7d5504, 0xa774135c, 0x481c2027, 0x53a8e4b5,
                0x4c1f89c5, 0x3f78c9c8
            ]
        );
    }

    #[test]
    fn test_double_round() {
        assert_eq!(
            double_round([1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
            [
                0x8186a22d, 0x0040a284, 0x82479210, 0x06929051, 0x08000090, 0x02402200, 0x00004000,
                0x00800000, 0x00010200, 0x20400000, 0x08008104, 0x00000000, 0x20500000, 0xa0000040,
                0x0008180a, 0x612a8020
            ]
        );
        assert_eq!(
            double_round([
                0xde501066, 0x6f9eb8f7, 0xe4fbbd9b, 0x454e3f57, 0xb75540d3, 0x43e93a4c, 0x3a6f2aa0,
                0x726d6b36, 0x9243f484, 0x9145d1e8, 0x4fa9d247, 0xdc8dee11, 0x054bf545, 0x254dd653,
                0xd9421b6d, 0x67b276c1
            ]),
            [
                0xccaaf672, 0x23d960f7, 0x9153e63a, 0xcd9a60d0, 0x50440492, 0xf07cad19, 0xae344aa0,
                0xdf4cfdfc, 0xca531c29, 0x8e7943db, 0xac1680cd, 0xd503ca00, 0xa74b2ad6, 0xbc331c5c,
                0x1dda24c7, 0xee928277
            ]
        );
    }
    #[test]
    fn test_salsa20() {
        assert_eq!(salsa20([0; 16]), [0; 16]);
        assert_eq!(
            salsa20([
                0x730d9fd3, 0xb752374c, 0x25de7503, 0x88eabbbf, 0x30b3ed31, 0xdbb26a01, 0x30a6c7af,
                0xcfb31056, 0x3f20f01f, 0xa15d530f, 0x71309374, 0x24cc37ee, 0x4febc94f, 0x2f9c5103,
                0xf3f41acb, 0x36687658
            ]),
            [
                0xa8b22a6d, 0xeef8f09c, 0xcbbec4a8, 0x9aaa6e1a, 0x1a961d1d, 0xf9eb1e96, 0x30fba3be,
                0x39339045, 0x9d982876, 0x5e1b39b4, 0x23ec2a6b, 0x72726f1b, 0x87e8ecdb, 0x126e9b6f,
                0x9e5fe818, 0xca3013b3
            ]
        );
        assert_eq!(
            salsa20([
                0x36687658, 0x4febc94f, 0x2f9c5103, 0xf3f41acb, 0x88eabbbf, 0x730d9fd3, 0xb752374c,
                0x25de7503, 0xcfb31056, 0x30b3ed31, 0xdbb26a01, 0x30a6c7af, 0x24cc37ee, 0x3f20f01f,
                0xa15d530f, 0x71309374
            ]),
            [
                0xca3013b3, 0x87e8ecdb, 0x126e9b6f, 0x9e5fe818, 0x9aaa6e1a, 0xa8b22a6d, 0xeef8f09c,
                0xcbbec4a8, 0x39339045, 0x1a961d1d, 0xf9eb1e96, 0x30fba3be, 0x72726f1b, 0x9d982876,
                0x5e1b39b4, 0x23ec2a6b
            ]
        );
    }
    #[test]
    fn salsa20_stress() {
        let input: [u32; 16] = [
            0x92537c06, 0x3209bf26, 0xde2fa104, 0xb9dfb67a, 0xd8001b4b, 0x59077a10, 0x936568a2,
            0x5f3615d5, 0xb08bfde1, 0x74178469, 0xcfb0294c, 0x6c9d22dd, 0x34635e5e, 0xdc5b755a,
            0x8fefbe92, 0xba82b0c4,
        ];
        let expected_output: [u32; 16] = [
            0xc7261208, 0x43d74c77, 0xa2907fad, 0xd9b0d467, 0x21e913c0, 0xa09ac59f, 0x41dbf380,
            0xe18788ab, 0x56440b7b, 0x9b1452ed, 0x5309bd85, 0x4ec274a7, 0xb9c37f7a, 0x5abcccb9,
            0xf8b709f5, 0x68f555e2,
        ];
        assert_eq!(
            (0..1_000_000).fold(input, |out, _| salsa20(out)),
            expected_output
        );
    }
}
