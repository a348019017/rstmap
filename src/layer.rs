extern crate geo_types;
use geo_types::{Point, Rect};

//图层结构的基本对象
pub struct Layer {
    name: String,
    aliasname: String,
    layertype: String,
}

//刷新指定区域的
trait ILayer {
    fn refresh(extent: Rect);
}
