use std::rc::Rc;

use crate::common::{data::arr2d::Arr2D, mq::texture::Texture};

use super::tiled_json::tileset::{JsonTile, JsonTileset};

pub struct Tileset<Tile> {
    pub tiles: Rc<Arr2D<Tile>>,
    pub texture: Texture,
}

impl<Tile> Clone for Tileset<Tile>
where
    Tile: Clone,
{
    fn clone(&self) -> Self {
        Self {
            tiles: self.tiles.clone(),
            texture: self.texture.clone(),
        }
    }
}

impl<Tile> Tileset<Tile>
where
    Tile: Default + Clone,
{
    /// Converts a `JsonTileset` object into a `Tileset`.
    pub fn from_json<F>(
        json: &JsonTileset,
        texture: Texture,
        tile_fn: F,
    ) -> Result<Tileset<Tile>, String>
    where
        F: Fn(&JsonTile) -> Result<Tile, String>,
    {
        // Default the tileset.
        let tiles = vec![Tile::default(); json.tilecount];
        let mut tiles = Arr2D::new(tiles, json.columns);

        // Parse the special tiles.
        for tile in json.tiles.iter() {
            let idx = tile.id as usize;

            let tile = tile_fn(tile).or_else(|e| Err(e.to_string()))?;

            tiles.set_i(idx, tile);
        }

        Ok(Tileset {
            tiles: Rc::new(tiles),
            texture,
        })
    }
}
