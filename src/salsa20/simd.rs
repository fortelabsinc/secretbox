use packed_simd::u32x4;

#[inline(always)]
fn row_to_col(a: u32x4, b: u32x4, c: u32x4, d: u32x4) -> (u32x4, u32x4, u32x4, u32x4) {
    (
        a,
        d.shuffle1_dyn(u32x4::new(1, 2, 3, 0)),
        c.shuffle1_dyn(u32x4::new(2, 3, 0, 1)),
        b.shuffle1_dyn(u32x4::new(3, 0, 1, 2)),
    )
}

#[inline(always)]
fn col_to_row(a: u32x4, b: u32x4, c: u32x4, d: u32x4) -> (u32x4, u32x4, u32x4, u32x4) {
    (
        a,
        d.shuffle1_dyn(u32x4::new(1, 2, 3, 0)),
        c.shuffle1_dyn(u32x4::new(2, 3, 0, 1)),
        b.shuffle1_dyn(u32x4::new(3, 0, 1, 2)),
    )
}
#[inline(always)]
pub fn prepare(input: [u32; 16]) -> (u32x4, u32x4, u32x4, u32x4) {
    let i0 = u32x4::new(input[0], input[5], input[10], input[15]);
    let i1 = u32x4::new(input[1], input[6], input[11], input[12]);
    let i2 = u32x4::new(input[2], input[7], input[8], input[13]);
    let i3 = u32x4::new(input[3], input[4], input[9], input[14]);
    (i0, i1, i2, i3)
}
#[inline(always)]
fn finalize(o0: u32x4, o1: u32x4, o2: u32x4, o3: u32x4) -> [u32; 16] {
    [
        o0.extract(0),
        o1.extract(0),
        o2.extract(0),
        o3.extract(0),
        o3.extract(1),
        o0.extract(1),
        o1.extract(1),
        o2.extract(1),
        o2.extract(2),
        o3.extract(2),
        o0.extract(2),
        o1.extract(2),
        o1.extract(3),
        o2.extract(3),
        o3.extract(3),
        o0.extract(3),
    ]
}

#[inline(always)]
pub fn round(i0: u32x4, i1: u32x4, i2: u32x4, i3: u32x4) -> (u32x4, u32x4, u32x4, u32x4) {
    let z1 = i1 ^ (i0 + i3).rotate_left(u32x4::splat(7));
    let z2 = i2 ^ (z1 + i0).rotate_left(u32x4::splat(9));
    let z3 = i3 ^ (z2 + z1).rotate_left(u32x4::splat(13));
    let z0 = i0 ^ (z3 + z2).rotate_left(u32x4::splat(18));
    (z0, z1, z2, z3)
}

#[inline(always)]
pub fn row_round(i0: u32x4, i1: u32x4, i2: u32x4, i3: u32x4) -> (u32x4, u32x4, u32x4, u32x4) {
    round(i0, i1, i2, i3)
}

#[inline(always)]
pub fn column_round(i0: u32x4, i1: u32x4, i2: u32x4, i3: u32x4) -> (u32x4, u32x4, u32x4, u32x4) {
    let (i0, i1, i2, i3) = row_to_col(i0, i1, i2, i3);
    let (i0, i1, i2, i3) = round(i0, i1, i2, i3);
    col_to_row(i0, i1, i2, i3)
}

#[inline(always)]
pub fn double_round(i0: u32x4, i1: u32x4, i2: u32x4, i3: u32x4) -> (u32x4, u32x4, u32x4, u32x4) {
    let (i0, i1, i2, i3) = column_round(i0, i1, i2, i3);
    row_round(i0, i1, i2, i3)
}

/// This is the raw salsa20 hash function
pub fn salsa20(input: [u32; 16]) -> [u32; 16] {
    // Initialize the 4 SIMD regs
    let (i0, i1, i2, i3) = prepare(input);
    // Calculate salsa20
    let (o0, o1, o2, o3) = (0..10).fold((i0, i1, i2, i3), |(i0, i1, i2, i3), _| {
        double_round(i0, i1, i2, i3)
    });
    // Convert it back into a usable format
    let buf = finalize(i0 + o0, i1 + o1, i2 + o2, i3 + o3);
    buf
}

