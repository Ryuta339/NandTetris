use crate::logic::*;

/// Half Adder
/// * `a` - first input of a half adder
/// * `b` - second input of a half adder
/// * return - [sum, carry] : [Bit; 2]
pub fn half_adder(a: Bit, b: Bit) -> [Bit; 2] {
    [xor(a, b), and(a, b)]
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
}
