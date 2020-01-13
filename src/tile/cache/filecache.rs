//实现一个带锁的支持多线程的读写的tilecache
use crate::tile::TileCache;
use crate::TileIndex;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::time::{Duration, Instant, SystemTime};
pub struct FileCache {
    //default max cache size unsupport
    pub maxsize: u32,
    //记录目录，默认为空
    pub directory: String,
    pub format: String,
    //指定超时时间，以秒为单位
    pub cache_expire_time: u64,
}

impl Default for FileCache {
    fn default() -> Self {
        FileCache {
            directory: String::from("./tilecache"),
            format: String::from("png"),
            //default update expire，默认更新周期为u64上限值
            cache_expire_time: std::u64::MAX,
            maxsize: 1024u32,
        }
    }
}

impl FileCache {
    //使用默认值初始化
    fn new() -> Self {
        FileCache::default()
    }
    fn get_filename(&self, index: &TileIndex) -> String {
        //{z}/{x}/{y} 符合tms的模式进行存储
        return format!(
            "{}/{}/{}.{}",
            index.level, index.col, index.row, self.format
        );
    }
    // fn get_datetime_u64(utc: u64) -> DateTime {
    //     //let local: DateTime<Local> = Local::now();
    // }
    fn with_cache_expire_time(mut self, sec: u64) -> Self {
        self.cache_expire_time = sec;
        self
    }
    fn exists(&self, index: &TileIndex) -> bool {
        let filename = self.get_filename(index);
        if (Path::new(&filename).exists()) {
            let metadata =
                fs::metadata(&filename).expect("get file metadata err when tile file cache");
            //取得最后一次修改的时间
            let last_write_time = metadata
                .modified()
                .expect("local system cant get modified datetime");
            // Create DateTime from SystemTime
            let datetime = SystemTime::now();
            //如果超时的时间以秒为单位
            let duration = datetime
                .duration_since(last_write_time)
                .expect("duration since error when file cache")
                .as_secs();
            return self.cache_expire_time == 0 || duration <= self.cache_expire_time;
        }
        return false;
    }
    //将对象写入到文件中
    fn write_to_file(&self, image: &[u8], index: &TileIndex) -> bool {
        //构造一个新的路径
        let str_path = self.get_filename(index);
        let mut f = File::create(&str_path).unwrap();
        f.write_all(image)
            .expect_err("create error when file cache tile");
        f.flush().is_ok()
    }
}
//这里的文件缓存使用heap进行收集
impl TileCache<Vec<u8>> for FileCache {
    //将二进制对象写入，这里的format实际上来源于Schema,不作额外的编码工作
    fn add(&self, index: &TileIndex, tile: &Vec<u8>) {
        if (self.exists(index)) {
            return; // ignore
        }
        //这里有个创建目录的过程
        // string dir = GetDirectoryName(index);
        // if (!Directory.Exists(dir))
        // {
        //     Directory.CreateDirectory(dir);
        // }
        self.write_to_file(tile.as_ref(), index);
    }
    /// Removes the tile that corresponds with the index passed as argument. When the tile is not found no exception is thrown.
    fn remove(&self, index: &TileIndex) {}
    /// Tries to find a tile that corresponds with the index. Returns null if not found.
    fn find(&self, index: &TileIndex) -> Option<Vec<u8>> {
        if (!self.exists(index)) {
            return None;
        } // to indicate not found
        let mut buf = Vec::new();
        File::open(self.get_filename(index))
            .unwrap()
            .read_to_end(&mut buf);
        Some(buf)
    }
}
