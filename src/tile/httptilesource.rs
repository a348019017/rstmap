use crate::tile::Attribution;
use crate::tile::BasicRequest;
use crate::tile::TileIndex;
use crate::tile::TileInfo;
use crate::tile::TileProvider;
use crate::tile::TileRange;
use crate::tile::TileSource;
use crate::tile::Tileschema;
use crate::Request;

extern crate reqwest;

//继承TileSource的实现，多余的参与均记录在request中，如url
pub struct HttpTileSource {
    pub base: TileSource,
    //用于请求的对象,Reuqest也是编译时大小未知，可以使用Box，也可以指定生命周期
    pub request: Box<dyn Request>,
    //useragent可以记录在request中，这里为求方便就记录在httpTileSource中
    pub useragent: String,
}

// //如何实现结构体的集成使用deref，实现继承
// impl Deref for HttpTileSource {
//     type Target = TileSource;
//     fn deref(&self) -> &TileSource {
//         &self.base
//     }
// }

impl HttpTileSource {
    //一个request只能被一个tilesource使用,not share schema too.
    fn new(schema: &Tileschema, request: Box<dyn Request>) -> HttpTileSource {
        HttpTileSource {
            base: TileSource {
                schema: schema.clone(),
                ..TileSource::default()
            },
            request: request,
            useragent: String::new(),
        }
    }
    //带个名称
    fn with_name(mut self, name: &str) -> Self {
        self.base.name = name.to_string();
        self
    }
    //带个标注
    fn with_attribute(mut self, attributes: Attribution) -> Self {
        //这里既希望保证安全，也希望复用attribute，这很不安全，还是采用获取所有权的方式,外部可以使用clone
        self.base.attribution = attributes;
        self
    }

    //返回指定的url规则
    //fn get_url(&self, tile_info: TileIndex) -> String {}
}

//实现TileProvider方法,返回二进制对象,暂未使用agend
impl<'a> TileProvider<Vec<u8>> for HttpTileSource {
    fn get_tile(&self, tile_info: &TileInfo) -> Vec<u8> {
        let mut buf: Vec<u8> = vec![];
        let url = self.request.get_url(tile_info);
        let cc = Some(1);
        reqwest::blocking::get(&url).unwrap().copy_to(&mut buf);
        buf
    }
    //实现一个通过制定范围查询tiles的办法
    // fn get_tile_byrange(&self, range: TileRange) {

    // }
}

//编写一个测试用例，获取指定范围的瓦片
#[cfg(test)]
mod httptilesource {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    extern crate geo_types;
    use crate::tile::predefined::GlobalSphericalMercator;
    use crate::tile::BasicRequest;
    use crate::tile::HttpTileSource;
    use crate::tile::*;
    use geo_types::Rect;
    use std::sync::mpsc::channel;
    use std::sync::Arc;
    use std::thread;
    use std::time::{Duration, Instant};

    #[test]
    fn test_async_tile_fetcher() {
        // arrange
        let tileschema = GlobalSphericalMercator::new()
            .withname("testscheam")
            .withmin(0)
            .withmax(18)
            .withyaxis(YAxis::OSM)
            .build();

        let range = tileschema
            .get_tileinfos_by_level(&tileschema.extent, 3i32)
            .unwrap();
        let start = Instant::now();
        // // act
        let request = Box::new(BasicRequest::new(
            "http://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png",
            &vec!["a".to_string(), "b".to_string(), "c".to_string()],
            "",
        ));
        // 使用arc，引用计数来多线程使用
        let tileSource = Arc::new(HttpTileSource::new(&tileschema, request));
        assert_eq!(range.len(), 64);
        let (sender, receiver) = channel();

        let testresult: Vec<_> = range
            .into_iter()
            .map(|i| {
                let request_clone = tileSource.clone();
                let sender_n = sender.clone();
                thread::spawn(move || {
                    let tiles = request_clone.get_tile(&i);
                    let result = (i, tiles.len());
                    sender_n.send(result);
                })
            })
            .collect();
        for i in testresult {
            i.join();
            //println!("index{:?},len:{:?}", result.0, result.1);
            //assert_ne!(result.1, 0);
        }

        // for i in range {
        //     let request_clone = tileSource.clone();
        //     let sender_n = sender.clone();
        //     thread::spawn(move || {
        //         let tiles = request_clone.get_tile(&i);
        //         let result = (i, tiles.len());
        //         sender_n.send(result);
        //     });
        // }
        // for i in 0..64 {
        //     let result = receiver.recv().unwrap();
        //     //println!("index{:?},len:{:?}", result.0, result.1);
        //     assert_ne!(result.1, 0);
        // }
        //多线程请求64个瓦片约为1s
        let lasttime = start.elapsed().as_secs();
        println!("last time {:?} s", lasttime);
    }
}
