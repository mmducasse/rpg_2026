use crate::common::num::{
    fvec2::FVec2,
    irect::{ir, IRect},
    ivec2::IVec2,
};
use macroquad::{
    prelude::{set_camera, set_default_camera, Camera2D, Rect, Vec2, WHITE},
    texture::{draw_texture_ex, render_target, DrawTextureParams, FilterMode, Texture2D},
    window::request_new_screen_size,
};

static mut WINDOW_EXISTS: bool = false;

pub struct Window {
    mq_camera: Camera2D,
    params: WindowParams,
}

pub struct WindowParams {
    pub resolution: IVec2,
    pub scale: f32,
}

impl Window {
    pub fn new(params: WindowParams) -> Self {
        unsafe {
            if WINDOW_EXISTS {
                panic!("Only one window should ever be created.");
            }
            WINDOW_EXISTS = true;
        }

        let resolution = params.resolution;
        let desired_size = resolution.as_fvec2() * FVec2::splat(params.scale);
        request_new_screen_size(desired_size.x, desired_size.y);

        Self {
            params,
            mq_camera: new_mq_camera(resolution),
        }
    }

    pub fn bounds(&self) -> IRect {
        ir(IVec2::ZERO, self.params.resolution)
    }

    pub fn mq_camera(&self) -> &Camera2D {
        &self.mq_camera
    }

    pub fn render_pass<F>(&self, render_action: F)
    where
        F: FnOnce(),
    {
        set_camera(self.mq_camera());

        render_action();

        self.render_camera();
    }

    fn render_camera(&self) {
        set_default_camera();

        let dest_size = {
            let s = self.params.resolution.as_fvec2();
            Vec2 { x: s.x, y: s.y }
        };
        let scale = self.params.scale;

        draw_texture_ex(
            self.render_target_texture().unwrap(),
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(dest_size * scale),
                source: None,
                rotation: 0.0,
                flip_x: false,
                flip_y: true,
                pivot: None,
            },
        );
    }

    fn render_target_texture(&self) -> Option<&Texture2D> {
        if let Some(render_target) = &self.mq_camera.render_target {
            return Some(&render_target.texture);
        }

        return None;
    }
}

fn new_mq_camera(size: IVec2) -> Camera2D {
    let render_target = render_target(size.x as u32, size.y as u32);
    render_target.texture.set_filter(FilterMode::Nearest);
    let rect = Rect::new(0.0, 0.0, size.x as f32, size.y as f32);
    let mut camera = Camera2D::from_display_rect(rect);
    camera.render_target = Some(render_target);
    camera
}
