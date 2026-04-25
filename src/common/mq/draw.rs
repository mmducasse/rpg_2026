use macroquad::{
    math::Vec2,
    prelude::{Color, WHITE},
    texture::{DrawTextureParams, draw_texture_ex},
};

use crate::common::num::{irect::IRect, ivec2::IVec2};

use super::texture::Texture;

pub fn draw_rect(rect: IRect, color: Color) {
    macroquad::shapes::draw_rectangle(
        rect.x() as f32,
        rect.y() as f32,
        rect.w() as f32,
        rect.h() as f32,
        color,
    );
}

pub fn draw_rect_scaled(rect: IRect, color: Color, scale: f32) {
    macroquad::shapes::draw_rectangle(
        rect.x() as f32 * scale,
        rect.y() as f32 * scale,
        rect.w() as f32 * scale,
        rect.h() as f32 * scale,
        color,
    );
}

// pub fn draw_ellipse(center: IVec2, size: IVec2, color: Color) {
//     macroquad::shapes::draw_ellipse(center.x as f32, center.y as f32, size.x as f32, size.y as f32, 0.0, color);
// }

pub fn draw_texture(texture: Texture, src: Option<IRect>, dst: IVec2) {
    draw_texture_ex(
        texture.inner(),
        dst.x as f32,
        dst.y as f32,
        WHITE,
        DrawTextureParams {
            dest_size: None,
            source: src.map(|s| s.as_rect()),
            rotation: 0.0,
            flip_x: false,
            flip_y: false,
            pivot: None,
        },
    )
}

pub fn draw_texture_scaled(texture: Texture, src: Option<IRect>, dst: IVec2, scale: f32) {
    let size = texture.inner().size();

    draw_texture_ex(
        texture.inner(),
        dst.x as f32 * scale,
        dst.y as f32 * scale,
        WHITE,
        DrawTextureParams {
            dest_size: Some(size * Vec2::splat(scale)),
            source: src.map(|s| s.as_rect()),
            rotation: 0.0,
            flip_x: false,
            flip_y: false,
            pivot: None,
        },
    )
}
