use macroquad::prelude::*;

use crate::input::buttons::{ButtonId, ButtonsState};

struct ButtonDef {
    rect: Rect,
    label: &'static str,
    event: ButtonId,
    color: Color,
}

pub struct Buttons {
    buttons: Vec<ButtonDef>,
    view_rect: Rect,
}

impl Buttons {
    pub fn new(view_rect: Rect) -> Self {
        let btn_size = view_rect.h * 0.275;
        let stride = view_rect.h * 0.30;

        let dpad_cx = view_rect.x + view_rect.w * 0.25;
        let dpad_cy = view_rect.y + view_rect.h * 0.50;

        let a_cx = view_rect.x + view_rect.w * 0.70;
        let b_cx = view_rect.x + view_rect.w * 0.875;
        let action_cy = dpad_cy;

        let dpad = Color::from_rgba(70, 70, 110, 255);
        let a_col = Color::from_rgba(170, 50, 50, 255);
        let b_col = Color::from_rgba(50, 90, 170, 255);

        let make = |cx: f32, cy: f32, label, event, color| ButtonDef {
            rect: Rect::new(cx - btn_size / 2.0, cy - btn_size / 2.0, btn_size, btn_size),
            label,
            event,
            color,
        };

        let buttons = vec![
            make(dpad_cx, dpad_cy - stride, "U", ButtonId::Up, dpad),
            make(dpad_cx, dpad_cy + stride, "D", ButtonId::Down, dpad),
            make(dpad_cx - stride, dpad_cy, "L", ButtonId::Left, dpad),
            make(dpad_cx + stride, dpad_cy, "R", ButtonId::Right, dpad),
            make(a_cx, action_cy, "A", ButtonId::A, a_col),
            make(b_cx, action_cy, "B", ButtonId::B, b_col),
        ];

        Self { buttons, view_rect }
    }

    pub fn update(&self, buttons_state: &mut ButtonsState) {
        // Mouse click
        if is_mouse_button_pressed(MouseButton::Left) {
            let (mx, my) = mouse_position();
            let pos = Vec2::new(mx, my);
            for btn in &self.buttons {
                if btn.rect.contains(pos) {
                    //return Some(btn.event);
                    buttons_state.set_button_state(btn.event, true);
                }
            }
        }

        // Touch input (for mobile/browser)
        for touch in touches() {
            if touch.phase == TouchPhase::Started {
                for btn in &self.buttons {
                    if btn.rect.contains(touch.position) {
                        //return Some(btn.event);
                        buttons_state.set_button_state(btn.event, true);
                    }
                }
            }
        }
    }

    pub fn draw(&self) {
        draw_rectangle(
            self.view_rect.x,
            self.view_rect.y,
            self.view_rect.w,
            self.view_rect.h,
            Color::from_rgba(25, 25, 35, 255),
        );
        draw_line(
            self.view_rect.x,
            self.view_rect.y,
            self.view_rect.x + self.view_rect.w,
            self.view_rect.y,
            2.0,
            Color::from_rgba(80, 80, 80, 255),
        );

        for btn in &self.buttons {
            draw_rectangle(btn.rect.x, btn.rect.y, btn.rect.w, btn.rect.h, btn.color);
            draw_rectangle_lines(
                btn.rect.x,
                btn.rect.y,
                btn.rect.w,
                btn.rect.h,
                2.0,
                Color::from_rgba(200, 200, 200, 80),
            );

            let dims = measure_text(btn.label, None, 22, 1.0);
            let tx = btn.rect.x + (btn.rect.w - dims.width) / 2.0;
            let ty = btn.rect.y + (btn.rect.h + dims.height) / 2.0;
            draw_text(btn.label, tx, ty, 22.0, WHITE);
        }
    }
}
