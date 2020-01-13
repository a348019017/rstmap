extern crate geo_types;
use crate::tile::TileIndex;
use geo_types::Rect;

#[derive(Debug)]
pub struct TileInfo {
    pub index: TileIndex,
    pub extent: Rect<f64>,
}
