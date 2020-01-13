use crate::tile::TileIndex;

//缓存的通用接口
pub trait TileCache<T> {
    fn add(&self, index: &TileIndex, tile: &T);
    /// Removes the tile that corresponds with the index passed as argument. When the tile is not found no exception is thrown.
    fn remove(&self, index: &TileIndex);
    /// Tries to find a tile that corresponds with the index. Returns null if not found.
    fn find(&self, index: &TileIndex) -> Option<T>;
}
