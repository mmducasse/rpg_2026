#![allow(dead_code)]
#![allow(unused)]

use macroquad::prelude::*;

mod input;
mod objects;
mod ui;

use crate::input::buttons::ButtonsState;
use crate::ui::buttons::Buttons;
use crate::ui::game::Game;

const PORTRAIT_W: f32 = 400.0;
const PORTRAIT_H: f32 = 600.0;
const GAME_SIZE: f32 = 400.0;

fn window_conf() -> Conf {
    Conf {
        window_title: String::from("Rpg2026"),
        window_width: PORTRAIT_W as i32,
        window_height: PORTRAIT_H as i32,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let game_rect = Rect::new(0.0, 0.0, GAME_SIZE, GAME_SIZE);
    let buttons_rect = Rect::new(0.0, GAME_SIZE, PORTRAIT_W, PORTRAIT_H - GAME_SIZE);

    let mut game = Game::new(game_rect);
    let buttons = Buttons::new(buttons_rect);

    let mut buttons_state = ButtonsState::new();

    loop {
        // Collect input: on-screen buttons take priority, then keyboard
        buttons_state.add_keyboard_state();
        buttons.update(&mut buttons_state);

        game.handle_buttons(&buttons_state);

        game.update(get_frame_time());

        clear_background(BLACK);
        game.draw();
        buttons.draw();

        buttons_state.advance();
        next_frame().await;
    }
}
