//! DWord — a 36-trit balanced-ternary double word (the (S, Y) pair of the
//! Setun-70 long arithmetic: 18-trit × 18-trit → 36-trit products, long
//! shifts and normalisation).
//!
//! Layout: value = hi · 3^18 + lo, with `lo` the balanced representative
//! (the canonical split). All arithmetic is trit-pure: built from
//! `Trit::add` (full ternary adder) and `Word` shifts; no binary
//! intermediates in the data path.

use crate::trit::Trit;
use crate::word::Word;

/// 3^36.
const B36: i128 = 150_094_635_296_999_121;

/// A 36-trit double word: (senior, junior).
#[derive(Clone, Copy, Debug)]
pub struct DWord {
    pub hi: Word,
    pub lo: Word,
}

impl DWord {
    pub fn new(hi: Word, lo: Word) -> Self {
        DWord { hi, lo }
    }

    pub fn zero() -> Self {
        DWord { hi: Word::zero(), lo: Word::zero() }
    }

    /// The whole 36-trit balanced value (representation read; the value
    /// fits i128 with huge headroom).
    pub fn to_i128(&self) -> i128 {
        let mut v: i128 = 0;
        let mut p: i128 = 1;
        for i in 0..18 {
            v += self.lo.trits[i].to_i8() as i128 * p;
            p *= 3;
        }
        for i in 0..18 {
            v += self.hi.trits[i].to_i8() as i128 * p;
            p *= 3;
        }
        v
    }

    /// Balanced conversion; values outside the range wrap modulo 3^36
    /// (machine arithmetic). For constants and data, not for computed
    /// values.
    pub fn from_i128(v: i128) -> Self {
        let mut trits = [Trit::Z; 36];
        let mut x = v;
        for i in 0..36 {
            let rem = x.rem_euclid(3) as i8;
            let t = if rem == 2 { -1 } else { rem };
            trits[i] = Trit::from_i8(t).unwrap();
            x = (x - t as i128) / 3;
        }
        let mut lo = [Trit::Z; 18];
        let mut hi = [Trit::Z; 18];
        lo.copy_from_slice(&trits[0..18]);
        hi.copy_from_slice(&trits[18..36]);
        DWord { lo: Word { trits: lo }, hi: Word { trits: hi } }
    }

    /// Senior-most trit (weight 3^35).
    pub fn top_trit(&self) -> Trit {
        self.hi.trits[17]
    }

    /// Parity of the whole 36-trit value.
    pub fn is_even(&self) -> bool {
        self.hi.is_even() == self.lo.is_even()
    }

    /// Index of the most significant nonzero trit (−1 if zero):
    /// 35..=18 from the senior word, 17..=0 from the junior one.
    pub fn top_nonzero(&self) -> i8 {
        let h = self.hi.top_nonzero();
        if h >= 0 {
            18 + h
        } else {
            self.lo.top_nonzero()
        }
    }

    pub fn is_zero(&self) -> bool {
        self.hi.trits.iter().all(|&t| t == Trit::Z) && self.lo.trits.iter().all(|&t| t == Trit::Z)
    }

    /// Full 36-trit addition with carry out of the top (discarded by the
    /// caller when mirroring machine wrap-around).
    pub fn add(&self, other: &DWord) -> (DWord, Trit) {
        let mut lo = [Trit::Z; 18];
        let mut hi = [Trit::Z; 18];
        let mut carry = Trit::Z;
        for i in 0..18 {
            let (s, c) = self.lo.trits[i].add(other.lo.trits[i], carry);
            lo[i] = s;
            carry = c;
        }
        for i in 0..18 {
            let (s, c) = self.hi.trits[i].add(other.hi.trits[i], carry);
            hi[i] = s;
            carry = c;
        }
        (
            DWord { lo: Word { trits: lo }, hi: Word { trits: hi } },
            carry,
        )
    }

    pub fn neg(&self) -> DWord {
        let mut lo = [Trit::Z; 18];
        let mut hi = [Trit::Z; 18];
        for i in 0..18 {
            lo[i] = self.lo.trits[i].neg();
            hi[i] = self.hi.trits[i].neg();
        }
        DWord { lo: Word { trits: lo }, hi: Word { trits: hi } }
    }

    pub fn sub(&self, other: &DWord) -> (DWord, Trit) {
        self.add(&other.neg())
    }

    /// Long shift toward the senior end (×3^n), wrapping inside 36 trits.
    /// lo's top n trits carry into hi's bottom n; hi's overflow is dropped.
    pub fn shl(&self, n: usize) -> DWord {
        if n == 0 {
            *self
        } else if n >= 36 {
            DWord::zero()
        } else if n < 18 {
            let mut hi = self.hi.shl(n);
            for i in 0..n {
                hi.trits[i] = self.lo.trits[18 - n + i];
            }
            DWord { hi, lo: self.lo.shl(n) }
        } else {
            // hi shifts out of the 36-trit frame entirely; lo becomes senior.
            DWord { hi: self.lo.shl(n - 18), lo: Word::zero() }
        }
    }

    /// Long shift toward the junior end (÷3^n, floor).
    /// hi's bottom n trits drop into lo's top n; lo's underflow is dropped.
    pub fn shr(&self, n: usize) -> DWord {
        if n == 0 {
            *self
        } else if n >= 36 {
            DWord::zero()
        } else if n < 18 {
            let mut lo = self.lo.shr(n);
            for i in 0..n {
                lo.trits[18 - n + i] = self.hi.trits[i];
            }
            DWord { hi: self.hi.shr(n), lo }
        } else {
            DWord { hi: Word::zero(), lo: self.hi.shr(n - 18) }
        }
    }

    /// Normalisation (XNN): if the senior-most trit is nonzero, divide by
    /// 3 once (i = 1); otherwise multiply by 3 until the senior-most trit
    /// becomes nonzero or the 34-shift bound is reached. Returns the new
    /// double word and the exponent adjustment i (e := e + i).
    pub fn normalize(&self) -> (DWord, i32) {
        let mut x = *self;
        let mut i: i32 = 0;
        if x.top_trit() != Trit::Z {
            x = x.shr(1);
            i = 1;
        } else {
            while x.top_trit() == Trit::Z && i > -34 {
                x = x.shl(1);
                i -= 1;
            }
        }
        (x, i)
    }
}

#[allow(dead_code)]
const _: () = {
    let _ = B36; // documented modulus; arithmetic never routes through it
};
