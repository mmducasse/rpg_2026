#[cfg(test)]
use crate::common::num::ivec2::{i2, IVec2};

#[test]
fn struct_of_i32() {
    let _a = IVec2 { x: 1, y: 2 };

    let _b = IVec2 { x: -1, y: -2 };
}

#[test]
fn easy_constr() {
    let _a = i2(3, 2);
}

#[test]
fn can_equate() {
    let a = i2(1, 2);
    let b = IVec2 { x: 1, y: 2 };

    assert_eq!(a, b);
}

#[test]
fn can_add() {
    let a = i2(5, 4);
    let b = i2(-1, -2);

    let c = a + b;

    assert_eq!(c, i2(4, 2));
}

#[test]
fn can_sub() {
    let a = i2(5, 7);
    let b = i2(-1, -2);

    let c = a - b;

    assert_eq!(c, i2(6, 9));
}

#[test]
fn can_mul() {
    let a = i2(3, 4);
    let b = i2(3, -5);

    let c = a * b;

    assert_eq!(c, i2(9, -20));
}

#[test]
fn can_scale() {
    let a = i2(3, 4);
    let b = 4;

    let c = a * b;

    assert_eq!(c, i2(12, 16));
}

#[test]
fn can_lerp() {
    use crate::common::num::range::Range;

    let y0 = i2(2, 4);
    let y1 = i2(8, 10);
    let r = Range::new(y0, y1);

    assert_eq!(r.lerp(-0.5), i2(-1, 1));
    assert_eq!(r.lerp(0.0), i2(2, 4));
    assert_eq!(r.lerp(0.5), i2(5, 7));
    assert_eq!(r.lerp(1.0), i2(8, 10));
    assert_eq!(r.lerp(1.5), i2(11, 13));
}

#[test]
fn can_wrap_from_i32() {
    assert_eq!(IVec2::wrap(0, 1), i2(0, 0));
    assert_eq!(IVec2::wrap(0, 5), i2(0, 0));
    assert_eq!(IVec2::wrap(2, 1), i2(0, 2));
    assert_eq!(IVec2::wrap(2, 2), i2(0, 1));
    assert_eq!(IVec2::wrap(2, 5), i2(2, 0));
    assert_eq!(IVec2::wrap(10, 5), i2(0, 2));
}

#[test]
fn can_unwrap_to_i32() {
    assert_eq!(IVec2::unwrap(i2(0, 0), 1), 0);
    assert_eq!(IVec2::unwrap(i2(0, 0), 10), 0);
    assert_eq!(IVec2::unwrap(i2(2, 0), 1), 2);
    assert_eq!(IVec2::unwrap(i2(2, 0), 10), 2);
    assert_eq!(IVec2::unwrap(i2(2, 3), 4), 14);
}
