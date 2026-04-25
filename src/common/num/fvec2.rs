use super::{fvec3::FVec3, ivec2::IVec2, ivec3::IVec3, vec2::Vec2};

/// 2D vector of `f32` values.
pub type FVec2 = Vec2<f32>;

/// Creates a `Vec2<f32>` from x and y position.
#[inline]
pub const fn f2(x: f32, y: f32) -> FVec2 {
    FVec2 { x, y }
}

impl FVec2 {
    pub const ZERO: FVec2 = FVec2::splat(0.0);

    pub fn as_fvec3(self) -> FVec3 {
        FVec3 {
            x: self.x,
            y: self.y,
            z: 0.0,
        }
    }

    pub fn as_ivec2(&self) -> IVec2 {
        IVec2 {
            x: self.x as i32,
            y: self.y as i32,
        }
    }

    pub fn as_ivec3(self) -> IVec3 {
        self.as_fvec3().as_ivec3()
    }

    pub fn abs(self) -> Self {
        Self {
            x: self.x.abs(),
            y: self.y.abs(),
        }
    }

    pub fn magnitude(&self) -> f32 {
        let x = self.x;
        let y = self.y;

        f32::sqrt(x * x + y * y)
    }

    pub fn normalize(&self) -> FVec2 {
        let mag = self.magnitude();
        f2(self.x / mag, self.y / mag)
    }
}
