use macroquad::input::{KeyCode, is_key_down, is_key_pressed};

const NUM_BUTTONS: usize = 6;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ButtonId {
    Up,
    Down,
    Left,
    Right,
    A,
    B,
}

impl ButtonId {
    fn idx(self) -> usize {
        self as usize
    }
}

pub struct ButtonsState {
    curr: [bool; NUM_BUTTONS],
    prev: [bool; NUM_BUTTONS],
}

impl ButtonsState {
    pub fn new() -> Self {
        Self {
            curr: [false; NUM_BUTTONS],
            prev: [false; NUM_BUTTONS],
        }
    }

    pub fn advance(&mut self) {
        self.prev = self.curr;
        self.curr = [false; NUM_BUTTONS];
    }

    pub fn add_keyboard_state(&mut self) {
        put_keyboard_events(&mut self.curr);
    }

    pub fn set_button_state(&mut self, button: ButtonId, is_down: bool) {
        self.curr[button.idx()] = is_down;
    }

    pub fn is_just_pressed(&self, button: ButtonId) -> bool {
        self.curr[button.idx()] && !self.prev[button.idx()]
    }

    pub fn is_down(&self, button: ButtonId) -> bool {
        self.curr[button.idx()]
    }

    pub fn get_dpad(&self) -> (i32, i32) {
        let mut x = 0;
        let mut y = 0;

        if self.curr[ButtonId::Left.idx()] {
            x -= 1;
        }
        if self.curr[ButtonId::Right.idx()] {
            x += 1;
        }
        if self.curr[ButtonId::Up.idx()] {
            y -= 1;
        }
        if self.curr[ButtonId::Down.idx()] {
            y += 1;
        }

        return (x, y);
    }
}

/// Maps keyboard keys to button events for desktop convenience.
fn put_keyboard_events(events: &mut [bool; NUM_BUTTONS]) {
    let mut check = |button_event: ButtonId, key_code: KeyCode| {
        if is_key_down(key_code) {
            events[button_event.idx()] = true;
        }
    };

    check(ButtonId::Up, KeyCode::Up);
    check(ButtonId::Down, KeyCode::Down);
    check(ButtonId::Left, KeyCode::Left);
    check(ButtonId::Right, KeyCode::Right);

    check(ButtonId::A, KeyCode::Z);
    check(ButtonId::B, KeyCode::X);
}
