use crate::{
    common::{anim::pane::Pane, num::irect::IRect},
    input::buttons::ButtonsState,
    objects::{object::Object, player::Player},
};
use macroquad::prelude::*;

pub struct Game {
    player: Player,
}

pub struct GameCtx<'a> {
    pub delta_time: f32,
    pub buttons_state: &'a ButtonsState,
}

impl Game {
    pub fn new(game_rect: IRect) -> Self {
        Self {
            player: Player::new(game_rect.center()),
        }
    }

    pub fn update(&mut self, ctx: GameCtx) {
        self.player.update(ctx);
    }

    pub fn draw(&self, pane: &Pane) {
        pane.clear_background(macroquad::color::BLACK);
        self.player.draw(pane);
    }
}
