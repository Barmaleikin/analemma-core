use crate::trit::Trit;
use crate::tryte::{Tryte, TRYTE_SIZE, MAX_TRYTE_VALUE, MIN_TRYTE_VALUE};
use crate::word::Word;
use crate::ternary_array::TernaryArray;
use crate::table;

#[test]
fn test_trit_operations() {
    let n = Trit::N; let z = Trit::Z; let p = Trit::P;
    assert_eq!(n.neg(), p); assert_eq!(z.neg(), z); assert_eq!(p.neg(), n);
    assert_eq!(n.mul(p), n); assert_eq!(p.mul(p), p); assert_eq!(z.mul(p), z);
    let (sum, carry) = p.add(p, z);
    assert_eq!(sum, Trit::N); assert_eq!(carry, Trit::P);
}

#[test]
fn test_tryte_conversion() {
    let t = Tryte::from_i16(13); assert_eq!(t.to_i16(), 13);
    let t_neg = Tryte::from_i16(-13); assert_eq!(t_neg.to_i16(), -13);
    let t_zero = Tryte::from_i16(0); assert_eq!(t_zero.to_i16(), 0);
}

#[test]
fn test_tryte_negative_numbers() {
    for v in [MIN_TRYTE_VALUE, -100, -13, -8, -1, 0, 1, 8, 13, 100, MAX_TRYTE_VALUE].iter() {
        let t = Tryte::from_i16(*v);
        assert_eq!(t.to_i16(), *v, "Failed for value {}", v);
    }
}

#[test]
fn test_tryte_arithmetic() {
    let a = Tryte::from_i16(10); let b = Tryte::from_i16(5);
    let (sum, _) = a.add(&b); assert_eq!(sum.to_i16(), 15);
    let (diff, _) = a.sub(&b); assert_eq!(diff.to_i16(), 5);
}

#[test]
fn test_word_arithmetic() {
    let a = Word::from_i64(100); let b = Word::from_i64(50);
    let (sum, _) = a.add(&b); assert_eq!(sum.to_i64(), 150);
    let (diff, _) = a.sub(&b); assert_eq!(diff.to_i64(), 50);
}

#[test]
fn test_word_tryte_roundtrip() {
    let low = Tryte::from_i16(100);
    let mid = Tryte::from_i16(-50);
    let high = Tryte::from_i16(200);
    let word = Word::from_trytes(&low, &mid, &high);
    let (l, m, h) = word.to_trytes();
    assert_eq!(l.to_i16(), 100);
    assert_eq!(m.to_i16(), -50);
    assert_eq!(h.to_i16(), 200);
}

#[test]
fn test_word_negative_numbers() {
    for v in [-100000, -1000, -100, -8, -1, 0, 1, 8, 100, 1000, 100000].iter() {
        let w = Word::from_i64(*v);
        assert_eq!(w.to_i64(), *v, "Failed for value {}", v);
    }
}

#[test]
fn test_negative_indices() {
    let mut arr = TernaryArray::new(-5, 5);
    arr.set(-3, Tryte::from_i16(42));
    assert_eq!(arr.get(-3).to_i16(), 42);
    let mut arr2d = crate::ternary_array::TernaryArray2D::new(-1, 1, -2, 2);
    arr2d.set(-1, -2, Tryte::from_i16(99));
    assert_eq!(arr2d.get(-1, -2).to_i16(), 99);
}

#[test]
fn test_tryte_slice() {
    let t = Tryte::from_i16(42);
    let sliced = t.slice(0, 2);
    assert_eq!(sliced.trits.len(), TRYTE_SIZE);
}

#[test]
fn test_word_slice() {
    let w = Word::from_i64(1000);
    let sliced = w.slice(0, 5);
    assert_eq!(sliced.trits.len(), 18);
}

#[test]
fn test_word_mul() {
    let a = Word::from_i64(10); let b = Word::from_i64(20);
    let (low, high) = a.mul(&b);
    let result = low.to_i64() + high.to_i64() * 3_i64.pow(18);
    assert_eq!(result, 200);
}

