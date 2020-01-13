extern crate geo;
use geo::intersects::Intersects;
use geo::{Coordinate, CoordinateType, Rect};
use std::cmp;

pub trait RectExtension<T: CoordinateType> {
    fn grow_in(&mut self, amount: T);
    fn grow(&self, amount: T) -> Self;
    //实现一个矩形求交,返回一个新的矩形对象
    fn intersection(&self, other: &Rect<T>) -> Option<Rect<T>>;
}

//基本泛型方法
impl<T: CoordinateType> RectExtension<T> for Rect<T> {
    fn grow_in(&mut self, amount: T) {
        self.max.x = self.max.x + amount;
        self.max.y = self.max.y + amount;
        self.min.x = self.min.x - amount;
        self.min.y = self.min.y - amount;
    }
    fn grow(&self, amount: T) -> Self {
        let mut clone: Rect<T> = self.clone();
        clone.max.x = self.max.x + amount;
        clone.max.y = self.max.y + amount;
        clone.min.x = self.min.x - amount;
        clone.min.y = self.min.y - amount;
        clone
    }
    fn intersection(&self, other: &Rect<T>) -> Option<Rect<T>> {
        //如果不相交则返回
        let minx = if self.min.x > other.min.x {
            self.min.x
        } else {
            other.min.x
        };
        let maxx = if self.max.x > other.max.x {
            other.max.x
        } else {
            self.max.x
        };
        let miny = if self.min.y > other.min.y {
            self.min.y
        } else {
            other.min.y
        };
        let maxy = if self.max.x > other.max.x {
            other.max.y
        } else {
            self.max.y
        };
        if maxx <= minx || maxy <= miny {
            None
        } else {
            Some(Rect::new((minx, miny), (maxx, maxy)))
        }
    }
}
