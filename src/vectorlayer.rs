extern crate geo_types;
use geo_types::{Point, Rect};

//瓦片图层对象
pub struct VectorLayer {
    name: String,
    aliasname: String,
    layertype: String,
}
