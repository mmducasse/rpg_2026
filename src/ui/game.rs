use crate::{
    common::{anim::pane::Pane, num::irect::IRect},
    input::buttons::ButtonsState,
    objects::{object::Object, player::Player},
};
use macroquad::prelude::*;

pub struct Game {
    game_rect: IRect,
    player: Player,
}

impl Game {
    pub fn new(game_rect: IRect) -> Self {
        Self {
            player: Player::new(game_rect.center()),
            game_rect,
        }
    }

    pub fn handle_buttons(&mut self, buttons_state: &ButtonsState) {
        self.player.update(buttons_state);
    }

    pub fn update(&mut self, _dt: f32) {}

    pub fn draw(&self, pane: &Pane) {
        pane.clear_background(macroquad::color::BLACK);
        self.player.draw(pane);
    }
}
