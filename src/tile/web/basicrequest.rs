use crate::tile::TileInfo;
use crate::Request;
use crate::TileIndex;
use std::sync::atomic::AtomicI8;
use std::thread;

//定义在外部,这里还是得使用原子操作进行+1
//Atomic types and operations are not guaranteed to be wait-free
//使用原子类型并不能保证
static mut NODECOUNTER: usize = 0;

#[derive(Clone)]
pub struct BasicRequest {
    pub url_formatter: String,
    //内部私有变量，用于切换多组服务器,当在外部调用的时候此参数无效
    nodeCounter: usize,
    pub serverNodes: Vec<String>,
    pub apikey: String,
}

impl BasicRequest {
    //常量的定义均在impl里面,因为是静态的所以一直会存在内存中，这好像不合理
    const QUADKEYTAG: &'static str = "{quadkey}";
    const XTAG: &'static str = "{x}";
    const YTAG: &'static str = "{y}";
    const ZTAG: &'static str = "{z}";
    const SERVERNODETAG: &'static str = "{s}";
    const APIKEYTAG: &'static str = "{k}";

    //使用借用来创建对象，因为可能需要创建多个BasicRequest来多线程调用，或者说多个线程去调用request
    pub fn new(url_formatter: &str, serverNodes: &Vec<String>, apikey: &str) -> BasicRequest {
        BasicRequest {
            url_formatter: url_formatter.to_string(),
            //这里使用clone来初始化构造
            serverNodes: serverNodes.clone(),
            apikey: apikey.to_string(),
            nodeCounter: 0usize,
        }
    }

    //返回下一个servernode的借用,这里没有考虑到线程安全和锁，因为也是纯随机的，暂时不考虑那么多
    fn get_next_servernode(&self) -> &str {
        //线程不安全的写法，不过问题也不大
        //let mut counter = NODECOUNTER;
        unsafe {
            let index = NODECOUNTER % self.serverNodes.len();
            let node = self.serverNodes.get(index).unwrap();
            NODECOUNTER += 1;
            &node
        }
    }

    //插入server的Tag
    // fn insert_server_node(&self, baseUrl: &String, serverNodes: &Vec<String>) {
    //     if (serverNodes.len() > 0usize) {
    //         let serverNode = self.get_next_servernode();
    //         baseUrl.replace(BasicRequest::SERVERNODETAG, serverNode);
    //     }
    // }
}

impl Request for BasicRequest {
    //如果在内部实现多线程，则需要使用async await的工作模式，这里get_url开销很小，不需要多线程，完整的gettile可以使用线程封装。
    //这里最好在内部能衔接外部的多线程调用
    fn get_url(&self, info: &TileInfo) -> String {
        //这种替换方式效率较低，尝试使用format,这里还不能使用runtime format
        //need help to improve performance
        let mut string_builder = self
            .url_formatter
            .replace(BasicRequest::XTAG, &info.index.col.to_string());
        string_builder = string_builder.replace(BasicRequest::YTAG, &info.index.row.to_string());
        string_builder = string_builder.replace(BasicRequest::ZTAG, &info.index.level.to_string());
        //unkown quadkey
        // stringBuilder.replace(
        //     BasicRequest::QUADKEYTAG,
        //     TileXyToQuadKey(info.Index.Col, info.Index.Row, info.Index.Level),
        // );
        if (self.serverNodes.len() > 0usize) {
            let server_node = self.get_next_servernode();
            string_builder = string_builder.replace(BasicRequest::SERVERNODETAG, server_node);
        }
        string_builder
    }
}

//编写一个测试用例，读取到测试url是否正确
//写个单元测试，测试builder的情况，console中显示参数详细值
#[cfg(test)]
mod basicrequest {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    extern crate geo_types;
    use geo_types::Rect;
    use std::sync::mpsc::channel;
    use std::thread;
    #[test]
    fn get_uri_test() {
        let request = BasicRequest::new(
            "http://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png",
            &vec!["a".to_string(), "b".to_string(), "c".to_string()],
            "",
        );
        let tileindex = TileIndex {
            col: 3,
            row: 4,
            level: 5,
        };
        let tileInfo = TileInfo {
            index: tileindex,
            extent: Rect::new((0.0f64, 0.0f64), (0.0f64, 0.0f64)),
        };

        // act
        let url = request.get_url(&tileInfo);

        // assert
        assert_eq!(url, "http://a.tile.openstreetmap.org/5/3/4.png");
    }

    #[test]
    fn get_url_in_parallel_test() {
        // arrange
        let request = BasicRequest::new(
            "http://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png",
            &vec!["a".to_string(), "b".to_string(), "c".to_string()],
            "",
        );
        // let tileindex = TileIndex {
        //     col: 3,
        //     row: 4,
        //     level: "5".to_string(),
        // };
        // let tileInfo = TileInfo {
        //     index: tileindex,
        //     extent: Rect::new((0.0f64, 0.0f64), (0.0f64, 0.0f64)),
        // };

        let (sender, receiver) = channel();

        //测试多线程,创建100个线程,从外部调用需要Copy Request，内部调用需要交控制权。BasicRequeset本身可能需要实现Copy
        for i in 0..100 {
            let request_clone = request.clone();
            let sender_n = sender.clone();
            thread::spawn(move || {
                let tileindex = TileIndex {
                    col: 3,
                    row: 4,
                    level: 5,
                };
                let tileInfo = TileInfo {
                    index: tileindex,
                    extent: Rect::new((0.0f64, 0.0f64), (0.0f64, 0.0f64)),
                };
                let url = request_clone.get_url(&tileInfo);
                sender_n.send(url);
            });
        }

        let mut result = Vec::new();
        for i in 0..100 {
            result.push(receiver.recv().unwrap());
        }
        println!("{:?}", result);
        // assert
        assert_eq!(
            result
                .iter()
                .any(|e| e == "http://a.tile.openstreetmap.org/5/3/4.png"),
            true
        );
        assert_eq!(
            result
                .iter()
                .any(|e| e == "http://b.tile.openstreetmap.org/5/3/4.png"),
            true
        );
        assert_eq!(
            result
                .iter()
                .any(|e| e == "http://c.tile.openstreetmap.org/5/3/4.png"),
            true
        );
    }
}
