use macroquad::prelude::*;

use crate::input::buttons::ButtonsState;

const SPEED: f32 = 8.0;
const SIZE: f32 = 16.0;

pub struct Player {
    pos: Vec2,
    bounds: Rect,
}

impl Player {
    pub fn new(bounds: Rect) -> Self {
        let pos = Vec2::new(
            bounds.x + bounds.w / 2.0 - SIZE / 2.0,
            bounds.y + bounds.h / 2.0 - SIZE / 2.0,
        );
        Self { pos, bounds }
    }

    pub fn update(&mut self, buttons_state: &ButtonsState) {
        let dir = buttons_state.get_dpad();

        let delta = Vec2::new(dir.0 as f32 * SPEED, dir.1 as f32 * SPEED);

        self.pos += delta;
        self.pos.x = self
            .pos
            .x
            .clamp(self.bounds.x, self.bounds.x + self.bounds.w - SIZE);
        self.pos.y = self
            .pos
            .y
            .clamp(self.bounds.y, self.bounds.y + self.bounds.h - SIZE);
    }

    pub fn draw(&self) {
        draw_rectangle(
            self.pos.x,
            self.pos.y,
            SIZE,
            SIZE,
            Color::from_rgba(100, 180, 255, 255),
        );
    }
}
