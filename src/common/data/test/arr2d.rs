#[cfg(test)]
#[test]
fn correct_size_and_count() {
    use crate::common::{data::arr2d::Arr2D, num::ivec2::i2};

    let a = Arr2D::new(vec![0; 5], 3);
    assert_eq!(a.size(), i2(3, 2));
    assert_eq!(a.count(), 5);

    let b = Arr2D::new(vec![0; 12], 3);
    assert_eq!(b.size(), i2(3, 4));
    assert_eq!(b.count(), 12);

    let c = Arr2D::new(vec![0; 0], 3);
    assert_eq!(c.size(), i2(3, 1));
    assert_eq!(c.count(), 0);
}

#[test]
fn get() {
    use crate::common::{data::arr2d::Arr2D, num::ivec2::i2};

    let a = Arr2D::new(vec![0, 1, 2, 3, 4], 3);
    assert_eq!(a.get(i2(1, 0)), Some(&1));
    assert_eq!(a.get(i2(2, 1)), None);
    assert_eq!(a.get_i(4), Some(&4));
}
