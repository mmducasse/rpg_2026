use super::{fvec2::FVec2, ivec3::IVec3, math::max_f32, vec3::Vec3};

/// 3D vector of `f32` values.
pub type FVec3 = Vec3<f32>;

/// Creates a `Vec3<f32>` from x, y and z position.
#[inline]
pub const fn f3(x: f32, y: f32, z: f32) -> FVec3 {
    FVec3 { x, y, z }
}

impl FVec3 {
    pub const ZERO: FVec3 = FVec3::splat(0.0);

    pub fn xy(self) -> FVec2 {
        FVec2 {
            x: self.x,
            y: self.y,
        }
    }

    pub fn as_ivec3(&self) -> IVec3 {
        IVec3 {
            x: self.x as i32,
            y: self.y as i32,
            z: self.z as i32,
        }
    }

    pub fn abs(self) -> Self {
        Self {
            x: self.x.abs(),
            y: self.y.abs(),
            z: self.z.abs(),
        }
    }

    pub fn max(self) -> f32 {
        max_f32(&[self.x, self.y, self.z])
    }

    pub fn magnitude(&self) -> f32 {
        let x = self.x;
        let y = self.y;
        let z = self.z;

        f32::sqrt(x * x + y * y + z * z)
    }

    pub fn normalize(&self) -> FVec3 {
        let mag = self.magnitude();
        f3(self.x / mag, self.y / mag, self.z / mag)
    }
}
