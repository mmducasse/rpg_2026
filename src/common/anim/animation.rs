use crate::common::num::ivec2::IVec2;

/// A sequence of tiles to be drawn from a sprite atlas.
pub struct Animation {
    pub tiles: Vec<IVec2>,
    pub size_in_tiles: IVec2,
    pub draw_offset: IVec2,
    pub frame_dur_s: f32,
    pub loops: bool,
}

impl Animation {
    /// The tile to draw at the given time (in seconds).
    pub fn at(&self, time_s: f32) -> IVec2 {
        let idx = self.idx(time_s);
        self.tiles[idx]
    }

    pub fn idx(&self, time_s: f32) -> usize {
        let len = self.tiles.len();
        let idx = (time_s / self.frame_dur_s) as usize;

        if self.loops {
            idx % len
        } else {
            idx.min(len - 1)
        }
    }

    pub fn total_dur_s(&self) -> f32 {
        self.tiles.len() as f32 * self.frame_dur_s
    }
}
