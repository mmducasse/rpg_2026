use std::rc::Rc;

use macroquad::texture::Texture2D;

#[derive(Clone)]
pub struct Texture {
    inner: Rc<Texture2D>,
}

impl Texture {
    pub fn new(inner: Texture2D) -> Self {
        Self {
            inner: Rc::new(inner),
        }
    }

    pub fn from_bytes(bytes: &'static [u8]) -> Self {
        let texture_2d = Texture2D::from_file_with_format(bytes, None);
        Self::new(texture_2d)
    }

    pub fn inner(&self) -> &Texture2D {
        &self.inner
    }
}
