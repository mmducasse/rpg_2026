use macroquad::{
    color::{BLACK, Color},
    input::{MouseButton, TouchPhase, is_mouse_button_down, mouse_position, touches},
};

use crate::{
    SCALE,
    common::{
        anim::pane::Pane,
        num::{
            fvec2::f2,
            irect::{IRect, ir, rect},
            ivec2::{IVec2, i2},
        },
    },
    input::buttons::{ButtonId, ButtonsState},
};

const BUTTON_SIZE: IVec2 = IVec2::splat(16);

struct ButtonDef {
    rect: IRect,
    id: ButtonId,
    color: Color,
}

pub struct ButtonsUI {
    buttons: Vec<ButtonDef>,
    view_rect: IRect,
}

impl ButtonsUI {
    pub fn new(view_rect: IRect) -> Self {
        let dpad = Color::from_rgba(70, 70, 110, 255);
        let ab_col = Color::from_rgba(50, 90, 170, 255);

        let make = |grid_x: i32, grid_y: i32, id: ButtonId, color| {
            let pos = i2(grid_x, grid_y);
            ButtonDef {
                rect: ir(pos * BUTTON_SIZE, BUTTON_SIZE),
                id,
                color,
            }
        };

        let buttons = vec![
            make(0, 1, ButtonId::Left, dpad),
            make(1, 0, ButtonId::Up, dpad),
            make(2, 1, ButtonId::Right, dpad),
            make(1, 2, ButtonId::Down, dpad),
            make(4, 1, ButtonId::A, ab_col),
            make(6, 1, ButtonId::B, ab_col),
        ];

        Self { buttons, view_rect }
    }

    pub fn update(&self, buttons_state: &mut ButtonsState) {
        let offset = self.view_rect.pos;

        // Mouse click
        if is_mouse_button_down(MouseButton::Left) {
            let (mx, my) = mouse_position();
            let pos = f2(mx / SCALE, my / SCALE).as_ivec2() - offset;
            for btn in &self.buttons {
                if btn.rect.contains(pos) {
                    buttons_state.set_button_state(btn.id, true);
                }
            }
        }

        // Touch input (for mobile/browser)
        let phases = [
            TouchPhase::Started,
            TouchPhase::Moved,
            TouchPhase::Stationary,
        ];
        for touch in touches() {
            if phases.contains(&touch.phase) {
                for btn in &self.buttons {
                    let p = touch.position;
                    let p = i2(p.x as i32, p.y as i32) - offset;
                    if btn.rect.contains(p) {
                        buttons_state.set_button_state(btn.id, true);
                    }
                }
            }
        }
    }

    pub fn draw(&self, pane: &Pane) {
        pane.clear_background(macroquad::color::DARKBLUE);

        for btn in &self.buttons {
            draw_button(btn, pane);
        }
    }
}

fn draw_button(btn: &ButtonDef, pane: &Pane) {
    pane.draw_rect(btn.rect.expand(-1), BLACK);
    pane.draw_rect(btn.rect.expand(-2), btn.color);
}
