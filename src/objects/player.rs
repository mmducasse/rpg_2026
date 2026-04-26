use macroquad::color::BLUE;

use crate::{
    common::{
        anim::pane::Pane,
        num::{
            irect::{IRect, ir},
            ivec2::{IVec2, i2},
        },
    },
    input::buttons::ButtonsState,
    objects::object::{Object, ObjectType, ObjectTypeMut},
};

const SPEED: i32 = 2;
const SIZE: i32 = 16;

pub struct Player {
    pos: IVec2,
}

impl Player {
    pub fn new(pos: IVec2) -> Self {
        Self { pos }
    }

    pub fn bounds(&self) -> IRect {
        ir(self.pos, IVec2::splat(SIZE))
    }

    pub fn update(&mut self, buttons_state: &ButtonsState) {
        let dir = buttons_state.get_dpad();

        let delta = i2(dir.0 * SPEED, dir.1 * SPEED);

        self.pos += delta;
        self.pos.x = self.pos.x.clamp(
            self.bounds().x(),
            self.bounds().x() + self.bounds().w() - SIZE,
        );
        self.pos.y = self.pos.y.clamp(
            self.bounds().y(),
            self.bounds().y() + self.bounds().h() - SIZE,
        );
    }
}

impl Object for Player {
    fn update(&mut self) {
        todo!()
    }

    fn draw(&self, pane: &Pane) {
        pane.draw_rect(self.bounds(), BLUE);
    }

    fn as_object_type<'a>(&'a self) -> ObjectType<'a> {
        ObjectType::Player(self)
    }

    fn as_object_type_mut<'a>(&'a mut self) -> ObjectTypeMut<'a> {
        ObjectTypeMut::Player(self)
    }
}
