use crate::{input::buttons::ButtonsState, objects::player::Player};
use macroquad::prelude::*;

pub struct Game {
    game_rect: Rect,
    player: Player,
}

impl Game {
    pub fn new(game_rect: Rect) -> Self {
        Self {
            player: Player::new(game_rect),
            game_rect,
        }
    }

    pub fn handle_buttons(&mut self, buttons_state: &ButtonsState) {
        self.player.update(buttons_state);
    }

    pub fn update(&mut self, _dt: f32) {}

    pub fn draw(&self) {
        draw_rectangle(
            self.game_rect.x,
            self.game_rect.y,
            self.game_rect.w,
            self.game_rect.h,
            Color::from_rgba(20, 30, 20, 255),
        );
        self.player.draw();
    }
}