// ============================================================================
// Numeric substitution table (architecture-neutral)
// ============================================================================

#[test]
fn test_table_constants() {
    assert_eq!(table::TABLE_SIZE, 3usize.pow(table::TRITS as u32));
    assert_eq!(table::OFFSET as i64, (table::TABLE_SIZE as i64 - 1) / 2);
}

#[test]
fn test_table_roundtrip() {
    let probes = [-table::OFFSET, -159, -13, -8, -1, 0, 1, 8, 13, 159, table::OFFSET];
    for v in probes.iter() {
        let e = table::entry_by_value(*v).expect("in range");
        assert_eq!(e.value, *v);
        assert_eq!(e.signed_binary, *v);
        assert_eq!(e.abs_binary, v.unsigned_abs());
        let e2 = table::entry_by_trits(&e.trits).expect("trits decode");
        assert_eq!(e2.value, *v);
        // trits -> value identity
        let mut check: i32 = 0;
        let mut pow: i32 = 1;
        for t in e.trits.iter() {
            check += t.to_i8() as i32 * pow;
            pow *= 3;
        }
        assert_eq!(check, *v);
    }
    assert!(table::entry_by_value(table::OFFSET + 1).is_none());
    assert!(table::entry_by_value(-table::OFFSET - 1).is_none());
}

#[cfg(analemma_trits = "6")]
#[test]
fn test_table_nonary_doc_example() {
    let e = table::entry_by_value(-159).expect("-159 in range");
    let as_i8: Vec<i8> = e.trits.iter().map(|t| t.to_i8()).collect();
    assert_eq!(as_i8, vec![0, 1, 0, 0, 1, -1]); // "ZPZZPN", least significant first
    assert_eq!(e.nonary_digits, [-2, 0, 3]);   // X, 0, 3
}

#[test]
fn test_table_nonary_bounds() {
    // Nonary digits are balanced: each in -4..=4, count = ceil(TRITS / 2).
    let expect_len = (table::TRITS + 1) / 2;
    for v in [-table::OFFSET, -100, -1, 0, 1, 100, table::OFFSET].iter() {
        let e = table::entry_by_value(*v).unwrap();
        assert_eq!(e.nonary_digits.len(), expect_len);
        for &d in e.nonary_digits.iter() {
            assert!((-4..=4).contains(&d));
        }
    }
}

#[cfg(analemma_trits = "6")]
#[test]
fn test_table_strings_debug() {
    let e = table::entry_by_value(-159).expect("-159 in range");
    assert_eq!(e.str_trits, "ZPZZPN");                              // LSB first
    assert_eq!(e.str_nonary, "Y03");                                // MSB first
    assert_eq!(e.str_abs_binary, "10011111");                       // 159
    assert_eq!(e.str_signed_binary, "11111111111111111111111101100001"); // -159, 32-bit
    assert_eq!(e.str_abs_decimal, "159");
    assert_eq!(e.str_signed_decimal, "-159");
    assert_eq!(e.str_abs_hex, "9F");                                // 159
    assert_eq!(e.str_signed_hex, "FFFFFF61");                       // -159, 32-bit
}

#[cfg(not(analemma_trits = "18"))]
#[test]
fn test_static_table_covers_all() {
    assert!(table::HAS_STATIC_TABLE);
    assert_eq!(table::TABLE.len(), table::TABLE_SIZE);
    assert_eq!(table::TABLE[0].value, -table::OFFSET);
    assert_eq!(table::TABLE[table::TABLE_SIZE - 1].value, table::OFFSET);
}

