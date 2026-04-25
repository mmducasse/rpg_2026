pub struct Countdown {
    rem: u32,
}

impl Countdown {
    pub fn new(duration: u32) -> Self {
        Self { rem: duration }
    }

    pub fn remaining(&self) -> u32 {
        self.rem
    }

    pub fn decrement(&mut self) {
        if self.rem > 0 {
            self.rem -= 1;
        }
    }

    pub fn is_done(&self) -> bool {
        self.rem == 0
    }
}
