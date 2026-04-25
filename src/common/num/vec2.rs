use std::{
    fmt::{Debug, Display},
    hash::Hash,
    ops::{Add, AddAssign, Div, Mul, MulAssign, Neg, Rem, Sub, SubAssign},
};

use super::{lerp::Lerp, vec3::Vec3};

/// 2D vector.
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vec2<T>
where
    T: Copy,
{
    pub const fn splat(m: T) -> Self {
        Self { x: m, y: m }
    }

    pub fn extend(self, z: T) -> Vec3<T> {
        Vec3 {
            x: self.x,
            y: self.y,
            z,
        }
    }

    pub fn flip(self) -> Vec2<T> {
        Self {
            x: self.y,
            y: self.x,
        }
    }
}

impl<T> Vec2<T>
where
    T: Add<T, Output = T> + Copy,
{
    pub fn sum(&self) -> T {
        self.x + self.y
    }
}

impl<T> Vec2<T>
where
    T: Mul<T, Output = T> + Copy,
{
    pub fn product(&self) -> T {
        self.x * self.y
    }
}

impl<T> Default for Vec2<T>
where T: Default
{
    fn default() -> Self {
        Self { x: Default::default(), y: Default::default() }
    }
}

impl<T> Hash for Vec2<T>
where
    T: Hash,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

impl<T> Clone for Vec2<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Self {
            x: self.x.clone(),
            y: self.y.clone(),
        }
    }
}

impl<T> Copy for Vec2<T> where T: Copy {}

impl<T> Debug for Vec2<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Vec2")
            .field("x", &self.x)
            .field("y", &self.y)
            .finish()
    }
}

impl<T> Display for Vec2<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl<T> PartialEq for Vec2<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl<T> Eq for Vec2<T>
where
    T: Eq,
{
    fn assert_receiver_is_total_eq(&self) {}
}

impl<T> Neg for Vec2<T>
where
    T: Copy + Neg<Output = T>,
{
    type Output = Vec2<T>;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl<T> Add<Vec2<T>> for Vec2<T>
where
    T: Copy + Add<T, Output = T>,
{
    type Output = Vec2<T>;

    fn add(self, rhs: Vec2<T>) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T> Sub<Vec2<T>> for Vec2<T>
where
    T: Copy + Sub<T, Output = T>,
{
    type Output = Vec2<T>;

    fn sub(self, rhs: Vec2<T>) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T> Mul<Vec2<T>> for Vec2<T>
where
    T: Copy + Mul<T, Output = T>,
{
    type Output = Vec2<T>;

    fn mul(self, rhs: Vec2<T>) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl<T> Mul<T> for Vec2<T>
where
    T: Copy + Mul<T, Output = T>,
{
    type Output = Vec2<T>;

    fn mul(self, rhs: T) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl<T> Div<Vec2<T>> for Vec2<T>
where
    T: Copy + Div<T, Output = T>,
{
    type Output = Vec2<T>;

    fn div(self, rhs: Vec2<T>) -> Self::Output {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}

impl<T> Div<T> for Vec2<T>
where
    T: Copy + Div<T, Output = T>,
{
    type Output = Vec2<T>;

    fn div(self, rhs: T) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl<T> Rem<Vec2<T>> for Vec2<T>
where 
    T: Copy + Rem<T, Output = T>
{
    type Output = Vec2<T>;

    fn rem(self, rhs: Vec2<T>) -> Self::Output {
        Self {
            x: self.x % rhs.x,
            y: self.y % rhs.y,
        }
    }
}

impl<T> Rem<T> for Vec2<T>
where 
    T: Copy + Rem<T, Output = T>
{
    type Output = Vec2<T>;

    fn rem(self, rhs: T) -> Self::Output {
        Self {
            x: self.x % rhs,
            y: self.y % rhs,
        }
    }
}

impl<T> AddAssign<Vec2<T>> for Vec2<T>
where
    T: Copy + AddAssign<T>,
{
    fn add_assign(&mut self, rhs: Vec2<T>) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T> SubAssign<Vec2<T>> for Vec2<T>
where
    T: Copy + SubAssign<T>,
{
    fn sub_assign(&mut self, rhs: Vec2<T>) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl<T> MulAssign<Vec2<T>> for Vec2<T>
where
    T: Copy + MulAssign<T>,
{
    fn mul_assign(&mut self, rhs: Vec2<T>) {
        self.x *= rhs.x;
        self.y *= rhs.y;
    }
}

impl<T> Lerp<Vec2<T>> for Vec2<T>
where
    T: Lerp<T>,
{
    fn lerp(y0: Self, y1: Self, x: f32) -> Vec2<T> {
        Self {
            x: T::lerp(y0.x, y1.x, x),
            y: T::lerp(y0.y, y1.y, x),
        }
    }
}