pub(crate) fn salsa20_rounds(input: [u32; 16]) -> [u32; 16] {
    // Initialize the 4 SIMD regs
    let (i0, i1, i2, i3) = prepare(input);
    // Calculate salsa20
    let (o0, o1, o2, o3) = (0..10).fold((i0, i1, i2, i3), |(i0, i1, i2, i3), _| {
        double_round(i0, i1, i2, i3)
    });
    // Convert it back into a usable format
    finalize(o0, o1, o2, o3)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_row_round() {
        let (i0, i1, i2, i3) = prepare([1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0]);
        assert_eq!(
            row_round(i0, i1, i2, i3),
            prepare([
                0x08008145, 0x80, 0x10200, 0x20500000, 0x20100001, 0x48044, 0x80, 0x10000, 1,
                0x2000, 0x80040000, 0, 1, 0x200, 0x402000, 0x88000100
            ])
        );
        let (i0, i1, i2, i3) = prepare([
            0x08521bd6, 0x1fe88837, 0xbb2aa576, 0x3aa26365, 0xc54c6a5b, 0x2fc74c2f, 0x6dd39cc3,
            0xda0a64f6, 0x90a2f23d, 0x067f95a6, 0x06b35f61, 0x41e4732e, 0xe859c100, 0xea4d84b7,
            0x0f619bff, 0xbc6e965a,
        ]);
        assert_eq!(
            row_round(i0, i1, i2, i3),
            prepare([
                0xa890d39d, 0x65d71596, 0xe9487daa, 0xc8ca6a86, 0x949d2192, 0x764b7754, 0xe408d9b9,
                0x7a41b4d1, 0x3402e183, 0x3c3af432, 0x50669f96, 0xd89ef0a8, 0x0040ede5, 0xb545fbce,
                0xd257ed4f, 0x1818882d
            ])
        );
    }

    #[test]
    fn test_column_round() {
        let (i0, i1, i2, i3) = prepare([1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0]);
        assert_eq!(
            column_round(i0, i1, i2, i3),
            prepare([0x10090288, 0, 0, 0, 0x101, 0, 0, 0, 0x20401, 0, 0, 0, 0x40a04001, 0, 0, 0])
        );
        let (i0, i1, i2, i3) = prepare([
            0x08521bd6, 0x1fe88837, 0xbb2aa576, 0x3aa26365, 0xc54c6a5b, 0x2fc74c2f, 0x6dd39cc3,
            0xda0a64f6, 0x90a2f23d, 0x067f95a6, 0x06b35f61, 0x41e4732e, 0xe859c100, 0xea4d84b7,
            0x0f619bff, 0xbc6e965a,
        ]);
        assert_eq!(
            column_round(i0, i1, i2, i3),
            prepare([
                0x8c9d190a, 0xce8e4c90, 0x1ef8e9d3, 0x1326a71a, 0x90a20123, 0xead3c4f3, 0x63a091a0,
                0xf0708d69, 0x789b010c, 0xd195a681, 0xeb7d5504, 0xa774135c, 0x481c2027, 0x53a8e4b5,
                0x4c1f89c5, 0x3f78c9c8
            ])
        );
    }
    #[test]
    fn test_double_round() {
        let (i0, i1, i2, i3) = prepare([1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        assert_eq!(
            double_round(i0, i1, i2, i3),
            prepare([
                0x8186a22d, 0x0040a284, 0x82479210, 0x06929051, 0x08000090, 0x02402200, 0x00004000,
                0x00800000, 0x00010200, 0x20400000, 0x08008104, 0x00000000, 0x20500000, 0xa0000040,
                0x0008180a, 0x612a8020
            ])
        );
        let (i0, i1, i2, i3) = prepare([
            0xde501066, 0x6f9eb8f7, 0xe4fbbd9b, 0x454e3f57, 0xb75540d3, 0x43e93a4c, 0x3a6f2aa0,
            0x726d6b36, 0x9243f484, 0x9145d1e8, 0x4fa9d247, 0xdc8dee11, 0x054bf545, 0x254dd653,
            0xd9421b6d, 0x67b276c1,
        ]);
        assert_eq!(
            double_round(i0, i1, i2, i3),
            prepare([
                0xccaaf672, 0x23d960f7, 0x9153e63a, 0xcd9a60d0, 0x50440492, 0xf07cad19, 0xae344aa0,
                0xdf4cfdfc, 0xca531c29, 0x8e7943db, 0xac1680cd, 0xd503ca00, 0xa74b2ad6, 0xbc331c5c,
                0x1dda24c7, 0xee928277
            ])
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
