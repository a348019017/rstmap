extern crate geo_types;
use geo_types::{Point, Rect};

pub struct Map {
    //获取当前地图的范围
    envelope: Rect<f64>,
    //当前地图预定义的分辨率
    resolutions: Vec<f64>,
    //当前map的实际显示器分辨率大小，窗体大小的改变触发内部的刷新
    height: u32,
    width: u32,
}

//刷新指定区域的
trait IMap {
    fn refresh(extent: Rect<f64>);
}

//初始化构造一个map对象
impl Map {
    fn new() {}
}
