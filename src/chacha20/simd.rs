use packed_simd::u32x4;

// Converts column vectors
// 0 4 8 A
// 1 5 9 D
// 2 6 A E
// 3 7 B F
//
// into diagonal vectors
// 0 5 A D
// 1 6 B E
// 2 7 8 F
// 3 4 9 A
#[inline(always)]
fn shift(a: u32x4, b: u32x4, c: u32x4, d: u32x4) -> (u32x4, u32x4, u32x4, u32x4) {
    (
        a,
        b.shuffle1_dyn(u32x4::new(1, 2, 3, 0)),
        c.shuffle1_dyn(u32x4::new(2, 3, 0, 1)),
        d.shuffle1_dyn(u32x4::new(3, 0, 1, 2)),
    )
}
// Converts diagonal vectors
// 0 5 A D
// 1 6 B E
// 2 7 8 F
// 3 4 9 A
//
// into column vectors
// 0 4 8 A
// 1 5 9 D
// 2 6 A E
// 3 7 B F

#[inline(always)]
fn unshift(a: u32x4, b: u32x4, c: u32x4, d: u32x4) -> (u32x4, u32x4, u32x4, u32x4) {
    (
        a,
        b.shuffle1_dyn(u32x4::new(3, 0, 1, 2)),
        c.shuffle1_dyn(u32x4::new(2, 3, 0, 1)),
        d.shuffle1_dyn(u32x4::new(1, 2, 3, 0)),
    )
}

// Converts the following matrix
// 0123
// 4567
// 89AB
// CDEF
//
// into these 4 vectors:
// 0 4 8 7
// 1 5 9 D
// 2 6 A E
// 3 7 B F
#[inline(always)]
pub fn prepare(input: [u32; 16]) -> (u32x4, u32x4, u32x4, u32x4) {
    (
        u32x4::new(input[0], input[1], input[2], input[3]),
        u32x4::new(input[4], input[5], input[6], input[7]),
        u32x4::new(input[8], input[9], input[10], input[11]),
        u32x4::new(input[12], input[13], input[14], input[15]),
    )
}

// Converts the following vectors
// 0 4 8 7
// 1 5 9 D
// 2 6 A E
// 3 7 B F
//
// into this matrix
// 0123
// 4567
// 89AB
// CDEF
#[inline(always)]
pub fn finalize(a: u32x4, b: u32x4, c: u32x4, d: u32x4) -> [u32; 16] {
    [
        a.extract(0),
        a.extract(1),
        a.extract(2),
        a.extract(3),
        b.extract(0),
        b.extract(1),
        b.extract(2),
        b.extract(3),
        c.extract(0),
        c.extract(1),
        c.extract(2),
        c.extract(3),
        d.extract(0),
        d.extract(1),
        d.extract(2),
        d.extract(3),
    ]
}

#[inline(always)]
pub fn round(
    mut a: u32x4,
    mut b: u32x4,
    mut c: u32x4,
    mut d: u32x4,
) -> (u32x4, u32x4, u32x4, u32x4) {
    a += b;
    d = (d ^ a).rotate_left(u32x4::splat(16));
    c += d;
    b = (b ^ c).rotate_left(u32x4::splat(12));
    a += b;
    d = (d ^ a).rotate_left(u32x4::splat(8));
    c += d;
    b = (b ^ c).rotate_left(u32x4::splat(7));;
    (a, b, c, d)
}
#[inline(always)]
pub fn column_round(a: u32x4, b: u32x4, c: u32x4, d: u32x4) -> (u32x4, u32x4, u32x4, u32x4) {
    round(a, b, c, d)
}
#[inline(always)]
pub fn diagonal_round(a: u32x4, b: u32x4, c: u32x4, d: u32x4) -> (u32x4, u32x4, u32x4, u32x4) {
    let (a, b, c, d) = shift(a, b, c, d);
    let (a, b, c, d) = column_round(a, b, c, d);
    unshift(a, b, c, d)
}
#[inline(always)]
pub fn double_round(a: u32x4, b: u32x4, c: u32x4, d: u32x4) -> (u32x4, u32x4, u32x4, u32x4) {
    let (a, b, c, d) = column_round(a, b, c, d);
    diagonal_round(a, b, c, d)
}
#[inline(always)]
pub fn chacha20(input: [u32; 16]) -> [u32; 16] {
    let abcd = prepare(input);
    let mut buf = (0..10).fold(abcd, |(a, b, c, d), _| double_round(a, b, c, d));
    buf.0 += abcd.0;
    buf.1 += abcd.1;
    buf.2 += abcd.2;
    buf.3 += abcd.3;
    finalize(buf.0, buf.1, buf.2, buf.3)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn chacha20_vec() {
        assert_eq!(
            chacha20([
                0x6170_7865,
                0x3320_646e,
                0x7962_2d32,
                0x6b20_6574,
                0x0302_0100,
                0x0706_0504,
                0x0b0a_0908,
                0x0f0e_0d0c,
                0x1312_1110,
                0x1716_1514,
                0x1b1a_1918,
                0x1f1e_1d1c,
                0x0000_0001,
                0x0900_0000,
                0x4a00_0000,
                0x0000_0000
            ]),
            [
                0xe4e7_f110,
                0x1559_3bd1,
                0x1fdd_0f50,
                0xc471_20a3,
                0xc7f4_d1c7,
                0x0368_c033,
                0x9aaa_2204,
                0x4e6c_d4c3,
                0x4664_82d2,
                0x09aa_9f07,
                0x05d7_c214,
                0xa202_8bd9,
                0xd19c_12b5,
                0xb94e_16de,
                0xe883_d0cb,
                0x4e3c_50a2
            ]
        );
    }
}
