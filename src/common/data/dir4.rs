#[derive(Default, Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Dir4 {
    #[default]
    N,
    E,
    S,
    W,
}

use std::{
    fmt::{self, Display},
    ops::Add,
    str::FromStr,
};

use Dir4::*;

use crate::common::num::{
    fvec2::FVec2,
    ivec2::{i2, IVec2},
    math::mod_,
};

use super::spin::Spin;

impl Dir4 {
    pub const ALL: [Self; 4] = [N, E, S, W];

    pub const fn unit(&self) -> IVec2 {
        match self {
            N => i2(0, -1),
            E => i2(1, 0),
            S => i2(0, 1),
            W => i2(-1, 0),
        }
    }

    pub fn opposite(&self) -> Self {
        Self::from(*self as i32 + 2)
    }

    pub fn rotate(self, dir: Spin) -> Self {
        let offset = match dir {
            Spin::Ccw => -1,
            Spin::Cw => 1,
        };

        let idx = mod_((self as i32) + offset, 4) as usize;

        Self::ALL[idx]
    }

    pub fn is_vertical(&self) -> bool {
        matches!(self, N | S)
    }

    pub fn is_horizontal(&self) -> bool {
        matches!(self, W | E)
    }

    pub fn cw(&self, turns: i32) -> Self {
        let mut i = *self as i32;
        i += turns;

        Self::from(i)
    }

    pub fn from_ivec2(ivec2: IVec2) -> Option<Dir4> {
        use Dir4::*;

        if ivec2 == IVec2::splat(0) {
            None
        } else if ivec2.y.abs() >= ivec2.x.abs() {
            Some(if ivec2.y < 0 { N } else { S })
        } else {
            Some(if ivec2.x < 0 { W } else { E })
        }
    }

    pub fn from_fvec2(fvec2: FVec2) -> Option<Dir4> {
        use Dir4::*;

        if fvec2 == FVec2::ZERO {
            None
        } else if fvec2.y.abs() >= fvec2.x.abs() {
            Some(if fvec2.y < 0.0 { N } else { S })
        } else {
            Some(if fvec2.x < 0.0 { W } else { E })
        }
    }
}

impl From<i32> for Dir4 {
    fn from(i: i32) -> Self {
        use Dir4::*;
        match mod_(i, 4) {
            1 => E,
            2 => S,
            3 => W,
            _ => N,
        }
    }
}

impl Display for Dir4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Dir4::*;

        let str = match *self {
            N => "N",
            E => "E",
            S => "S",
            W => "W",
        };

        write!(f, "{}", str)
    }
}

impl FromStr for Dir4 {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Dir4::*;

        match s.to_lowercase().as_str() {
            "n" => Ok(N),
            "e" => Ok(E),
            "s" => Ok(S),
            "w" => Ok(W),
            _ => Err(()),
        }
    }
}

impl Add for Dir4 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::from(self as i32 + rhs as i32)
    }
}
