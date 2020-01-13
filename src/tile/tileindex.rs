use std::cmp::Ordering;

//tile的索引，需要实现Compare以支持排序
#[derive(Debug)]
pub struct TileIndex {
    pub col: i32,
    pub row: i32,
    pub level: i32,
}

impl TileIndex {
    fn new(col: i32, row: i32, level: i32) -> TileIndex {
        TileIndex {
            col: col,
            row: row,
            level: level,
        }
    }
}

//实现其eq方式
impl PartialEq for TileIndex {
    fn eq(&self, other: &Self) -> bool {
        self.col == other.col && self.row == other.row && self.level == other.level
    }
}
// //实现其排序

// impl Ord for TileIndex {
//     fn cmp(&self, other: &TileIndex) -> Ordering {
//         //先比较列，再比较行，最后比较其level，这里暂不实现
//         // if (Col < index.Col) return -1;
//         //     if (Col > index.Col) return 1;
//         //     if (Row < index.Row) return -1;
//         //     if (Row > index.Row) return 1;
//         //     return String.Compare(Level, index.Level, StringComparison.Ordinal);
//         Ordering::Greater
//     }
// }
