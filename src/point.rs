use num::integer::Roots;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

#[macro_export]
macro_rules! point {
    ($x: expr, $y: expr) => {
        Vector2::new($x, $y)
    };
}

macro_rules! op_assign_t_and_t {
    ($type: ident, $class: ident, $fn: ident, $op: tt) => {
        impl $class<Point2<$type>> for Point2<$type> {
            #[inline(always)]
            fn $fn(&mut self, v: Point2<$type>) {
                self.x = self.x $op v.x;
                self.y = self.y $op v.y;
            }
        }
    };
}

macro_rules! op_assign_t_and_num {
    ($type: ident, $class: ident, $fn: ident, $op: tt) => {
        impl $class<$type> for Point2<$type> {
            #[inline(always)]
            fn $fn(&mut self, v: $type) {
                self.x = self.x $op v;
                self.y = self.y $op v;
            }
        }
    };
}

macro_rules! op_assign_t_and_tuple {
    ($type: ident, $class: ident, $fn: ident, $op: tt) => {
        impl $class<($type, $type)> for Point2<$type> {
            #[inline(always)]
            fn $fn(&mut self, v: ($type, $type)) {
                self.x = self.x $op v.0;
                self.y = self.y $op v.1;
            }
        }
    };
}

macro_rules! op_t_and_t {
    ($type: ident, $class: ident, $fn: ident, $op: tt) => {
        impl $class<Point2<$type>> for Point2<$type> {
            type Output = Point2<$type>;

            #[inline(always)]
            fn $fn(self, v: Point2<$type>) -> Point2<$type> {
                Point2 {
                    x: self.x $op v.x,
                    y: self.y $op v.y,
                }
            }
        }
    };
}

macro_rules! op_t_and_num {
    ($type: ident, $class: ident, $fn: ident, $op: tt) => {
        impl $class<$type> for Point2<$type> {
            type Output = Point2<$type>;

            #[inline(always)]
            fn $fn(self, v: $type) -> Point2<$type> {
                Point2 {
                    x: self.x $op v,
                    y: self.y $op v,
                }
            }
        }
    };
}

macro_rules! op_t_and_tuple {
    ($type: ident, $class: ident, $fn: ident, $op: tt) => {
        impl $class<($type, $type)> for Point2<$type> {
            type Output = Point2<$type>;

            #[inline(always)]
            fn $fn(self, v: ($type, $type)) -> Point2<$type> {
                Point2 {
                    x: self.x $op v.0,
                    y: self.y $op v.1,
                }
            }
        }
    };
}

macro_rules! define_struct {
    ($type: ident) => {
        impl Point2<$type> {
            #[inline(always)]
            pub fn new(x: $type, y: $type) -> Point2<$type> {
                Point2 { x, y }
            }

            #[inline(always)]
            pub fn distance(p1: Point2<$type>, p2: Point2<$type>) -> $type {
                let a = p1.x - p2.x;
                let b = p1.y - p2.y;
                (a * a + b * b).sqrt()
            }
        }

        op_t_and_t!($type, Add, add, +);
        op_t_and_t!($type, Sub, sub, -);
        op_t_and_tuple!($type, Add, add, +);
        op_t_and_tuple!($type, Sub, sub, -);
        op_t_and_num!($type, Mul, mul, *);
        op_t_and_num!($type, Div, div, /);

        op_assign_t_and_t!($type, AddAssign, add_assign, +);
        op_assign_t_and_t!($type, SubAssign, sub_assign, -);
        op_assign_t_and_tuple!($type, AddAssign, add_assign, +);
        op_assign_t_and_tuple!($type, SubAssign, sub_assign, -);
        op_assign_t_and_num!($type, MulAssign, mul_assign, *);
        op_assign_t_and_num!($type, DivAssign, div_assign, /);
    };
}

#[derive(Debug, Clone, Copy)]
pub struct Point2<T> {
    pub x: T,
    pub y: T,
}

