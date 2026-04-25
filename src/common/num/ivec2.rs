use super::{fvec2::FVec2, fvec3::FVec3, ivec3::IVec3, vec2::Vec2};

/// 2D vector of `i32` values.
pub type IVec2 = Vec2<i32>;

/// Creates a `Vec2<i32>` from x and y position.
#[inline]
pub const fn i2(x: i32, y: i32) -> IVec2 {
    IVec2 { x, y }
}

impl IVec2 {
    pub const ZERO: IVec2 = IVec2::splat(0);

    pub fn as_ivec3(self) -> IVec3 {
        IVec3 {
            x: self.x,
            y: self.y,
            z: 0,
        }
    }

    pub fn as_fvec2(self) -> FVec2 {
        FVec2 {
            x: self.x as f32,
            y: self.y as f32,
        }
    }

    pub fn as_fvec3(self) -> FVec3 {
        self.as_ivec3().as_fvec3()
    }

    pub fn abs(self) -> Self {
        Self {
            x: self.x.abs(),
            y: self.y.abs(),
        }
    }

    pub fn min(self, b: IVec2) -> IVec2 {
        Self {
            x: self.x.min(b.x),
            y: self.y.min(b.y),
        }
    }

    pub fn max(self, b: IVec2) -> IVec2 {
        Self {
            x: self.x.max(b.x),
            y: self.y.max(b.y),
        }
    }

    pub const fn add(a: IVec2, b: IVec2) -> IVec2 {
        IVec2 {
            x: a.x + b.x,
            y: a.y + b.y,
        }
    }

    pub const fn sub(a: IVec2, b: IVec2) -> IVec2 {
        IVec2 {
            x: a.x - b.x,
            y: a.y - b.y,
        }
    }

    pub const fn mul(a: IVec2, b: IVec2) -> IVec2 {
        IVec2 {
            x: a.x * b.x,
            y: a.y * b.y,
        }
    }

    pub const fn div(a: IVec2, b: IVec2) -> IVec2 {
        IVec2 {
            x: a.x / b.x,
            y: a.y / b.y,
        }
    }

    pub const fn wrap(idx: i32, width: i32) -> IVec2 {
        assert!(idx >= 0);
        assert!(width >= 0);
        IVec2 {
            x: idx % width,
            y: idx / width,
        }
    }

    pub const fn unwrap(pos: IVec2, width: i32) -> i32 {
        assert!(pos.x >= 0);
        assert!(pos.y >= 0);
        assert!(width >= 0);
        (pos.y * width) + pos.x
    }
}