#[cfg(analemma_trits = "18")]
#[test]
fn test_composite18_extremes() {
    assert!(!table::HAS_STATIC_TABLE);
    let max = table::entry_by_value(table::OFFSET).unwrap();
    assert!(max.trits.iter().all(|&t| t == Trit::P));
    assert_eq!(max.trytes.0.value, 364);
    assert_eq!(max.trytes.1.value, 364);
    assert_eq!(max.trytes.2.value, 364);
    let min = table::entry_by_value(-table::OFFSET).unwrap();
    assert!(min.trits.iter().all(|&t| t == Trit::N));
    // composition is exact: value == hi*3^12 + mid*3^6 + lo
    let e = table::entry_by_value(123_456).unwrap();
    let recomposed = e.trytes.2.value as i64 * 3_i64.pow(12)
        + e.trytes.1.value as i64 * 3_i64.pow(6)
        + e.trytes.0.value as i64;
    assert_eq!(recomposed, 123_456);
    // Display composes strings on demand; -159 is "NPZZPZ" MSB-first,
    // and the word-level nonary is the per-tryte nonary concatenation.
    let d = table::entry_by_value(-159).unwrap();
    let s = format!("{}", d);
    assert!(s.contains("value=-159"), "{}", s);
    assert!(s.contains("NPZZPZ"), "{}", s); // 12 leading zero trits + tryte -159
    assert!(s.contains("nonary=000000Y03"), "{}", s);
    assert!(s.contains("hex=FFFFFF61"), "{}", s);
}


// ============================================================================
// DWord: 36-trit double word
// ============================================================================

use crate::dword::DWord;

fn bal36(v: i128) -> i128 {
    let r = v.rem_euclid(3i128.pow(36));
    if r > (3i128.pow(36) - 1) / 2 { r - 3i128.pow(36) } else { r }
}

#[test]
fn test_dword_roundtrip() {
    for v in [0i128, 1, -1, 54, -54, 3i128.pow(18), -(3i128.pow(18)),
              (3i128.pow(36) - 1) / 2, -((3i128.pow(36) - 1) / 2)] {
        assert_eq!(DWord::from_i128(v).to_i128(), v, "value {}", v);
    }
}

#[test]
fn test_dword_add_sub_vs_oracle() {
    let samples = [0i128, 1, -1, 5, -13, 3i128.pow(17), 3i128.pow(18) - 1,
                   123_456_789, -987_654_321, (3i128.pow(36) - 1) / 2];
    for &a in &samples {
        for &b in &samples {
            let (s, _) = DWord::from_i128(a).add(&DWord::from_i128(b));
            assert_eq!(s.to_i128(), bal36(a + b), "{} + {}", a, b);
            let (d, _) = DWord::from_i128(a).sub(&DWord::from_i128(b));
            assert_eq!(d.to_i128(), bal36(a - b), "{} - {}", a, b);
        }
    }
}

/// Shift-right oracle: dropping the n junior-most balanced trits
/// (shift-register semantics — (v − low_residue)/3^n, exact division).
fn shr_oracle(v: i128, n: u32) -> i128 {
    if n >= 36 { return 0; }
    let mut r: i128 = 0;
    let mut p: i128 = 1;
    let mut x = v;
    for _ in 0..n {
        let rem = x.rem_euclid(3) as i128;
        let t = if rem == 2 { -1 } else { rem };
        r += t * p;
        p *= 3;
        x = (x - t) / 3;
    }
    (v - r) / 3i128.pow(n)
}

#[test]
fn test_dword_shifts_vs_oracle() {
    let x = DWord::from_i128(123_456i128);
    for n in [0usize, 1, 5, 17, 18, 19, 35, 36, 40] {
        assert_eq!(x.shl(n).to_i128(), bal36(123_456 * 3i128.pow(n as u32)), "shl {}", n);
        assert_eq!(x.shr(n).to_i128(), shr_oracle(123_456, n as u32), "shr {}", n);
    }
    let neg = DWord::from_i128(-123_456);
    // -123456 has a zero junior trit: drop == floor here
    assert_eq!(neg.shr(1).to_i128(), -123_456 / 3);
    assert_eq!(neg.shl(1).to_i128(), bal36(-123_456 * 3));
    // -13 = [N,N,N,...]: dropping the -1 trit rounds toward the senior end
    assert_eq!(DWord::from_i128(-13).shr(1).to_i128(), (-13 + 1) / 3);
}

