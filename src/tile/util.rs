extern crate geo;
extern crate geo_types;
use crate::core::RectExtension;
use crate::core::Resolution;
use crate::tile::TileRange;
use crate::tile::Tileschema;
use crate::tile::YAxis;
use geo::intersects::Intersects;
use geo_types::Rect;
use std::collections::HashMap;

const TOLERANCE: f64 = 0.000000001f64;

//算法需要理清楚
pub fn world_to_tile(extent: &Rect<f64>, level_id: i32, schema: &Tileschema) -> Option<TileRange> {
    if let Some(newextent) = extent.intersection(&schema.extent) {
        let resolution: &Resolution = schema.resolutions.get(&level_id)?;
        let tile_world_units = resolution.unitsPerPixel * schema.get_tile_width(level_id) as f64;
        let first_col =
            ((newextent.min.x - schema.originx) / tile_world_units + TOLERANCE).floor() as i32;
        let first_row = if schema.yAxis == YAxis::TMS {
            ((newextent.min.y - schema.originy) / tile_world_units + TOLERANCE).floor() as i32
        } else {
            ((-newextent.max.y + schema.originy) / tile_world_units + TOLERANCE).floor() as i32
        };
        let last_col =
            ((newextent.max.x - schema.originx) / tile_world_units - TOLERANCE).ceil() as i32;
        let last_row = if schema.yAxis == YAxis::TMS {
            ((newextent.max.y - schema.originy) / tile_world_units - TOLERANCE).ceil() as i32
        } else {
            ((-newextent.min.y + schema.originy) / tile_world_units - TOLERANCE).ceil() as i32
        };
        Some(TileRange {
            first_col: first_col,
            first_row: first_row,
            col_count: (last_col - first_col).abs(),
            row_count: (last_row - first_row).abs(),
        })
    } else {
        None
    }
}
//range to worldcoord
pub fn tile_to_world(range: &TileRange, level_id: i32, schema: &Tileschema) -> Option<Rect<f64>> {
    let resolution: &Resolution = schema.resolutions.get(&level_id)?;
    let tile_world_units = resolution.unitsPerPixel * schema.get_tile_width(level_id) as f64;
    let minx = range.first_col as f64 * tile_world_units + schema.originx;
    let miny = if schema.yAxis == YAxis::TMS {
        range.first_row as f64 * tile_world_units + schema.originy
    } else {
        schema.originy - (range.first_row + range.row_count) as f64 * tile_world_units
    };
    let maxx = (range.first_col + range.col_count) as f64 * tile_world_units + schema.originx;
    let maxy = if schema.yAxis == YAxis::TMS {
        (range.first_row + range.row_count) as f64 * tile_world_units + schema.originy
    } else {
        schema.originy - range.first_row as f64 * tile_world_units
    };
    Some(Rect::new((minx, miny), (maxx, maxy)))
}

//get nearest resoultion level
pub fn get_nearest_level(
    resolutions: &HashMap<i32, Resolution>,
    unitsPerPixel: f64,
) -> Option<i32> {
    if resolutions.len() == 0 {
        return None;
    }
    let (maxlevel, minresolution) = resolutions.iter().max_by_key(|(key, value)| *key)?;
    if unitsPerPixel <= minresolution.unitsPerPixel {
        return Some(*maxlevel);
    }
    let (minlevel, maxresoulution) = resolutions.iter().min_by_key(|(key, value)| *key)?;
    if unitsPerPixel >= maxresoulution.unitsPerPixel {
        return Some(*minlevel);
    }
    let mut result_distance: f64 = std::f64::MAX;
    let mut result: Option<i32> = None;
    //居然不能比较浮点型
    for (level, resoultion) in resolutions.iter() {
        let distance: f64 = (resoultion.unitsPerPixel - unitsPerPixel).abs();
        if distance < result_distance {
            result = Some(*level);
            result_distance = distance;
        }
    }
    result
}

#[cfg(test)]
mod test {
    use crate::tile::util::get_nearest_level;
    use crate::tile::GlobalSphericalMercator;
    use crate::tile::*;
    const Epsilon: f64 = 0.000001;
    extern crate geo_types;
    use crate::core::RectExtension;
    use geo_types::Rect;

    #[test]
    pub fn test_get_nearest_level() {
        // arrange
        let schema = GlobalSphericalMercator::new().build();
        // act
        let level_id = get_nearest_level(&schema.resolutions, 300.0).unwrap();
        // assert
        assert_eq!(level_id, 9)
    }

    #[test]
    fn tile_to_world_should_return_correct_extent() {
        // arrange
        let range = TileRange::new(1, 2);
        let schema = GlobalSphericalMercator::new().withyaxis(YAxis::TMS).build();
        let expectedExtent = Rect::new(
            (-15028131.257989, -10018754.173189),
            (-10018754.173189, -5009377.088389),
        );
        const toleratedDelta: f64 = 0.01;

        // act
        let extent = tile_to_world(&range, 3, &schema).unwrap();

        //println!("{:?}", extent);

        // 浮点的精度比较可以尝试使用float-cmp库，这里暂不深入
        assert_eq!(
            (extent.min.x - expectedExtent.min.x).abs() <= toleratedDelta,
            true
        );
        assert_eq!(
            (extent.min.y - expectedExtent.min.y).abs() <= toleratedDelta,
            true,
        );
        assert_eq!(
            (extent.max.x - expectedExtent.max.x).abs() <= toleratedDelta,
            true
        );
        assert_eq!(
            (extent.max.y - expectedExtent.max.y).abs() <= toleratedDelta,
            true
        );
    }

    #[test]
    fn WorldToTileShouldReturnCorrectTileRange() {
        // arrange
        let expectedRange = TileRange::new(1, 2);
        let schema = GlobalSphericalMercator::new().withyaxis(YAxis::TMS).build();
        let extent = Rect::new((-15028130f64, -10018753f64), (-10018755f64, -5009378f64));

        // act
        let range = world_to_tile(&extent, 3, &schema).unwrap();

        // assert
        assert_eq!(range, expectedRange);
    }
}
