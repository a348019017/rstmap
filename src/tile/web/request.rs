use crate::TileInfo;
//use url::Url;
//定义一个通用接口返回指定url
pub trait Request: Sync + Send {
    fn get_url(&self, info: &TileInfo) -> String;
}
