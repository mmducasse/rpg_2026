use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TileProperties {
    pub name: String,
    pub value: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct JsonTile {
    pub id: i32,
    pub type_: Option<String>,
    pub properties: Option<Vec<TileProperties>>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct JsonTileset {
    pub columns: usize,
    pub image: String,
    pub imageheight: i32,
    pub imagewidth: i32,
    pub name: String,
    pub spacing: i32,
    pub tilecount: usize,
    pub tileheight: i32,
    pub tilewidth: i32,
    pub tiles: Vec<JsonTile>,
}

impl JsonTileset {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, String> {
        serde_json::from_slice(bytes).or_else(|e| Err(e.to_string()))
    }
}
