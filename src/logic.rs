// Bool代数を表す型
pub type Bit = bool;

// NANDゲート
pub fn nand(a: Bit, b: Bit) -> Bit {
    ! (a && b)
}

// NOTゲート
pub fn not(a: Bit) -> Bit {
    nand(a, a)
}

// ANDゲート
pub fn and(a: Bit, b: Bit) -> Bit {
    not(nand(a, b))
}

// ORゲート
pub fn or(a: Bit, b: Bit) -> Bit {
    nand(not(a), not(b))
}

// NORゲート
pub fn nor(a: Bit, b: Bit) -> Bit {
    not(or(a, b))
}

// XORゲート
pub fn xor(a: Bit, b: Bit) -> Bit {
    and(nand(a, b), or(a, b))
}

// マルチプレクサ
pub fn mux(a: Bit, b: Bit, sel: Bit) -> Bit {
    or(and(not(sel), a), and(sel, b))
}

// デマルチプレクサ
pub fn dmux(b_in: Bit, sel: Bit) -> [Bit; 2] {
    [
        and(not(sel), b_in),
        and(sel, b_in)
    ]
}

// 16ビットNOT
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

// 16ビットAND
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

// 16ビットOR
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

// 16ビットマルチプレクサ
pub fn mux16(a: [Bit; 16], b: [Bit; 16], sel: Bit) -> [Bit; 16] {
    [
        mux(a[0], b[0], sel),
        mux(a[1], b[1], sel),
        mux(a[2], b[2], sel),
        mux(a[3], b[3], sel),
        mux(a[4], b[4], sel),
        mux(a[5], b[5], sel),
        mux(a[6], b[6], sel),
        mux(a[7], b[7], sel),
        mux(a[8], b[8], sel),
        mux(a[9], b[9], sel),
        mux(a[10], b[10], sel),
        mux(a[11], b[11], sel),
        mux(a[12], b[12], sel),
        mux(a[13], b[13], sel),
        mux(a[14], b[14], sel),
        mux(a[15], b[15], sel),
    ]
}

// 8ビットor
pub fn or8way(b_in: [Bit; 8]) -> Bit {
    or(or(or(b_in[0], b_in[1]), or(b_in[2], b_in[3])), or(or(b_in[4], b_in[5]), or(b_in[6], b_in[7])))
}

// 4入力16ビットマルチプレクサ
pub fn mux4way16(a: [Bit; 16], b: [Bit; 16], c: [Bit; 16], d: [Bit; 16], sel: [Bit; 2]) -> [Bit; 16] {
    mux16(mux16(a, b, sel[0]), mux16(c, d, sel[0]), sel[1])
}

// 8入力16ビットマルチプレクサ
pub fn mux8way16(
    a: [Bit; 16],
    b: [Bit; 16],
    c: [Bit; 16],
    d: [Bit; 16],
    e: [Bit; 16],
    f: [Bit; 16],
    g: [Bit; 16],
    h: [Bit; 16],
    sel: [Bit; 3],
) -> [Bit; 16] {
    mux16(mux4way16(a, b, c, d, [sel[0], sel[1]]), mux4way16(e, f, g, h, [sel[0], sel[1]]), sel[2])
}

// 1ビット1入力4出力デマルチプレクサ
pub fn dmux4way(b_in: Bit, sel: [Bit; 2]) -> [Bit; 4] {
    [
        and(and(not(sel[0]), not(sel[1])), b_in),
        and(and(sel[0], not(sel[1])), b_in),
        and(and(not(sel[0]), sel[1]), b_in),
        and(and(sel[0], sel[1]), b_in),
    ]
}

