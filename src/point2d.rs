use num::Float;
use std::ops::{Add, Div, Mul, MulAssign, Sub};

#[macro_export]
macro_rules! point {
    ($x: expr, $y: expr) => {
        Point2D::new($x, $y)
    };
}

macro_rules! op {
    ($opn: ident, $opln: ident, $op: tt) => {
        impl<T: Float> $opn<Point2D<T>> for Point2D<T> {
            type Output = Point2D<T>;

            #[inline(always)]
            fn $opln(self, v: Point2D<T>) -> Point2D<T> {
                Point2D {
                    x: self.x $op v.x,
                    y: self.y $op v.y,
                }
            }
        }
    };
}

macro_rules! op_num {
    ($opn: ident, $opln: ident, $op: tt) => {
        impl<T: Float> $opn<T> for Point2D<T> {
            type Output = Point2D<T>;

            #[inline(always)]
            fn $opln(self, v: T) -> Point2D<T> {
                Point2D {
                    x: self.x $op v,
                    y: self.y $op v,
                }
            }
        }
    };
}

#[derive(Debug, Clone, Copy)]
pub struct Point2D<T: Float> {
    pub x: T,
    pub y: T,
}

impl<T: Float> Point2D<T> {
    #[inline(always)]
    pub fn new(x: T, y: T) -> Point2D<T> {
        Point2D { x, y }
    }

    #[inline(always)]
    pub fn distance(p1: Point2D<T>, p2: Point2D<T>) -> T {
        let a = p1.x - p2.x;
        let b = p1.y - p2.y;
        (a * a + b * b).sqrt()
    }
}

// Point - Point
op!(Sub, sub, -);
// Point + Point
op!(Add, add, +);
// Point * Number
op_num!(Mul, mul, *);
// Point / Number
op_num!(Div, div, /);

// Point *= Float
impl<T: Float> MulAssign<T> for Point2D<T> {
    #[inline(always)]
    fn mul_assign(&mut self, v: T) {
        self.x = self.x * v;
        self.y = self.y * v;
    }
}
