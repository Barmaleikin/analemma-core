use crate::trit::Trit;

pub const TRYTE_SIZE: usize = 6;
pub const BASE: i64 = 3;
pub const MAX_TRYTE_VALUE: i16 = 364;
pub const MIN_TRYTE_VALUE: i16 = -364;

/// A 6-trit balanced-ternary word (the tryte of the Setun-70 era,
/// but defined here as a plain numeric type).
#[derive(Clone, Copy, Debug)]
pub struct Tryte {
    pub trits: [Trit; TRYTE_SIZE],
}

impl Tryte {
    pub fn zero() -> Self { Tryte { trits: [Trit::Z; TRYTE_SIZE] } }

    /// Balanced conversion; values outside ±364 wrap modulo 3^6,
    /// mirroring machine arithmetic.
    pub fn from_i16(mut v: i16) -> Self {
        let mut trits = [Trit::Z; TRYTE_SIZE];
        let mut i = 0;
        while v != 0 && i < TRYTE_SIZE {
            let rem = v.rem_euclid(BASE as i16) as i8;
            let trit_val = if rem == 2 { -1 } else { rem };
            trits[i] = Trit::from_i8(trit_val).unwrap();
            v = (v - trit_val as i16).div_euclid(BASE as i16);
            i += 1;
        }
        Tryte { trits }
    }

    pub fn to_i16(&self) -> i16 {
        let mut result: i16 = 0;
        let mut pow: i16 = 1;
        for i in 0..TRYTE_SIZE {
            result += self.trits[i].to_i8() as i16 * pow;
            pow *= BASE as i16;
        }
        result
    }

    pub fn add(&self, other: &Tryte) -> (Tryte, Trit) {
        let mut result = Tryte::zero();
        let mut carry = Trit::Z;
        for i in 0..TRYTE_SIZE {
            let (sum, new_carry) = self.trits[i].add(other.trits[i], carry);
            result.trits[i] = sum;
            carry = new_carry;
        }
        (result, carry)
    }

    pub fn sub(&self, other: &Tryte) -> (Tryte, Trit) {
        let mut neg = Tryte::zero();
        for i in 0..TRYTE_SIZE { neg.trits[i] = other.trits[i].neg(); }
        self.add(&neg)
    }

    /// Tritwise multiplication (SMT).
    pub fn smt(&self, other: &Tryte) -> Tryte {
        let mut result = Tryte::zero();
        for i in 0..TRYTE_SIZE { result.trits[i] = self.trits[i].mul(other.trits[i]); }
        result
    }

    /// Tritwise addition without carry, i.e. modulo 3 per trit (SAT).
    pub fn sat(&self, other: &Tryte) -> Tryte {
        let mut result = Tryte::zero();
        for i in 0..TRYTE_SIZE {
            let a = self.trits[i];
            let b = other.trits[i];
            result.trits[i] = if a == Trit::Z { b } else if a != b && b != Trit::Z { Trit::Z } else { a };
        }
        result
    }

    pub fn shl(&self, n: usize) -> Tryte {
        let mut result = Tryte::zero();
        for i in (n..TRYTE_SIZE).rev() { result.trits[i] = self.trits[i - n]; }
        result
    }

    pub fn shr(&self, n: usize) -> Tryte {
        let mut result = Tryte::zero();
        for i in 0..(TRYTE_SIZE - n) { result.trits[i] = self.trits[i + n]; }
        result
    }

    pub fn abs(&self) -> Tryte { Tryte::from_i16(self.to_i16().abs()) }

    /// Parity: true if the value is even. Trit-pure: 3 ≡ 1 (mod 2), so
    /// the value mod 2 equals the number of nonzero trits mod 2.
    pub fn is_even(&self) -> bool {
        self.trits.iter().filter(|&&t| t != Trit::Z).count() % 2 == 0
    }

    /// Index of the most significant nonzero trit (−1 if the tryte is
    /// zero). Intended for sign checks and converters.
    pub fn top_nonzero(&self) -> i8 {
        for i in (0..TRYTE_SIZE).rev() {
            if self.trits[i] != Trit::Z {
                return i as i8;
            }
        }
        -1
    }

    pub fn slice(&self, from: usize, to: usize) -> Tryte {
        assert!(from <= to && to < TRYTE_SIZE);
        let mut result = Tryte::zero();
        let len = to - from + 1;
        for i in 0..len { result.trits[i] = self.trits[from + i]; }
        result
    }
}

impl Default for Tryte { fn default() -> Self { Tryte::zero() } }
