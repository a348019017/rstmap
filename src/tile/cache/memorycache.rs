use std::collections::HashMap;

//memoryCache需要支持多线程的话
pub struct MemoryCache<T> {
    pub bitmaps: HashMap<TileIndex, T>,
    //最少最大瓦片
    pub min_tiles: i32,
    pub max_tiles: i32,
}
