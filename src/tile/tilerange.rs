//describe a range tiles

#[derive(PartialEq, Debug)]
pub struct TileRange {
    pub first_col: i32,
    pub first_row: i32,
    pub col_count: i32,
    pub row_count: i32,
}

impl Default for TileRange {
    fn default() -> TileRange {
        TileRange {
            first_col: 0i32,
            first_row: 0i32,
            col_count: 1i32,
            row_count: 1i32,
        }
    }
}

impl TileRange {
    pub fn new(first_col: i32, first_row: i32) -> Self {
        TileRange {
            first_col: first_col,
            first_row: first_row,
            col_count: 1i32,
            row_count: 1i32,
        }
    }
}
