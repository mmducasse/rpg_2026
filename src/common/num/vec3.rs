use std::{
    fmt::{Debug, Display},
    hash::Hash,
    ops::{Add, AddAssign, Div, Mul, MulAssign, Neg, Sub, SubAssign},
};

use super::{lerp::Lerp, vec2::Vec2};

/// 3D vector.
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Vec3<T>
where
    T: Copy,
{
    pub const fn splat(m: T) -> Self {
        Self { x: m, y: m, z: m }
    }

    pub fn truncate(self) -> Vec2<T> {
        Vec2 {
            x: self.x,
            y: self.y,
        }
    }
}

impl<T> Vec3<T>
where
    T: Add<T, Output = T> + Copy,
{
    pub fn sum(&self) -> T {
        self.x + self.y + self.z
    }
}

impl<T> Vec3<T>
where
    T: Mul<T, Output = T> + Copy,
{
    pub fn product(&self) -> T {
        self.x * self.y * self.z
    }
}

impl<T> Hash for Vec3<T>
where
    T: Hash,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
        self.z.hash(state);
    }
}

impl<T> Clone for Vec3<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Self {
            x: self.x.clone(),
            y: self.y.clone(),
            z: self.z.clone(),
        }
    }
}

impl<T> Copy for Vec3<T> where T: Copy {}

impl<T> Debug for Vec3<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Vec3")
            .field("x", &self.x)
            .field("y", &self.y)
            .field("z", &self.z)
            .finish()
    }
}

impl<T> Display for Vec3<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl<T> PartialEq for Vec3<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

impl<T> Eq for Vec3<T>
where
    T: Eq,
{
    fn assert_receiver_is_total_eq(&self) {}
}

impl<T> Neg for Vec3<T>
where
    T: Copy + Neg<Output = T>,
{
    type Output = Vec3<T>;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl<T> Add<Vec3<T>> for Vec3<T>
where
    T: Copy + Add<T, Output = T>,
{
    type Output = Vec3<T>;

    fn add(self, rhs: Vec3<T>) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<T> Sub<Vec3<T>> for Vec3<T>
where
    T: Copy + Sub<T, Output = T>,
{
    type Output = Vec3<T>;

    fn sub(self, rhs: Vec3<T>) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl<T> Mul<Vec3<T>> for Vec3<T>
where
    T: Copy + Mul<T, Output = T>,
{
    type Output = Vec3<T>;

    fn mul(self, rhs: Vec3<T>) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl<T> Mul<T> for Vec3<T>
where
    T: Copy + Mul<T, Output = T>,
{
    type Output = Vec3<T>;

    fn mul(self, rhs: T) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl<T> Div<Vec3<T>> for Vec3<T>
where
    T: Copy + Div<T, Output = T>,
{
    type Output = Vec3<T>;

    fn div(self, rhs: Vec3<T>) -> Self::Output {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}

impl<T> Div<T> for Vec3<T>
where
    T: Copy + Div<T, Output = T>,
{
    type Output = Vec3<T>;

    fn div(self, rhs: T) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl<T> AddAssign<Vec3<T>> for Vec3<T>
where
    T: Copy + AddAssign<T>,
{
    fn add_assign(&mut self, rhs: Vec3<T>) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl<T> SubAssign<Vec3<T>> for Vec3<T>
where
    T: Copy + SubAssign<T>,
{
    fn sub_assign(&mut self, rhs: Vec3<T>) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl<T> MulAssign<Vec3<T>> for Vec3<T>
where
    T: Copy + MulAssign<T>,
{
    fn mul_assign(&mut self, rhs: Vec3<T>) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}

impl<T> Lerp<Vec3<T>> for Vec3<T>
where
    T: Lerp<T>,
{
    fn lerp(y0: Self, y1: Self, x: f32) -> Vec3<T> {
        Self {
            x: T::lerp(y0.x, y1.x, x),
            y: T::lerp(y0.y, y1.y, x),
            z: T::lerp(y0.z, y1.z, x),
        }
    }
}
