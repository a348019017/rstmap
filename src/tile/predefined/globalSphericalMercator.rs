//实现一个EPSG3857的Schema
use crate::tile::{Tileschema, YAxis};
use crate::Resolution;
use std::collections::HashMap;
use std::ops::Deref;
extern crate geo_types;
use geo_types::Rect;

//其实可以不必包含schema因为本身这只是标致一个类型，返回的仍然是TileSchema，TileSchema足够使用
pub struct GlobalSphericalMercator {
    //内部包含一个schemaparams，可以使用解引用实现继承
    pub schema_params: TileschemaParams,
}

// //实现一个解引用
// impl Deref for GlobalSphericalMercator {
//     type Target = Tileschema;
//     fn deref(&self) -> &Tileschema {
//         &self.schema
//     }
// }

//构造的参数，不支持levels的参数传法
pub struct TileschemaParams {
    pub min: u32,
    pub max: u32,
    //pub extent: Rect<f64>,
    pub name: String,
    pub format: String,
    pub yAxis: YAxis,
}

//其它使用默认参数
impl GlobalSphericalMercator {
    //参考系的默认常量，用于构造函数的使用
    const SCALEFACTOR: f64 = 78271.51696401953125;
    //TMS可以指定Grid/Extent 这里先不强制指定
    const DEFAULTFNAME: &'static str = "GlobalSphericalMercator";
    const DEFAULTFORMAT: &'static str = "png";
    const DEFAULTMINZOOMLEVEL: u32 = 0;
    const DEFAULTMAXZOOMLEVEL: u32 = 19;
    const TILESIZE: u32 = 256;

    //根据最低和最高级别创建分辨率
    fn to_resolutions(min: u32, max: u32) -> HashMap<i32, Resolution> {
        // //if (levels == null) return to_resolutions(DefaultMinZoomLevel, DefaultMaxZoomLevel);
        let mut resolutions = HashMap::new();
        for i in min..=max {
            //这种按位计算的方式在转换成f64的时候可能会出现问题
            let res = 2.0 * GlobalSphericalMercator::SCALEFACTOR / (1i32 << i) as f64;
            resolutions.insert(i as i32, Resolution::new(i.to_string(), res));
        }
        resolutions
    }
    pub fn withmin(mut self, min: u32) -> Self {
        self.schema_params.min = min;
        self
    }
    pub fn withmax(mut self, max: u32) -> Self {
        self.schema_params.max = max;
        self
    }
    //所有权提交，意味着只能链式，效率应该比创建借用更高
    pub fn withname(mut self, name: &str) -> Self {
        self.schema_params.name = name.to_string();
        self
    }
    pub fn withformat(mut self, format: &str) -> Self {
        self.schema_params.format = format.to_string();
        self
    }
    pub fn withyaxis(mut self, yAxis: YAxis) -> Self {
        self.schema_params.yAxis = yAxis;
        self
    }
    pub fn build(self) -> Tileschema {
        let originx =
            -GlobalSphericalMercator::SCALEFACTOR * GlobalSphericalMercator::TILESIZE as f64;
        let mut originy =
            -GlobalSphericalMercator::SCALEFACTOR * GlobalSphericalMercator::TILESIZE as f64;
        let extent = Rect::new((originx, originy), (-originx, -originy));
        if (self.schema_params.yAxis == YAxis::OSM) {
            originy = -originy;
        }
        let resolutions =//这里为什么不能用self.to_resolutions(min: u32, max: u32)
            GlobalSphericalMercator::to_resolutions(self.schema_params.min, self.schema_params.max);
        let schema = Tileschema {
            name: self.schema_params.name.clone(),
            format: self.schema_params.format.clone(),
            srs: "EPSG:3857".to_string(),
            originx: originx,
            originy: originy,
            extent: extent,
            resolutions: resolutions,
            yAxis: self.schema_params.yAxis,
            ..Default::default()
        };
        schema
    }
    //rust can use builder mode to create a object flexible,bu with new what you can do is just like this,
    //so the best practise is builder mode
    pub fn new() -> GlobalSphericalMercator {
        GlobalSphericalMercator {
            schema_params: TileschemaParams {
                name: GlobalSphericalMercator::DEFAULTFNAME.to_string(),
                min: GlobalSphericalMercator::DEFAULTMINZOOMLEVEL,
                max: GlobalSphericalMercator::DEFAULTMAXZOOMLEVEL,
                format: GlobalSphericalMercator::DEFAULTFORMAT.to_string(),
                yAxis: YAxis::OSM,
            },
        }
    }
}

//写个单元测试，测试builder的情况，console中显示参数详细值
#[cfg(test)]
mod GlobalSphericalMercatortests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn testGlobalSpherical() {
        let schema = GlobalSphericalMercator::new()
            .withmin(0u32)
            .withmax(19)
            .withname("test")
            .withformat("jpg")
            .withyaxis(YAxis::TMS)
            .build();

        assert_eq!(schema.name, "test".to_string());
        assert_eq!(schema.format, "jpg".to_string());
        assert_eq!(schema.yAxis, YAxis::TMS);
        //验证创建的范围是否正确

        assert_eq!(
            schema.extent,
            Rect::new(
                (-20037508.342789f64, -20037508.342789f64),
                (20037508.342789f64, 20037508.342789f64)
            )
        );
        //验证级别是否正确
        assert_eq!(schema.resolutions.len(), 20);
        //验证每个级别的是否正确
        for i in 0..19 {
            let rs = 20037508.342789f64 / (256.0f64 * (1i32 << i) as f64) * 2.0f64;
            assert_eq!(schema.resolutions.get(&i).unwrap().unitsPerPixel, rs);
        }

        //assert_eq!(sc)
        println!("resoulutionis:{:?}", schema.resolutions);
        // This assert would fire and test will fail.
        // Please note, that private functions can be tested too!
        // assert_eq!(bad_add(1, 2), 3);
    }

    #[test]
    fn InitializeAsBingSchema() {
        // act
        let schema = GlobalSphericalMercator::new()
            .withmin(1)
            .withmax(19)
            .withname("Bingmaps")
            .withformat("jpg")
            .withyaxis(YAxis::OSM)
            .build();

        assert!(schema.resolutions.len() == 19);
        assert!(schema.resolutions.values().all(|i| i.id != "0"));
        assert!(schema.resolutions.values().any(|r| r.id == "1"));
        assert!(schema.resolutions.get(&3i32).unwrap().id == "3");
        assert!(schema.name == "Bingmaps");
        assert!(schema.format == "jpg");
    }
}
