use std::{
    fmt::{Debug, Display},
    ops::{Div, Sub},
};

use super::lerp::Lerp;

/// A range that spans two values (inclusive).
pub struct Range<T> {
    pub a: T,
    pub b: T,
}

impl<T> Range<T> {
    pub const fn new(a: T, b: T) -> Self {
        Self { a, b }
    }
}

impl<T> Range<T>
where
    T: PartialOrd<T>,
{
    pub fn contains(&self, value: T) -> bool {
        self.a <= value && value <= self.b
    }
}

impl<T> Range<T>
where
    T: Clone + Copy + PartialOrd<T>,
{
    pub fn abs(&self) -> Range<T> {
        if self.b < self.a {
            Range::new(self.b, self.a)
        } else {
            self.clone()
        }
    }
}

impl<T> Range<T>
where
    T: Sub<T, Output = T> + Clone,
{
    pub fn delta(&self) -> T {
        self.b.clone() - self.a.clone()
    }
}

impl<T> Range<T>
where
    T: Lerp<T> + Clone,
{
    pub fn lerp(&self, x: f32) -> T {
        T::lerp(self.a.clone(), self.b.clone(), x)
    }
}

impl<T> Clone for Range<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Self {
            a: self.a.clone(),
            b: self.b.clone(),
        }
    }
}

impl<T> Copy for Range<T> where T: Clone + Copy {}

impl<T> Debug for Range<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Range")
            .field("a", &self.a)
            .field("b", &self.b)
            .finish()
    }
}

impl<T> Display for Range<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {}]", self.a, self.b)
    }
}

impl<T> PartialEq<Range<T>> for Range<T>
where
    T: PartialEq<T>,
{
    fn eq(&self, other: &Range<T>) -> bool {
        self.a == other.a && self.b == other.b
    }
}

impl<T> Div<T> for Range<T>
where T: Div<T, Output = T> + Clone
{
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Self {
            a: self.a / rhs.clone(),
            b: self.b / rhs,
        }
    }
}