// 1ビット1入力8出力デマルチプレクサ
pub fn dmux8way(b_in: Bit, sel: [Bit; 3]) -> [Bit; 8] {
    [
        and(and(and(not(sel[0]), not(sel[1])), not(sel[2])), b_in),
        and(and(and(sel[0], not(sel[1])), not(sel[2])), b_in),
        and(and(and(not(sel[0]), sel[1]), not(sel[2])), b_in),
        and(and(and(sel[0], sel[1]), not(sel[2])), b_in),
        and(and(and(not(sel[0]), not(sel[1])), sel[2]), b_in),
        and(and(and(sel[0], not(sel[1])), sel[2]), b_in),
        and(and(and(not(sel[0]), sel[1]), sel[2]), b_in),
        and(and(and(sel[0], sel[1]), sel[2]), b_in),
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

    #[test]
    fn test_mux16() {
        let a: [Bit; 16] = [false; 16];
        let b: [Bit; 16] = [true; 16];
        for sel in BOOLS.iter() {
            let out = mux16(a, b, *sel);
            for i in 0..16 {
                if *sel {
                    assert_eq!(out[i], b[i]);
                } else {
                    assert_eq!(out[i], a[i]);
                }
            }
        }
    }

    #[test]
    fn test_or8way() {
        let a: [Bit; 8] = [false; 8];
        assert_eq!(or8way(a), false);
        let mut b: [Bit; 8] = [false; 8];
        for i in 0..8 {
            b[i] = true;
            assert_eq!(or8way(b), true);
            b[i] = false;
        }
    }

    #[test]
    fn test_mux4way16() {
        let a: [Bit; 16] = [
            false, false, false, false,
            false, false, false, false,
            false, false, false, false,
            false, false, false, false,
        ];
        let b: [Bit; 16] = [
            false, false, false, false,
            true, true, true, true,
            false, false, false, false,
            true, true, true, true,
        ];
        let c: [Bit; 16] = [
            true, true, true, true,
            false, false, false, false,
            true, true, true, true,
            false, false, false, false,
        ];
        let d: [Bit; 16] = [
            true, true, true, true,
            true, true, true, true,
            true, true, true, true,
            true, true, true, true,
        ];
        assert_eq!(mux4way16(a, b, c, d, [false, false]), a);
        assert_eq!(mux4way16(a, b, c, d, [true, false]), b);
        assert_eq!(mux4way16(a, b, c, d, [false, true]), c);
        assert_eq!(mux4way16(a, b, c, d, [true, true]), d);
    }

    #[test]
    fn test_mux8way16() {
        let arrs: [[Bit; 16]; 8] = [
            [
                false, false, false, false,
                false, false, false, false,
                false, false, false, false,
                false, false, false, false,
            ],
            [
                true, false, false, false,
                false, false, false, false,
                false, false, false, false,
                false, false, false, false,
            ],
            [
                false, true, false, false,
                false, false, false, false,
                false, false, false, false,
                false, false, false, false,
            ],
            [
                false, false, true, false,
                false, false, false, false,
                false, false, false, false,
                false, false, false, false,
            ],
            [
                false, false, false, true,
                false, false, false, false,
                false, false, false, false,
                false, false, false, false,
            ],
            [
                false, false, false, false,
                false, false, false, false,
                true, false, false, false,
                false, false, false, false,
            ],
            [
                false, false, false, false,
                false, false, false, false,
                false, true, false, false,
                false, false, false, false,
            ],
            [
                false, false, false, false,
                false, false, false, false,
                false, false, true, false,
                false, false, false, false,
            ],
        ];
        let sels: [[Bit; 3]; 8] = [
            [false, false, false],
            [true, false, false],
            [false, true, false],
            [true, true, false],
            [false, false, true],
            [true, false, true],
            [false, true, true],
            [true, true, true],
        ];
        for i in 0..8 {
            assert_eq!(mux8way16(arrs[0], arrs[1], arrs[2], arrs[3], arrs[4], arrs[5], arrs[6], arrs[7], sels[i]), arrs[i]);
        }
    }

    #[test]
    fn test_dmux4way() {
        let expecteds: [[Bit; 4]; 4] = [
            [true, false, false, false],
            [false, true, false, false],
            [false, false, true, false],
            [false, false, false, true],
        ];
        let sels: [[Bit; 2]; 4] = [
            [false, false],
            [true, false],
            [false, true],
            [true, true],
        ];
        for i in 0..4 {
            assert_eq!(dmux4way(false, sels[i]), [false, false, false, false]);
            assert_eq!(dmux4way(true, sels[i]), expecteds[i]);
        }
    }

    #[test]
    fn test_dmux8way() {
        let mut expecteds: [[Bit; 8]; 8] = [[false; 8]; 8];
        for i in 0..8 {
            expecteds[i][i] = true;
        }
        let mut sels: [[Bit; 3]; 8] = [[false; 3]; 8];
        for i in 0..8 {
            sels[i][0] = (i & 1) == 1;
            sels[i][1] = (i & 2) == 2;
            sels[i][2] = (i & 4) == 4;
        }
        for i in 0..8 {
            assert_eq!(dmux8way(false, sels[i]), [
                false, false, false, false, false, false, false, false
            ]);
            assert_eq!(dmux8way(true, sels[i]), expecteds[i]);
        }
    }
}
