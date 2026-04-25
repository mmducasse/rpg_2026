use crate::common::num::ivec2::IVec2;

use super::dir4::Dir4;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum DirH {
    L,
    R,
}

impl DirH {
    pub fn to_dir4(self) -> Dir4 {
        match self {
            DirH::L => Dir4::W,
            DirH::R => Dir4::E,
        }
    }

    pub fn from_x(x: i32) -> DirH {
        if x < 0 { DirH::L } else { DirH::R }
    }

    pub fn unit(self) -> IVec2 {
        self.to_dir4().unit()
    }

    pub fn opposite(self) -> DirH {
        match self {
            DirH::L => DirH::R,
            DirH::R => DirH::L,
        }
    }
}
