use crate::{common::anim::pane::Pane, objects::player::Player};

pub trait Object {
    fn update(&mut self);
    fn draw(&self, pane: &Pane);
    fn as_object_type<'a>(&'a self) -> ObjectType<'a>;
    fn as_object_type_mut<'a>(&'a mut self) -> ObjectTypeMut<'a>;
}

pub enum ObjectType<'a> {
    Player(&'a Player),
}

pub enum ObjectTypeMut<'a> {
    Player(&'a mut Player),
}
