use super::time::delta_s;

/// A timer that completes after a given duration elapses.
#[derive(Clone, PartialEq)]
pub struct Timer {
    duration_s: f32,
    elapsed_s: f32,
}

impl Timer {
    pub const fn new(duration_s: f32) -> Self {
        Self {
            duration_s,
            elapsed_s: 0.0,
        }
    }

    pub const fn new_done(duration_s: f32) -> Self {
        Self {
            duration_s,
            elapsed_s: duration_s,
        }
    }

    pub fn reset(&mut self) {
        self.elapsed_s = 0.0;
    }

    pub fn update(&mut self) {
        self.elapsed_s += delta_s();
    }

    pub fn update_and_check(&mut self) -> bool {
        self.update();
        if self.is_done() {
            self.reset();
            true
        } else {
            false
        }
    }

    pub fn duration(&self) -> f32 {
        self.duration_s
    }
    pub fn elapsed(&self) -> f32 {
        self.elapsed_s
    }

    pub fn is_done(&self) -> bool {
        self.elapsed_s >= self.duration_s
    }

    pub fn completion(&self) -> f32 {
        self.elapsed_s / self.duration_s
    }
}
