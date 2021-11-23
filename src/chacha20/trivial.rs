#[inline(always)]
pub fn quarter_round(mut a: u32, mut b: u32, mut c: u32, mut d: u32) -> (u32, u32, u32, u32) {
    a = a.wrapping_add(b);
    d = (a ^ d).rotate_left(16);
    c = c.wrapping_add(d);
    b = (b ^ c).rotate_left(12);
    a = a.wrapping_add(b);
    d = (a ^ d).rotate_left(8);
    c = c.wrapping_add(d);
    b = (b ^ c).rotate_left(7);
    (a, b, c, d)
}

#[inline(always)]
pub fn column_round(state: [u32; 16]) -> [u32; 16] {
    let (a0, a1, a2, a3) = quarter_round(state[0], state[4], state[8], state[12]);
    let (b0, b1, b2, b3) = quarter_round(state[1], state[5], state[9], state[13]);
    let (c0, c1, c2, c3) = quarter_round(state[2], state[6], state[10], state[14]);
    let (d0, d1, d2, d3) = quarter_round(state[3], state[7], state[11], state[15]);
    [
        a0, b0, c0, d0, a1, b1, c1, d1, a2, b2, c2, d2, a3, b3, c3, d3,
    ]
}

#[inline(always)]
pub fn diagonal_round(state: [u32; 16]) -> [u32; 16] {
    let (a0, a1, a2, a3) = quarter_round(state[0], state[5], state[10], state[15]);
    let (b0, b1, b2, b3) = quarter_round(state[1], state[6], state[11], state[12]);
    let (c0, c1, c2, c3) = quarter_round(state[2], state[7], state[8], state[13]);
    let (d0, d1, d2, d3) = quarter_round(state[3], state[4], state[9], state[14]);
    [
        a0, b0, c0, d0, d1, a1, b1, c1, c2, d2, a2, b2, b3, c3, d3, a3,
    ]
}

#[inline(always)]
pub fn double_round(state: [u32; 16]) -> [u32; 16] {
    diagonal_round(column_round(state))
}

#[inline(always)]
pub fn chacha20(mut input: [u32; 16]) -> [u32; 16] {
    let mut buf = (0..10).fold(input, |state, _| double_round(state));
    for i in 0..16 {
        buf[i] = buf[i].wrapping_add(input[i]);
        input[i] = 0;
    }
    buf
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn quarter_round_vec() {
        assert_eq!(
            quarter_round(0x1111_1111, 0x0102_0304, 0x9b8d_6f43, 0x0123_4567),
            (0xea2a_92f4, 0xcb1c_f8ce, 0x4581_472e, 0x5881_c4bb)
        );
    }
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
