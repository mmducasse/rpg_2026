use std::{collections::HashMap, hash::Hash};

use macroquad::texture::Texture2D;

use super::texture::Texture;

pub struct Textures<TKey> {
    map: Option<HashMap<TKey, Texture>>,
}

impl<TKey> Textures<TKey>
where
    TKey: Hash + PartialEq + Eq + Clone,
{
    pub const fn new() -> Self {
        Self { map: None }
    }

    pub fn get_or_load(&mut self, id: TKey, map_fn: impl Fn(TKey) -> &'static [u8]) -> Texture {
        if self.map.is_none() {
            self.map = Some(HashMap::new());
        }

        let Some(map) = &mut self.map else {
            panic!();
        };

        if !map.contains_key(&id) {
            let bytes = map_fn(id.clone());
            let texture_2d = Texture2D::from_file_with_format(bytes, None);
            let texture = Texture::new(texture_2d);
            map.insert(id.clone(), texture);
        }

        map.get(&id).unwrap().clone()
    }

    pub fn unload(&mut self) {
        self.map.as_mut().unwrap().clear();
    }
}
