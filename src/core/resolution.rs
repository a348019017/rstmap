//分辨率的结构体
use std::string::String;

#[derive(Debug, Clone)]
pub struct Resolution {
    pub id: String,
    //像素点对应的地图单位
    pub unitsPerPixel: f64,
    //当前的比例尺
    pub scaleDenominator: f64,
    top: f64,
    left: f64,
    pub tileWidth: i32,
    pub tileHeight: i32,
    //矩阵宽度
    pub matrixWidth: i64,
    pub matrixHeight: i64,
}

impl Resolution {
    //id所有权进来外面就不存在了，所有权到内部
    pub fn new(id: String, units_per_pixel: f64) -> Resolution {
        Resolution {
            id: id,
            unitsPerPixel: units_per_pixel,
            //其它值采用默认值
            ..Default::default()
        }
    }
}

impl Default for Resolution {
    fn default() -> Resolution {
        Resolution {
            id: String::new(),
            unitsPerPixel: 0.0f64,
            scaleDenominator: 0.0f64,
            top: 0.0f64,
            left: 0.0f64,
            tileWidth: 256,
            tileHeight: 256,
            matrixWidth: 0,
            matrixHeight: 0,
        }
    }
}
