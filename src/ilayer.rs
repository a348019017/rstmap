extern crate geo_types;
use geo_types::{Point, Rect};

//刷新图层的指定区域的
trait ILayer {
    fn refresh(extent: Rect);
}
