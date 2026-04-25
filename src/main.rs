#![allow(dead_code)]
#![allow(unused)]

use macroquad::prelude::*;

mod common;
mod input;
mod objects;
mod ui;

use crate::common::num::irect::rect;
use crate::input::buttons::ButtonsState;
use crate::ui::buttons::Buttons;
use crate::ui::game::Game;

const GAME_SIZE_W: i32 = 16 * 10;
const GAME_SIZE_H: i32 = 16 * 9;
const PORTRAIT_W: i32 = GAME_SIZE_W;
const PORTRAIT_H: i32 = PORTRAIT_W * 3 / 2;
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
    let game_rect = rect(0, 0, GAME_SIZE_W, GAME_SIZE_H);
    let buttons_rect = rect(0, GAME_SIZE_H, PORTRAIT_W, PORTRAIT_H - GAME_SIZE_H);

    let mut game = Game::new(game_rect);
    let buttons = Buttons::new(buttons_rect);

    let mut buttons_state = ButtonsState::new();

    let render_target = render_target(PORTRAIT_W as u32, PORTRAIT_H as u32);
    render_target.texture.set_filter(FilterMode::Nearest);

    let mut camera =
        Camera2D::from_display_rect(Rect::new(0.0, 0.0, PORTRAIT_W as f32, PORTRAIT_H as f32));
    camera.render_target = Some(render_target.clone());

    loop {
        // Collect input: on-screen buttons take priority, then keyboard
        buttons_state.add_keyboard_state();
        buttons.update(&mut buttons_state);

        game.handle_buttons(&buttons_state);
        game.update(get_frame_time());

        // Draw game content into the render target at logical resolution
        set_camera(&camera);
        clear_background(BLACK);
        game.draw();
        buttons.draw();

        // Scale the render target up to fill the window
        set_default_camera();
        clear_background(BLACK);
        draw_texture_ex(
            &render_target.texture,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(
                    PORTRAIT_W as f32 * SCALE,
                    PORTRAIT_H as f32 * SCALE,
                )),
                flip_y: true,
                ..Default::default()
            },
        );

        buttons_state.advance();
        next_frame().await;
    }
}
