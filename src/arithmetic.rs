use crate::logic::*;

/// Half Adder
/// * `a` - first input of a half adder
/// * `b` - second input of a half adder
/// * return - [sum, carry] : [Bit; 2]
pub fn half_adder(a: Bit, b: Bit) -> [Bit; 2] {
    [xor(a, b), and(a, b)]
}

/// Full Adder
/// * `a` - first input of a full adder
/// * `b` - second input of a full adder
/// * `c` - third input of a full adder
/// * return - [sum, carry] : [Bit; 3]
pub fn full_adder(a: Bit, b: Bit, c: Bit) -> [Bit; 2] {
    [xor(xor(a, b), c), or(or(and(a, b), and(b, c)), and(c, a))]
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
        let expecteds: [[Bit; 2]; 4] = [
            [false, false],
            [true, false],
            [true, false],
            [false, true],
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
        let expecteds: [[Bit; 2]; 8] = [
            [false, false],
            [true, false],
            [true, false],
            [false, true],
            [true, false],
            [false, true],
            [false, true],
            [true, true],
        ];

            for i in 0..8 {
                assert_eq!(full_adder(ins[i][0], ins[i][1], ins[i][2]), expecteds[i]);
            }
    }
}
