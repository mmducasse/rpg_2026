#[cfg(test)]
#[test]
fn can_check_if_contains() {
    use crate::common::num::range::Range;

    let start = 2.5;
    let end = 10.0;
    let r = Range::new(start, end);
    assert!(r.contains(1.0) == false);
    assert!(r.contains(2.5));
    assert!(r.contains(5.0));
    assert!(r.contains(10.0));
    assert!(r.contains(10.01) == false);
    assert!(r.contains(f32::NAN) == false);
    assert!(r.contains(f32::INFINITY) == false);
    assert!(r.contains(f32::NEG_INFINITY) == false);
}

#[test]
fn can_get_delta() {
    use crate::common::num::range::Range;

    let start = 1;
    let end = 24;
    let r = Range::new(start, end);
    assert_eq!(r.delta(), end - start);

    let start = 1.34;
    let end = 24.56;
    let r = Range::new(start, end);
    assert_eq!(r.delta(), end - start);
}

#[test]
fn can_get_abs() {
    use crate::common::num::range::Range;

    let a = 5;
    let b = 100;
    let fwd = Range::new(a, b);
    let back = Range::new(b, a);

    assert_eq!(fwd, back.abs());
    assert_eq!(fwd.abs(), back.abs());
}

#[test]
fn can_lerp() {
    use crate::common::num::range::Range;

    let r = Range::new(5.0, 10.0);

    assert_eq!(r.lerp(0.0), 5.0);
    assert_eq!(r.lerp(0.5), 7.5);
    assert_eq!(r.lerp(1.0), 10.0);
    assert_eq!(r.lerp(1.5), 12.5);
}
