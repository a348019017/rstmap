extern crate geo_types;
use geo_types::{Point, Rect};

//数据提供者模式,通过范围查询指定的数据供给前端显示，一个通用基接口
pub trait Provider<T> {
    fn getbyextent(env: Rect<f64>) -> Vec<T>;
}
