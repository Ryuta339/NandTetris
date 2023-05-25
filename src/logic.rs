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
}
