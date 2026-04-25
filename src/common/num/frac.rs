use std::{
    fmt::Display,
    ops::{Add, Mul, Sub},
};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Frac<const DEN: i32> {
    pub num: i32,
}

impl<const DEN: i32> Frac<DEN> {
    pub const fn whole(value: i32) -> Self {
        Self { num: value * DEN }
    }
}

impl<const DEN: i32> Add<Frac<DEN>> for Frac<DEN> {
    type Output = Frac<DEN>;

    fn add(self, rhs: Frac<DEN>) -> Self::Output {
        Frac {
            num: self.num + rhs.num,
        }
    }
}

impl<const DEN: i32> Sub<Frac<DEN>> for Frac<DEN> {
    type Output = Frac<DEN>;

    fn sub(self, rhs: Frac<DEN>) -> Self::Output {
        Frac {
            num: self.num - rhs.num,
        }
    }
}

impl<const DENA: i32, const DENB: i32> Mul<Frac<DENB>> for Frac<DENA> {
    type Output = Frac<DENA>;

    fn mul(self, rhs: Frac<DENB>) -> Self::Output {
        Frac {
            num: (self.num * rhs.num) / DENB,
        }
    }
}

impl<const DEN: i32> Into<f32> for Frac<DEN> {
    fn into(self) -> f32 {
        (self.num as f32) / (DEN as f32)
    }
}

impl<const DEN: i32> From<i32> for Frac<DEN> {
    fn from(i: i32) -> Self {
        Frac { num: i * DEN }
    }
}

impl<const DEN: i32> From<f32> for Frac<DEN> {
    fn from(f: f32) -> Self {
        Frac {
            num: (f * DEN as f32) as i32,
        }
    }
}

impl<const DEN: i32> Display for Frac<DEN> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.num, DEN)
    }
}
