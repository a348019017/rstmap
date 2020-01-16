extern crate geo_types;
use geo_types::{Coordinate, Point, Rect};

pub struct Map {
    //获取当前地图的范围
    envelope: Rect<f64>,
    //当前地图预定义的分辨率
    resolutions: Vec<f64>,
    curresolution: f64,
    //以Coordinate作为点位的存储
    center: Coordinate<f64>,
    //当前map的实际显示器分辨率大小，窗体大小的改变触发内部的刷新
    height: u32,
    width: u32,
}

impl Map {
    //刷新指定区域的
    //fn refresh(extent: Rect<f64>);

    //get current envelope at current resolution and center
    fn get_envelope(&self) -> Rect<f64> {
        let minx = -self.curresolution * (self.width / 2u32) as f64 + self.center.x;
        let miny = -self.curresolution * (self.height / 2u32) as f64 + self.center.y;
        let maxx = self.curresolution * (self.width / 2u32) as f64 + self.center.x;
        let maxy = self.curresolution * (self.height / 2u32) as f64 + self.center.y;
        Rect::new((minx, miny), (maxx, maxy))
    }
}

// //初始化构造一个map对象
// impl Map {
//     fn new() {}
// }