define_struct!(i8);
define_struct!(i16);
define_struct!(i32);
define_struct!(i64);
define_struct!(i128);
define_struct!(f32);
define_struct!(f64);

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_add {
        ($type: ident) => {
            paste::item! {
                /// Add Point<T> + Point<T>
                #[test]
                fn [<add_ $type _and_ $type>]() {
                    let p1 = Point2::<$type>::new(1 as $type, 1 as $type);
                    let p2 = Point2::<$type>::new(2 as $type, -2 as $type);
                    let p3 = p1 + p2;

                    assert_eq!(p3.x, 3 as $type);
                    assert_eq!(p3.y, -1 as $type);
                }

                /// Add Point<T> + (T, T)
                #[test]
                fn [<add_ $type _and_tuple>]() {
                    let p1 = Point2::<$type>::new(1 as $type, 1 as $type);
                    let p2 = (2 as $type, -2 as $type);
                    let p3 = p1 + p2;

                    assert_eq!(p3.x, 3 as $type);
                    assert_eq!(p3.y, -1 as $type);
                }

                /// Add Point<T> += Point<T>
                #[test]
                fn [<add_assign_ $type _and_ $type>]() {
                    let mut p1 = Point2::<$type>::new(1 as $type, 1 as $type);
                    let p2 = Point2::<$type>::new(2 as $type, -2 as $type);
                    p1 += p2;

                    assert_eq!(p1.x, 3 as $type);
                    assert_eq!(p1.y, -1 as $type);
                }

                /// Add Point<T> += (T, T)
                #[test]
                fn [<add_assign_ $type _and_tuple>]() {
                    let mut p1 = Point2::<$type>::new(1 as $type, 1 as $type);
                    p1 += (2 as $type, -2 as $type);

                    assert_eq!(p1.x, 3 as $type);
                    assert_eq!(p1.y, -1 as $type);
                }
            }
        };
    }

    macro_rules! test_sub {
        ($type: ident) => {
            paste::item! {
                /// Sub Point<T> - Point<T>
                #[test]
                fn [<sub_ $type _and_ $type>]() {
                    let p1 = Point2::<$type>::new(1 as $type, 1 as $type);
                    let p2 = Point2::<$type>::new(2 as $type, -2 as $type);
                    let p3 = p1 - p2;

                    assert_eq!(p3.x, -1 as $type);
                    assert_eq!(p3.y, 3 as $type);
                }

                /// Sub Point<T> - (T, T)
                #[test]
                fn [<sub_ $type _and_tuple>]() {
                    let p1 = Point2::<$type>::new(1 as $type, 1 as $type);
                    let p2 = (2 as $type, -2 as $type);
                    let p3 = p1 - p2;

                    assert_eq!(p3.x, -1 as $type);
                    assert_eq!(p3.y, 3 as $type);
                }

                /// Sub Point<T> -= Point<T>
                #[test]
                fn [<sub_assign_ $type _and_ $type>]() {
                    let mut p1 = Point2::<$type>::new(1 as $type, 1 as $type);
                    let p2 = Point2::<$type>::new(2 as $type, -2 as $type);
                    p1 -= p2;

                    assert_eq!(p1.x, -1 as $type);
                    assert_eq!(p1.y, 3 as $type);
                }

                /// Sub Point<T> -= (T, T)
                #[test]
                fn [<sub_assign_ $type _and_tuple>]() {
                    let mut p1 = Point2::<$type>::new(1 as $type, 1 as $type);
                    p1 -= (2 as $type, -2 as $type);

                    assert_eq!(p1.x, -1 as $type);
                    assert_eq!(p1.y, 3 as $type);
                }
            }
        };
    }

    macro_rules! test_mul {
        ($type: ident) => {
            paste::item! {
                /// Mul Point<T> * T
                #[test]
                fn [<mul_ $type _and_num>]() {
                    let p1 = Point2::<$type>::new(1 as $type as $type, 1 as $type);
                    let p3 = p1 * 2 as $type;

                    assert_eq!(p3.x, 2 as $type);
                    assert_eq!(p3.y, 2 as $type);
                }

                /// Mul Point<T> *= T
                #[test]
                fn [<mul_assign_ $type _and_num>]() {
                    let mut p1 = Point2::<$type>::new(2 as $type, 2 as $type);
                    p1 *= 2 as $type;

                    assert_eq!(p1.x, 4 as $type);
                    assert_eq!(p1.y, 4 as $type);
                }
            }
        };
    }

    macro_rules! test_div {
        ($type: ident) => {
            paste::item! {
                /// Div Point<T> / T
                #[test]
                fn [<div_ $type _and_num>]() {
                    let p1 = Point2::<$type>::new(4 as $type, 4 as $type);
                    let p3 = p1 / 2 as $type;

                    assert_eq!(p3.x, 2 as $type);
                    assert_eq!(p3.y, 2 as $type);
                }

                /// Div Point<T> /= T
                #[test]
                fn [<div_assign_ $type _and_num>]() {
                    let mut p1 = Point2::<$type>::new(4 as $type, 4 as $type);
                    p1 /= 2 as $type;

                    assert_eq!(p1.x, 2 as $type);
                    assert_eq!(p1.y, 2 as $type);
                }
            }
        };
    }

    macro_rules! test_all {
        ($($x: ident), *) => {
            $(
                test_add!($x);
                test_sub!($x);
                test_mul!($x);
                test_div!($x);
            )*
        };
    }

    test_all!(i8, i16, i32, i64, i128, f32, f64);
}
