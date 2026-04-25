#[cfg(test)]
#[test]
fn add_and_sub_assign() {
    use crate::common::num::limit::Limit;

    let mut lim_i32: Limit<i32> = Limit::new_min(0, 10);

    assert_eq!(lim_i32.value, 0);

    lim_i32 += 7;
    assert_eq!(lim_i32.value, 7);

    lim_i32 += 7;
    assert_eq!(lim_i32.value, 10);

    lim_i32 -= 7;
    assert_eq!(lim_i32.value, 3);

    lim_i32 -= 7;
    assert_eq!(lim_i32.value, 0);
}

#[test]
fn compare_to_value_type() {
    use crate::common::num::limit::Limit;

    let lim_i32: Limit<i32> = Limit::new(0, 10, 5);

    assert!(lim_i32 == 5);

    let lim_f32: Limit<f32> = Limit::new(0.0, 10.0, 5.0);

    assert!(lim_f32 == 5.0);
}
