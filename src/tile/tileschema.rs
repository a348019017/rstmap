extern crate geo_types;
use crate::core::Resolution;
use crate::tile::world_to_tile;
use crate::tile::TileInfo;
use crate::tile::TileRange;
use crate::tile::*;
use geo_types::Rect;
use geo_types::{Coordinate, CoordinateType};
use std::collections::HashMap;

#[derive(PartialEq, Debug, Clone)]
pub enum YAxis {
    /// The y-axis direction of the tiles match that of the map. This is used by TMS.
    TMS,
    /// The y-axis direction is inverted compared to that of the map. This is used by OpenStreetMap
    OSM,
}

#[derive(Debug, Clone)]
//tileschma 应该使用结构体，至于其中的方法可以再trait描述,至于墨卡托等，应该使用enum定义，这也是和JavaC#的最大区别
pub struct Tileschema {
    //返回其名称的借用
    pub name: String,
    pub srs: String,
    pub extent: Rect<f64>,
    //分辨率集合
    pub resolutions: HashMap<i32, Resolution>,
    pub originx: f64,
    pub originy: f64,
    pub format: String,
    //84参考系的包围盒
    pub wgs84BoundingBox: Rect<f64>,
    pub yAxis: YAxis,
}

impl Default for Tileschema {
    fn default() -> Tileschema {
        Tileschema {
            name: String::new(),
            srs: String::new(),
            //使用180 90作为其范围
            extent: Rect::new((-180.0f64, -90.0f64), (180.0f64, 90.0f64)),
            originx: std::f64::NAN,
            originy: std::f64::NAN,
            format: String::from("png"),
            wgs84BoundingBox: Rect::new((-180.0f64, -90.0f64), (180.0f64, 90.0f64)),
            yAxis: YAxis::TMS,
            resolutions: HashMap::new(),
        }
    }
}

//不使用trait 直接实现其基本方法
impl Tileschema {
    pub fn get_tile_width(&self, level_id: i32) -> i32 {
        self.resolutions.get(&level_id).unwrap().tileWidth
    }
    pub fn get_tile_height(&self, level_id: i32) -> i32 {
        self.resolutions.get(&level_id).unwrap().tileHeight
    }
    //算法改进需要
    pub fn get_tileinfos_by_level(
        &self,
        extent: &Rect<f64>,
        level_id: i32,
    ) -> Option<Vec<TileInfo>> {
        let range = world_to_tile(extent, level_id, self).unwrap();
        let mut tiles = Vec::<TileInfo>::new();
        //let minx=if range.co
        for i in range.first_col..(range.first_col + range.col_count) {
            for j in range.first_row..(range.first_row + range.row_count) {
                tiles.push(TileInfo {
                    extent: tile_to_world(&TileRange::new(i, j), level_id, self).unwrap(),
                    index: TileIndex {
                        col: i,
                        row: j,
                        level: level_id,
                    },
                });
            }
        }
        Some(tiles)
    }
    //返回指定级别下的瓦片信息,交出TileInfo的归属权给上一级使用
    //fn get_tileinfos_by_level(self, extent: &Rect<f64>, level_id: &str) -> [TileInfo] {}
}

pub trait ITileSchema {
    fn GetTileHeight(level_id: &str) -> i32;
    fn get_originx(level_id: &str) -> f64;
    fn get_originy(level_id: &str) -> f64;
    fn get_matrix_width(level_id: &str) -> i64;
    fn get_matrix_height(level_id: &str) -> i64;
    fn get_tileinfos_by_resolution(extent: &Rect<f64>, units_per_pixel: f64)
        -> &'static [TileInfo];
    fn get_extentoftiles_inview(extent: &Rect<f64>, level_id: &str) -> Rect<f64>;
}

// /// <summary>
// /// Function to get the first matrix column served by this schema for a given zoom level.
// /// </summary>
// /// <param name="levelId">The zoom level's id</param>
// int GetMatrixFirstCol(string levelId);

// /// <summary>
// /// Function to get the first matrix row served by this schema for a given zoom level.
// /// </summary>
// /// <param name="levelId">The zoom level's id</param>
// int GetMatrixFirstRow(string levelId);
