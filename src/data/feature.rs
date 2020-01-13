//缺少一个feature对象的定义
use std::collections::HashMap;

//这里考虑使用JObject的包装
extern crate serde_json;
use serde_json::{Map, Number, Value};
extern crate geo_types;
use geo_types::Geometry;

pub struct Feature {
    //其实T大可不必，统一以f64存储
    Geometry: Geometry<f64>,
    //暂只使用一个dictionary存储，这里的Value采用的是JValue/JObject的包装类型
    Attributes: HashMap<String, Value>,
}
