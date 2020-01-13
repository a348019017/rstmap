extern crate geo_types;
use crate::tile::TileInfo;
use geo_types::{Point, Rect};

//一个瓦片的数据提供接口，需要向下支持getbyextent,T是我们要返回的值，大小未知
pub trait TileProvider<T: ?Sized> {
    //返回一个二进制流，这里暂时使用[u8]或者vec<u8>代替,考虑到Cache的情况，需要返回bytes的借用,这里还需要标志其为静态的借用，保持其生命周期
    //internal contain cache return &T,not contain cache return T
    fn get_tile(&self, tile_info: &TileInfo) -> T;
}
