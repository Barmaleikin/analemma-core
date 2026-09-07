use crate::trit::Trit;
use crate::tryte::{Tryte, TRYTE_SIZE, BASE};

pub const WORD_SIZE: usize = 18;
pub const TRYTES_PER_WORD: usize = 3;

/// An 18-trit balanced-ternary machine word, stored as three trytes.
#[derive(Clone, Copy, Debug)]
pub struct Word {
    pub trits: [Trit; WORD_SIZE],
}

impl Word {
    pub fn zero() -> Self { Word { trits: [Trit::Z; WORD_SIZE] } }

    /// Assembles an 18-trit word from three trytes:
    ///   low  -> trits 0..5,
    ///   mid  -> trits 6..11,
    ///   high -> trits 12..17.
    pub fn from_trytes(low: &Tryte, mid: &Tryte, high: &Tryte) -> Self {
        let mut w = Word::zero();
        for i in 0..TRYTE_SIZE {
            w.trits[i] = low.trits[i];
            w.trits[i + TRYTE_SIZE] = mid.trits[i];
            w.trits[i + 2 * TRYTE_SIZE] = high.trits[i];
        }
        w
    }

    /// Splits the word into (low, mid, high) trytes.
    pub fn to_trytes(&self) -> (Tryte, Tryte, Tryte) {
        let mut low = Tryte::zero();
        let mut mid = Tryte::zero();
        let mut high = Tryte::zero();
        for i in 0..TRYTE_SIZE {
            low.trits[i] = self.trits[i];
            mid.trits[i] = self.trits[i + TRYTE_SIZE];
            high.trits[i] = self.trits[i + 2 * TRYTE_SIZE];
        }
        (low, mid, high)
    }

    /// Balanced conversion; values outside ±387_420_489 wrap modulo 3^18,
    /// mirroring machine arithmetic.
    pub fn from_i64(mut v: i64) -> Self {
        let mut trits = [Trit::Z; WORD_SIZE];
        let mut i = 0;
        while v != 0 && i < WORD_SIZE {
            let rem = v.rem_euclid(BASE) as i8;
            let trit_val = if rem == 2 { -1 } else { rem };
            trits[i] = Trit::from_i8(trit_val).unwrap();
            v = (v - trit_val as i64).div_euclid(BASE);
            i += 1;
        }
        Word { trits }
    }

    pub fn to_i64(&self) -> i64 {
        let mut result: i64 = 0;
        let mut pow: i64 = 1;
        for i in 0..WORD_SIZE {
            result += self.trits[i].to_i8() as i64 * pow;
            pow *= BASE;
        }
        result
    }

    pub fn add(&self, other: &Word) -> (Word, Trit) {
        let mut result = Word::zero();
        let mut carry = Trit::Z;
        for i in 0..WORD_SIZE {
            let (sum, new_carry) = self.trits[i].add(other.trits[i], carry);
            result.trits[i] = sum;
            carry = new_carry;
        }
        (result, carry)
    }

    pub fn sub(&self, other: &Word) -> (Word, Trit) {
        let mut neg = Word::zero();
        for i in 0..WORD_SIZE { neg.trits[i] = other.trits[i].neg(); }
        self.add(&neg)
    }

    /// Full multiplication, double-word result (low, high).
    pub fn mul(&self, other: &Word) -> (Word, Word) {
        let prod = self.to_i64() * other.to_i64();
        let base = BASE.pow(WORD_SIZE as u32);
        (Word::from_i64(prod.rem_euclid(base)), Word::from_i64(prod.div_euclid(base)))
    }

    /// Multiply-accumulate: `other * r * 9 + self`, double-word result.
    pub fn mul_acc(&self, other: &Word, r: &Word) -> (Word, Word) {
        let prod = other.to_i64() * r.to_i64() * 9 + self.to_i64();
        let base = BASE.pow(WORD_SIZE as u32);
        (Word::from_i64(prod.rem_euclid(base)), Word::from_i64(prod.div_euclid(base)))
    }

    pub fn shl(&self, n: usize) -> Word {
        let mut result = Word::zero();
        for i in (n..WORD_SIZE).rev() { result.trits[i] = self.trits[i - n]; }
        result
    }

    pub fn shr(&self, n: usize) -> Word {
        let mut result = Word::zero();
        for i in 0..(WORD_SIZE - n) { result.trits[i] = self.trits[i + n]; }
        result
    }

    pub fn neg(&self) -> Word {
        let mut result = Word::zero();
        for i in 0..WORD_SIZE { result.trits[i] = self.trits[i].neg(); }
        result
    }

    pub fn abs(&self) -> Word { Word::from_i64(self.to_i64().abs()) }

    /// Parity: true if the value is even (number of nonzero trits is
    /// even — 3 ≡ 1 mod 2).
    pub fn is_even(&self) -> bool {
        self.trits.iter().filter(|&&t| t != Trit::Z).count() % 2 == 0
    }

    /// Index of the most significant nonzero trit (−1 if the word is
    /// zero). Intended for sign checks and converters.
    pub fn top_nonzero(&self) -> i8 {
        for i in (0..WORD_SIZE).rev() {
            if self.trits[i] != Trit::Z {
                return i as i8;
            }
        }
        -1
    }

    pub fn slice(&self, from: usize, to: usize) -> Word {
        assert!(from <= to && to < WORD_SIZE);
        let mut result = Word::zero();
        let len = to - from + 1;
        for i in 0..len { result.trits[i] = self.trits[from + i]; }
        result
    }

    pub fn set_slice(&mut self, from: usize, to: usize, src: &Word) {
        assert!(from <= to && to < WORD_SIZE);
        let len = to - from + 1;
        for i in 0..len { self.trits[from + i] = src.trits[i]; }
    }
}

impl Default for Word { fn default() -> Self { Word::zero() } }
