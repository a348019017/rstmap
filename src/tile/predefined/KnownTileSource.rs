//预定义的枚举
pub enum KnownTileSource {
    OpenStreetMap(OpenStreetMapTileSource),
}

//OpenStreet的构造函数
pub struct OpenStreetMapTileSource {}

impl OpenStreetMapTileSource {
    fn new() {
        //使用TileSource的构造函数，构造这个特殊的对象,返回一个Tile
    }
}

// impl KnownTileSource {
//     new()
// }
