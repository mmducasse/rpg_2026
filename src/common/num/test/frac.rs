#[cfg(test)]
use crate::common::num::frac::Frac;

#[allow(dead_code)]
fn f128(num: i32) -> crate::common::num::frac::Frac<128> {
    crate::common::num::frac::Frac { num }
}

#[test]
fn arithmetic() {
    let f1 = f128(5);
    let f2 = f128(7);

    assert!(f1 != f2); // Eq
    assert!(f1 < f2); // Ord
    assert_eq!(f1 + f2, f128(12)); // Add
    assert_eq!(f1 - f2, f128(-2)); // Sub
}

#[test]
fn convert_f32() {
    let frac = f128(64);
    let f: f32 = frac.into();

    assert_eq!(f, 0.5); // Into<f32>
    assert_eq!(Frac::from(f), frac); // From<f32>
}
