use macroquad::color::Color;

use crate::common::{
    mq::{self, texture::Texture},
    num::{
        irect::{IRect, rect},
        ivec2::IVec2,
    },
};

#[derive(Clone)]
pub struct Pane {
    abs_bounds: IRect,
    scale: f32,
}

impl Pane {
    pub fn new(size: IVec2, scale: f32) -> Self {
        Self {
            abs_bounds: IRect::of_size(size),
            scale,
        }
    }

    pub fn child(&self, rel_bounds: IRect) -> Self {
        Self {
            abs_bounds: rel_bounds.offset_by(self.abs_bounds.pos),
            scale: self.scale,
        }
    }

    pub fn abs_bounds(&self) -> IRect {
        self.abs_bounds
    }

    pub fn split_y(&self, sizes_y: &[i32]) -> Vec<Self> {
        let mut offset_y = 0;
        let mut children = vec![];

        let w = self.abs_bounds.w();

        for size_y in sizes_y {
            let y = offset_y;
            let h = *size_y;

            let rel_bounds = rect(0, y, w, h);

            children.push(self.child(rel_bounds));

            offset_y += *size_y;
        }

        let y = offset_y;
        let h = self.abs_bounds.h() - offset_y;

        let rel_bounds = rect(0, y, w, h);

        children.push(self.child(rel_bounds));

        children
    }

    pub fn clear_background(&self, color: Color) {
        mq::draw::draw_rect_scaled(self.abs_bounds, color, self.scale);
    }

    pub fn draw_rect(&self, rect: IRect, color: Color) {
        let abs_rect = rect.offset_by(self.abs_bounds.pos);
        mq::draw::draw_rect_scaled(abs_rect, color, self.scale);
    }

    pub fn draw_texture(&self, texture: Texture, src: Option<IRect>, dst: IVec2) {
        let dst = dst + self.abs_bounds.pos;
        mq::draw::draw_texture_scaled(texture, src, dst, self.scale);
    }
}
