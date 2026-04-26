#![allow(dead_code)]
#![allow(unused)]

mod common;
mod input;
mod objects;
mod ui;

use macroquad::input::{KeyCode, is_key_down};
use macroquad::time::get_frame_time;
use macroquad::window::{Conf, next_frame};

use crate::common::anim::pane::Pane;
use crate::common::num::irect::rect;
use crate::common::num::ivec2::{IVec2, i2};
use crate::input::buttons::ButtonsState;
use crate::ui::buttons::ButtonsUI;
use crate::ui::game::{Game, GameCtx};

const GAME_SIZE_W: i32 = 16 * 10;
const GAME_SIZE_H: i32 = 16 * 9;
const PORTRAIT_W: i32 = GAME_SIZE_W;
const PORTRAIT_H: i32 = PORTRAIT_W * 3 / 2;
const WINDOW_SIZE: IVec2 = i2(PORTRAIT_W, PORTRAIT_H);
const SCALE: f32 = 3.0;

fn window_conf() -> Conf {
    Conf {
        window_title: String::from("Rpg2026"),
        window_width: (PORTRAIT_W as f32 * SCALE) as i32,
        window_height: (PORTRAIT_H as f32 * SCALE) as i32,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let window_pane = Pane::new(WINDOW_SIZE, SCALE);
    let child_panes = window_pane.split_y(&[GAME_SIZE_H]);
    let game_pane = child_panes[0].clone();
    let buttons_pane = child_panes[1].clone();

    let mut game = Game::new(game_pane.abs_bounds());
    let buttons = ButtonsUI::new(buttons_pane.abs_bounds());

    let mut buttons_state = ButtonsState::new();

    // Main game loop.
    loop {
        buttons_state.add_keyboard_state();
        buttons.update(&mut buttons_state);

        game.update(GameCtx {
            delta_time: get_frame_time(),
            buttons_state: &buttons_state,
        });

        game.draw(&game_pane);
        buttons.draw(&buttons_pane);

        buttons_state.advance();
        next_frame().await;

        if is_key_down(KeyCode::Escape) {
            return;
        }
    }
}
