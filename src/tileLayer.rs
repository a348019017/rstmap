extern crate geo_types;
use geo_types::{Point, Rect};

//瓦片图层对象
pub struct TileLayer {
    name: String,
    aliasname: String,
    layertype: String,
}

//刷新指定区域的
trait ILayer {
    fn refresh(extent: Rect);
}
