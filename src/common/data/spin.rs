use super::dir4::Dir4;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Spin {
    Ccw,
    Cw,
}

impl Spin {
    pub fn is_ccw(self) -> bool {
        self == Self::Ccw
    }

    pub fn opposite(self) -> Self {
        match self {
            Spin::Ccw => Spin::Cw,
            Spin::Cw => Spin::Ccw,
        }
    }

    pub fn as_dir_4(self) -> Dir4 {
        match self {
            Spin::Ccw => Dir4::W,
            Spin::Cw => Dir4::E,
        }
    }
}
