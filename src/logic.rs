type Bit = bool;

pub fn nand(a: Bit, b: Bit) -> Bit {
    ! (a && b)
}

pub fn not(a: Bit) -> Bit {
    nand(a, a)
}

pub fn and(a: Bit, b: Bit) -> Bit {
    not(nand(a, b))
}

pub fn or(a: Bit, b: Bit) -> Bit {
    nand(not(a), not(b))
}

pub fn nor(a: Bit, b: Bit) -> Bit {
    not(or(a, b))
}

pub fn xor(a: Bit, b: Bit) -> Bit {
    and(nand(a, b), or(a, b))
}

pub fn mux(a: Bit, b: Bit, sel: Bit) -> Bit {
    or(and(not(sel), a), and(sel, b))
}

pub fn dmux(b_in: Bit, sel: Bit) -> [Bit; 2] {
    [
        and(not(sel), b_in),
        and(sel, b_in)
    ]
}

pub fn not16(m_in: [Bit; 16]) -> [Bit; 16] {
    [
        not(m_in[0]),
        not(m_in[1]),
        not(m_in[2]),
        not(m_in[3]),
        not(m_in[4]),
        not(m_in[5]),
        not(m_in[6]),
        not(m_in[7]),
        not(m_in[8]),
        not(m_in[9]),
        not(m_in[10]),
        not(m_in[11]),
        not(m_in[12]),
        not(m_in[13]),
        not(m_in[14]),
        not(m_in[15])
    ]
}

pub fn and16(a: [Bit; 16], b: [Bit; 16]) -> [Bit; 16] {
    [
        and(a[0], b[0]),
        and(a[1], b[1]),
        and(a[2], b[2]),
        and(a[3], b[3]),
        and(a[4], b[4]),
        and(a[5], b[5]),
        and(a[6], b[6]),
        and(a[7], b[7]),
        and(a[8], b[8]),
        and(a[9], b[9]),
        and(a[10], b[10]),
        and(a[11], b[11]),
        and(a[12], b[12]),
        and(a[13], b[13]),
        and(a[14], b[14]),
        and(a[15], b[15])
    ]
}

pub fn or16(a: [Bit; 16], b: [Bit; 16]) -> [Bit; 16] {
    [
        or(a[0], b[0]),
        or(a[1], b[1]),
        or(a[2], b[2]),
        or(a[3], b[3]),
        or(a[4], b[4]),
        or(a[5], b[5]),
        or(a[6], b[6]),
        or(a[7], b[7]),
        or(a[8], b[8]),
        or(a[9], b[9]),
        or(a[10], b[10]),
        or(a[11], b[11]),
        or(a[12], b[12]),
        or(a[13], b[13]),
        or(a[14], b[14]),
        or(a[15], b[15])
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    const BOOLS: [Bit; 2] = [false, true];

    #[test]
    fn test_nand() {
        assert_eq!(nand(false, false), true);
        assert_eq!(nand(false, true), true);
        assert_eq!(nand(true, false), true);
        assert_eq!(nand(true, true), false);
    }

    #[test]
    fn test_not() {
        for b in BOOLS.iter() {
            assert_eq!(not(*b), !*b);
        }
    }

    #[test]
    fn test_and() {
        for b1 in BOOLS.iter() {
            for b2 in BOOLS.iter() {
                assert_eq!(and(*b1, *b2), *b1 && *b2);
            }
        }
    }

    #[test]
    fn test_or() {
        for b1 in BOOLS.iter() {
            for b2 in BOOLS.iter() {
                assert_eq!(or(*b1, *b2), *b1 || *b2);
            }
        }
    }

    #[test]
    fn test_nor() {
        for b1 in BOOLS.iter() {
            for b2 in BOOLS.iter() {
                assert_eq!(nor(*b1, *b2), !(*b1 || *b2));
            }
        }
    }

    #[test]
    fn test_xor() {
        assert_eq!(xor(false, false), false);
        assert_eq!(xor(false, true), true);
        assert_eq!(xor(true, false), true);
        assert_eq!(xor(true, true), false);
    }

    #[test]
    fn test_mux() {
        for a in BOOLS.iter() {
            for b in BOOLS.iter() {
                for sel in BOOLS.iter() {
                    if *sel {
                        assert_eq!(mux(*a, *b, *sel), *b);
                    } else {
                        assert_eq!(mux(*a, *b, *sel), *a);
                    }
                }
            }
        }
    }

    #[test]
    fn test_dmux() {
        for b_in in BOOLS.iter() {
            for sel in BOOLS.iter() {
                let out = dmux(*b_in, *sel);
                if *sel {
                    assert_eq!(out[0], false);
                    assert_eq!(out[1], *b_in);
                } else {
                    assert_eq!(out[0], *b_in);
                    assert_eq!(out[1], false);
                }
            }
        }
    }

    #[test]
    fn test_not16() {
       let m_in: [Bit; 16] = [
           false, false, false, false,
           false, false, false, false,
           false, false, false, false,
           false, false, false, false
       ];
       assert_eq!(not16(m_in), [
           true, true, true, true,
           true, true, true, true,
           true, true, true, true,
           true, true, true, true
       ])
    }

    #[test]
    fn test_and16() {
        let a: [Bit; 16] = [
            false, false, false, false,
            false, false, true, true,
            true, true, false, false,
            true, true, true, true
        ];
        let b: [Bit; 16] = [
            false, false, false, false,
            false, true, false, true,
            true, false, true, false,
            true, true, true, true
        ];
        assert_eq!(and16(a, b), [
            false, false, false, false,
            false, false, false, true,
            true, false, false, false,
            true, true, true, true
        ])
    }

    #[test]
    fn test_or16() {
        let a: [Bit; 16] = [
            false, false, false, false,
            false, false, true, true,
            true, true, false, false,
            true, true, true, true
        ];
        let b: [Bit; 16] = [
            false, false, false, false,
            false, true, false, true,
            true, false, true, false,
            true, true, true, true
        ];
        assert_eq!(or16(a, b), [
            false, false, false, false,
            false, true, true, true,
            true, true, true, false,
            true, true, true, true
        ])
    }
}
