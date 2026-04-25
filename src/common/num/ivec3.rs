use super::{fvec3::FVec3, ivec2::IVec2, vec3::Vec3};

/// 3D vector of `i32` values.
pub type IVec3 = Vec3<i32>;

/// Creates a `Vec3<i32>` from x, y and z position.
#[inline]
pub const fn i3(x: i32, y: i32, z: i32) -> IVec3 {
    IVec3 { x, y, z }
}

impl IVec3 {
    pub const ZERO: IVec3 = IVec3::splat(0);

    pub fn xy(self) -> IVec2 {
        IVec2 {
            x: self.x,
            y: self.y,
        }
    }

    pub fn as_fvec3(self) -> FVec3 {
        FVec3 {
            x: self.x as f32,
            y: self.y as f32,
            z: self.z as f32,
        }
    }

    pub fn abs(self) -> Self {
        Self {
            x: self.x.abs(),
            y: self.y.abs(),
            z: self.z.abs(),
        }
    }

    pub const fn add(a: IVec3, b: IVec3) -> IVec3 {
        IVec3 {
            x: a.x + b.x,
            y: a.y + b.y,
            z: a.z + b.z,
        }
    }

    pub const fn mul(a: IVec3, b: IVec3) -> IVec3 {
        IVec3 {
            x: a.x * b.x,
            y: a.y * b.y,
            z: a.z * b.z,
        }
    }
}
