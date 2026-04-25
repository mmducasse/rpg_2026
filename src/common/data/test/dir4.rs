#[cfg(test)]
#[test]
fn convert_i32() {
    use crate::common::data::dir4::Dir4;

    assert_eq!(Dir4::N as i32, 0);
    assert_eq!(Dir4::W as i32, 3);

    assert_eq!(Dir4::E, Dir4::from(-3));
    assert_eq!(Dir4::N, Dir4::from(0));
    assert_eq!(Dir4::W, Dir4::from(3));
    assert_eq!(Dir4::N, Dir4::from(8));
}

#[test]
fn opposite() {
    use crate::common::data::dir4::Dir4;

    assert_eq!(Dir4::N, Dir4::S.opposite());
    assert_eq!(Dir4::S, Dir4::N.opposite());
    assert_eq!(Dir4::W, Dir4::E.opposite());
    assert_eq!(Dir4::E, Dir4::W.opposite());
}
