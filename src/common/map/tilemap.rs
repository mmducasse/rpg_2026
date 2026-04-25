use crate::common::{
    data::arr2d::Arr2D,
    num::ivec2::{i2, IVec2},
};

use super::{
    tiled_json::tilemap::{JsonTilemap, Layer},
    tileset::Tileset,
};

pub struct Tilemap<Tile> {
    pub name: String,
    pub tile_srcs: Arr2D<Option<IVec2>>,
    pub tileset: Tileset<Tile>,
}

impl<Tile> Tilemap<Tile>
where
    Tile: Clone,
{
    pub fn size(&self) -> IVec2 {
        self.tile_srcs.size()
    }

    pub fn get(&self, pos: IVec2) -> Option<&Tile> {
        let src_pos = (*self.tile_srcs.get(pos)?)?;
        self.tileset.tiles.get(src_pos)
    }
}

impl<Tile> Clone for Tilemap<Tile>
where
    Tile: Clone,
{
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            tile_srcs: self.tile_srcs.clone(),
            tileset: self.tileset.clone(),
        }
    }
}

impl<Tile> Tilemap<Tile>
where
    Tile: Clone,
{
    /// Converts the Tilelayers of a `JsonTileset` object into a Vec of `Tilemap`.
    pub fn from_json(json: &JsonTilemap, tileset: Tileset<Tile>) -> Result<Vec<Self>, String> {
        let size = i2(json.width, json.height);
        let mut tilemaps = vec![];

        for layer in &json.layers {
            if let Layer::Tilelayer { data, .. } = layer {
                let tileset_cols = tileset.tiles.size().x;
                let tile_srcs: Vec<Option<IVec2>> = data
                    .iter()
                    .map(|&id| {
                        if id == 0 {
                            None
                        } else {
                            let id = (id - 1) as i32;
                            Some(i2(id % tileset_cols, id / tileset_cols))
                        }
                    })
                    .collect();

                let tile_srcs = Arr2D::new(tile_srcs, size.x as usize);

                tilemaps.push(Tilemap {
                    name: layer.name().to_owned(),
                    tile_srcs,
                    tileset: tileset.clone(),
                });
            }
        }

        Ok(tilemaps)
    }
}
