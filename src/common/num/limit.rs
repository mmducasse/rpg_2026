use std::{
    fmt::Display,
    ops::{Add, AddAssign, Sub, SubAssign},
};

#[derive(Clone, Copy)]
pub struct Limit<T> {
    pub min: T,
    pub max: T,
    pub value: T,
}

impl<T> Limit<T>
where
    T: Copy,
{
    pub const fn new(min: T, max: T, value: T) -> Self {
        Self { min, max, value }
    }

    pub const fn new_min(min: T, max: T) -> Self {
        Self {
            min,
            max,
            value: min,
        }
    }

    pub const fn new_max(min: T, max: T) -> Self {
        Self {
            min,
            max,
            value: max,
        }
    }

    pub fn value(&self) -> T {
        self.value
    }

    pub fn set_min(&mut self) {
        self.value = self.min;
    }

    pub fn set_max(&mut self) {
        self.value = self.max;
    }
}

impl<T> Limit<T>
where 
    T: PartialEq
{
    #[inline]
    pub fn is_at_min(&self) -> bool {
        self.value == self.min
    }

    #[inline]
    pub fn is_at_max(&self) -> bool {
        self.value == self.max
    }
}

impl<T> Limit<T>
where
    T: Ord + Copy,
{
    pub fn set(&mut self, value: T) {
        self.value = value.min(self.max).max(self.min);
    }
}

impl<T> Display for Limit<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.value.fmt(f)
    }
}

impl<T> AddAssign<T> for Limit<T>
where
    T: Add<T, Output = T> + PartialOrd + Copy,
{
    fn add_assign(&mut self, rhs: T) {
        let next = self.value + rhs;
        self.value = if self.max < next { self.max } else { next };
    }
}

impl<T> SubAssign<T> for Limit<T>
where
    T: Sub<T, Output = T> + PartialOrd + Copy,
{
    fn sub_assign(&mut self, rhs: T) {
        let next = self.value - rhs;
        self.value = if self.min > next { self.min } else { next };
    }
}

impl<T> PartialEq<T> for Limit<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &T) -> bool {
        self.value == *other
    }
}
