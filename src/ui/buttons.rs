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
        let view_rect = macroquad::math::Rect::new(
            view_rect.x() as f32,
            view_rect.y() as f32,
            view_rect.w() as f32,
            view_rect.h() as f32,
        );

        let btn_size = 32;
        let stride = (view_rect.h * 0.30) as i32;

        let dpad_cx = (view_rect.x + view_rect.w * 0.25) as i32;
        let dpad_cy = (view_rect.y + view_rect.h * 0.50) as i32;

        let a_cx = (view_rect.x + view_rect.w * 0.70) as i32;
        let b_cx = (view_rect.x + view_rect.w * 0.875) as i32;
        let action_cy = dpad_cy;

        let dpad = Color::from_rgba(70, 70, 110, 255);
        let a_col = Color::from_rgba(170, 50, 50, 255);
        let b_col = Color::from_rgba(50, 90, 170, 255);

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
            make(4, 1, ButtonId::A, dpad),
            make(6, 1, ButtonId::B, dpad),
        ];

        let view_rect = rect(
            view_rect.x as i32,
            view_rect.y as i32,
            view_rect.w as i32,
            view_rect.h as i32,
        );
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