#[test]
fn test_dword_normalize() {
    // value 5: senior end empty -> shift left until the top trit is set
    let (x, i) = DWord::from_i128(5).normalize();
    assert_eq!(i, -33);
    assert_ne!(x.top_trit(), crate::trit::Trit::Z);
    assert_eq!(x.to_i128(), bal36(5 * 3i128.pow(33)));
    // already-normalised: top set -> one right shift (trit-drop semantics), i = 1
    let v = 3i128.pow(35) + 5;
    let (x, i) = DWord::from_i128(v).normalize();
    assert_eq!(i, 1);
    assert_eq!(x.to_i128(), shr_oracle(v, 1));
}

#[test]
fn test_dword_pair_layout() {
    // (S, Y) pairing: hi = S, lo = Y, value = S*3^18 + Y
    let d = DWord::new(Word::from_i64(7), Word::from_i64(-3));
    assert_eq!(d.to_i128(), 7 * 3i128.pow(18) - 3);
}


// ============================================================================
// top_nonzero: index of the most significant nonzero trit (−1 if zero)
// ============================================================================

#[test]
fn test_top_nonzero() {
    use crate::dword::DWord;
    use crate::tryte::Tryte;
    use crate::word::Word;

    assert_eq!(Word::zero().top_nonzero(), -1);
    assert_eq!(Tryte::zero().top_nonzero(), -1);
    assert_eq!(DWord::zero().top_nonzero(), -1);

    assert_eq!(Word::from_i64(1).top_nonzero(), 0);
    assert_eq!(Word::from_i64(-1).top_nonzero(), 0);
    assert_eq!(Word::from_i64(5 * 3i64.pow(12)).top_nonzero(), 14); // left-aligned
    assert_eq!(Tryte::from_i16(-13).top_nonzero(), 2);             // [N,N,N,Z,Z,Z]
    assert_eq!(Tryte::from_i16(-159).top_nonzero(), 5);
    assert_eq!(DWord::from_i128(3i128.pow(35) + 5).top_nonzero(), 35);
    assert_eq!(DWord::from_i128(7).top_nonzero(), 2); // 7 = [P,N,P,0..]

    // sign via top_nonzero: the sign trit of a balanced number
    let sign_of = |w: &Word| {
        let i = w.top_nonzero();
        if i < 0 { crate::trit::Trit::Z } else { w.trits[i as usize] }
    };
    assert_eq!(sign_of(&Word::from_i64(5 * 3i64.pow(12))), crate::trit::Trit::P);
    assert_eq!(sign_of(&Word::from_i64(-5 * 3i64.pow(12))), crate::trit::Trit::N);
    assert_eq!(sign_of(&Word::zero()), crate::trit::Trit::Z);

    // the table carries it too
    let e = table::entry_by_value(0).unwrap();
    assert_eq!(e.top_nonzero, -1);
    let e = table::entry_by_value(-159).unwrap();
    assert_eq!(e.top_nonzero, 5);
    assert_eq!(e.trits[e.top_nonzero as usize], crate::trit::Trit::N);
}


#[test]
fn test_is_even() {
    use crate::dword::DWord;
    use crate::tryte::Tryte;
    use crate::word::Word;

    assert!(Tryte::zero().is_even());
    assert!(!Tryte::from_i16(-159).is_even()); // 3 nonzero trits -> odd
    assert!(!Tryte::from_i16(-13).is_even());  // [N,N,N]: 3 nonzero -> odd
    assert!(!Word::from_i64(5 * 3i64.pow(12)).is_even()); // 3 nonzero -> odd
    assert!(Word::from_i64(4 * 3i64.pow(12)).is_even());  // 4=[P,P]: 2 nonzero
    assert!(DWord::from_i128(3i128.pow(35) + 5).is_even()); // 1+3 = 4 nonzero
    assert!(DWord::zero().is_even());

    // the table carries it too
    assert!(table::entry_by_value(0).unwrap().is_even);
    assert!(!table::entry_by_value(-159).unwrap().is_even);
    assert!(table::entry_by_value(-160).unwrap().is_even);
}

