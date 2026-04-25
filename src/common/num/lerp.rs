/// Linear interpolation between two values of type `Y`.
pub trait Lerp<Y> {
    /// Performs linear interpolation between two values: `a` and `b`.
    /// `lerp(a, b, 0.0) == a`
    /// `lerp(a, b, 1.0) == b`
    fn lerp(y0: Self, y1: Self, x: f32) -> Y;
}

impl Lerp<i32> for i32 {
    fn lerp(y0: Self, y1: Self, x: f32) -> i32 {
        ((y1 - y0) as f32 * x) as i32 + y0
    }
}

impl Lerp<f32> for f32 {
    fn lerp(y0: Self, y1: Self, x: f32) -> f32 {
        ((y1 - y0) * x) + y0
    }
}
