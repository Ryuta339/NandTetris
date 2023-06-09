use crate::logic::*;

/// Half Adder
/// * `a` - first input of a half adder
/// * `b` - second input of a half adder
/// * return - [sum, carry] : (Bit, Bit)
pub fn half_adder(a: Bit, b: Bit) -> (Bit, Bit) {
    ( xor(a, b), and(a, b) )
}

/// Full Adder
/// * `a` - first input of a full adder
/// * `b` - second input of a full adder
/// * `c` - third input of a full adder
/// * return - [sum, carry] : (Bit, Bit)
pub fn full_adder(a: Bit, b: Bit, c: Bit) -> (Bit, Bit) {
    (
        xor(xor(a, b), c), 
        or(or(and(a, b), and(b, c)), and(c, a))
    )
}

/// 16-bit-width adder
/// * `a` - first 16-bit-width input of an adder
/// * `b` - second 16-bit-width input of an adder
/// * return a + b (mod 0x10000)
///
/// Do not detect an overflow.
pub fn add16(a: [Bit; 16], b: [Bit; 16]) -> [Bit; 16] {
    a.iter().zip(b.iter())
        .scan(false, |carry, (x, y)| {
            let tmp: Bit;
            (tmp, *carry) = full_adder(*x, *y, *carry);
            Some(tmp)
        })
        .collect::<Vec<Bit>>()
        .as_slice()
        .try_into()
        .unwrap()
}

/// 16-bit-width increment
/// * `a` - input
/// * return a+1 (mod 0x10000)
///
/// Do not detect an overflow.
pub fn inc16(a: [Bit; 16]) -> [Bit; 16] {
    add16(a, [
        true, false, false, false,
        false, false, false, false,
        false, false, false, false,
        false, false, false, false,
    ])
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_half_adder() {
        let mut ins: [[Bit; 2]; 4] = [[false; 2]; 4];
        for i in 0..4 {
            ins[i][0] = (i & 1) == 1;
            ins[i][1] = (i & 2) == 2;
        }
        let expecteds: [(Bit, Bit); 4] = [
            (false, false),
            (true, false),
            (true, false),
            (false, true),
        ];

        for i in 0..4 {
            assert_eq!(half_adder(ins[i][0], ins[i][1]), expecteds[i]);
        }
    }

    #[test]
    fn test_full_adder() {
        let mut ins: [[Bit; 3]; 8] = [[false; 3]; 8];
        for i in 0..8 {
            ins[i][0] = (i & 1) == 1;
            ins[i][1] = (i & 2) == 2;
            ins[i][2] = (i & 4) == 4;
        }
        let expecteds: [(Bit, Bit); 8] = [
            (false, false),
            (true, false),
            (true, false),
            (false, true),
            (true, false),
            (false, true),
            (false, true),
            (true, true),
        ];

            for i in 0..8 {
                assert_eq!(full_adder(ins[i][0], ins[i][1], ins[i][2]), expecteds[i]);
            }
    }

    #[test]
    fn test_add16() {
        let arr_a: [u16; 5] = [0, 1, 0, 3, 0x8000];
        let arr_b: [u16; 5] = [0, 1, 2, 2, 0x8000];
        for (a, b) in arr_a.iter().zip(arr_b.iter()){
            // オーバーフローが起きた時は切り捨てる
            let ans: u16 = a.wrapping_add(*b);
            let mut bin_a: [Bit; 16] = [false; 16];
            let mut bin_b: [Bit; 16] = [false; 16];
            let mut expected: [Bit; 16] = [false; 16];
            for i in 0..16 {
                let flag: u16 = 1 << i;
                bin_a[i] = (a & flag) == flag;
                bin_b[i] = (b & flag) == flag;
                expected[i] = (ans & flag) == flag;
            }
            assert_eq!(add16(bin_a, bin_b), expected);
        }
    }

    #[test]
    fn test_inc16() {
        let arr: [u16; 5] = [0, 1, 2, 3, 0xFFFF];
        for a in arr.iter() {
            // オーバーフローが起きた時は切り捨てる
            let ans: u16 = a.wrapping_add(1);
            let mut bin_a: [Bit; 16] = [false; 16];
            let mut expected: [Bit; 16] = [false; 16];
            for i in 0..16 {
                let flag: u16 = 1 << i;
                bin_a[i] = (a & flag) == flag;
                expected[i] = (ans & flag) == flag;
            }
            assert_eq!(inc16(bin_a), expected);
        }
    }
}
