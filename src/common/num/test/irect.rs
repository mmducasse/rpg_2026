#[cfg(test)]
use crate::common::num::{irect::*, ivec2::i2};

#[test]
fn struct_of_ivec2() {
    let _a = IRect {
        pos: i2(1, 2),
        size: i2(4, 4),
    };
}

#[test]
fn easy_constr() {
    let _a = IRect {
        pos: i2(1, 2),
        size: i2(4, 5),
    };

    let _b = ir(i2(1, 2), i2(4, 5));
    let _c = rect(1, 2, 4, 5);
}

#[test]
fn can_eq() {
    let a = IRect {
        pos: i2(1, 2),
        size: i2(4, 5),
    };

    let b = ir(i2(1, 2), i2(4, 5));
    let c = rect(1, 2, 4, 5);

    assert_eq!(a, b);
    assert_eq!(a, c);
}

#[test]
fn has_top_bottom_left_right() {
    let irect = rect(1, 2, 3, 4);

    assert_eq!(irect.top(), 2);
    assert_eq!(irect.right(), 3);
    assert_eq!(irect.bottom(), 5);
    assert_eq!(irect.left(), 1);
}

#[test]
fn contains_point_if_inside_or_on_edge() {
    let irect = rect(0, 0, 4, 5);

    let inside = i2(2, 3);
    let outside_x = i2(2, 9);
    let outside_y = i2(9, 2);

    assert!(irect.contains(inside));
    assert!(false == irect.contains(outside_x));
    assert!(false == irect.contains(outside_y));

    let on_top_edge = i2(2, 0);
    let above_top_edge = i2(2, -1);
    let on_bottom_edge = i2(2, 4);
    let below_bottom_edge = i2(2, 5);

    assert!(irect.contains(on_top_edge));
    assert!(false == irect.contains(above_top_edge));
    assert!(irect.contains(on_bottom_edge));
    assert!(false == irect.contains(below_bottom_edge));

    let on_left_edge = i2(0, 2);
    let adj_left_edge = i2(-1, 2);
    let on_right_edge = i2(3, 2);
    let adj_right_edge = i2(4, 2);

    assert!(irect.contains(on_left_edge));
    assert!(false == irect.contains(adj_left_edge));
    assert!(irect.contains(on_right_edge));
    assert!(false == irect.contains(adj_right_edge));
}

#[test]
fn can_overlap_other_rects() {
    let irect = rect(0, 0, 3, 4);

    let crosses_x = rect(-1, 1, 5, 2);
    assert!(irect.overlaps(crosses_x));

    let crosses_y = rect(1, -1, 2, 6);
    assert!(irect.overlaps(crosses_y));

    let around = rect(-1, -1, 5, 6);
    assert!(irect.overlaps(around));

    let inside = rect(1, 1, 1, 1);
    assert!(irect.overlaps(inside));

    let outside = rect(7, 7, 1, 1);
    assert!(false == irect.overlaps(outside));
}

#[test]
fn can_get_intersection() {
    let a = rect(0, 0, 2, 4);
    let b = rect(1, 1, 2, 4);
    let c = rect(5, 5, 1, 1);

    let y1 = a.intersection(b);
    assert_eq!(y1, Some(rect(1, 1, 1, 3)));

    let y2 = a.intersection(c);
    assert_eq!(y2, None);
}

#[test]
fn can_get_union() {
    let a = rect(0, 0, 2, 4);
    let b = rect(1, 1, 2, 4);
    let c = rect(5, 5, 1, 1);

    let y1 = a.union(b);
    assert_eq!(y1, rect(0, 0, 3, 5));

    let y2 = a.union(c);
    assert_eq!(y2, rect(0, 0, 6, 6));
}

#[test]
fn iterable() {
    let points = vec![i2(0, 0), i2(1, 0), i2(2, 0), i2(0, 1), i2(1, 1), i2(2, 1)];

    let irect = rect(0, 0, 2, 1);

    for x in irect.iter().zip(points.iter()) {
        assert_eq!(x.0, *x.1);
    }
}
