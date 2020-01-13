use crate::tile::Attribution;
use crate::tile::TileProvider;
use crate::tile::Tileschema;

//相当于一个基类
#[derive(Debug, Clone)]
pub struct TileSource {
    //记录TileSource的Schema
    pub schema: Tileschema,
    //tilesource的名称
    pub name: String,
    //可选，schema必选
    pub attribution: Attribution,
}

//实现一个默认的TileSource，不带attribution

impl Default for TileSource {
    fn default() -> Self {
        TileSource {
            schema: Tileschema::default(),
            name: String::new(),
            attribution: Attribution::default(),
        }
    }
}

// impl TileSource {
//     fn new(schema:Tileschema,) -> TileSource {
//         TileSource {
//             schema:
//         }
//     }
// }
