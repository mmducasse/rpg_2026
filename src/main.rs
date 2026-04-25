use macroquad::prelude::*;

mod objects;
mod ui;

use crate::ui::buttons::{ButtonEvent, Buttons};
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

    loop {
        // Collect input: on-screen buttons take priority, then keyboard
        let event: Option<ButtonEvent> = buttons.update().or_else(|| keyboard_event());

        if let Some(ev) = event {
            game.handle_button(ev);
        }

        game.update(get_frame_time());

        clear_background(BLACK);
        game.draw();
        buttons.draw();

        next_frame().await;
    }
}

/// Maps keyboard keys to button events for desktop convenience.
fn keyboard_event() -> Option<ButtonEvent> {
    if is_key_pressed(KeyCode::Up) || is_key_pressed(KeyCode::W) {
        Some(ButtonEvent::Up)
    } else if is_key_pressed(KeyCode::Down) || is_key_pressed(KeyCode::S) {
        Some(ButtonEvent::Down)
    } else if is_key_pressed(KeyCode::Left) || is_key_pressed(KeyCode::A) {
        Some(ButtonEvent::Left)
    } else if is_key_pressed(KeyCode::Right) || is_key_pressed(KeyCode::D) {
        Some(ButtonEvent::Right)
    } else if is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::Space) {
        Some(ButtonEvent::A)
    } else {
        None
    }
}
