use super::ivec2::{i2, IVec2};

pub fn lerp(a: i32, b: i32, f: f32) -> i32 {
    let x0;
    let x1;
    let _f;
    if a <= b {
        x0 = a as f32;
        x1 = b as f32;
        _f = f;
    } else {
        x0 = b as f32;
        x1 = a as f32;
        _f = 1.0 - f;
    }

    let x = x0 + ((x1 - x0) * _f);

    x as i32
}

pub fn lerp_c(a: i32, b: i32, f: f32) -> i32 {
    let f = f.clamp(0.0, 1.0);
    lerp(a, b, f)
}

pub fn lerp_p(a: IVec2, b: IVec2, f: f32) -> IVec2 {
    i2(lerp(a.x, b.x, f), lerp(a.y, b.y, f))
}

pub const fn mod_(x: i32, d: i32) -> i32 {
    if x >= 0 {
        x % d
    } else {
        (d + x) % d
    }
}

pub const fn mod_p(p: IVec2, d: IVec2) -> IVec2 {
    i2(mod_(p.x, d.x), mod_(p.y, d.y))
}

pub fn max_f32(nums: &[f32]) -> f32 {
    let mut max = f32::MIN;

    for num in nums {
        max = f32::max(max, *num);
    }

    max
}
