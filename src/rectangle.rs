use crate::Point2;

#[derive(Debug, Clone, Copy)]
pub struct Rectangle<T> {
    pub left: T,
    pub top: T,
    pub right: T,
    pub bottom: T,
}

macro_rules! define_struct {
    ($type: ident) => {
        impl Rectangle<$type> {
            #[inline(always)]
            pub fn new(l: $type, t: $type, r: $type, b: $type) -> Rectangle<$type> {
                Rectangle {
                    left: l,
                    top: t,
                    right: r,
                    bottom: b,
                }
            }

            #[inline(always)]
            pub fn width(&self) -> $type {
                (self.left - self.right).abs()
            }

            #[inline(always)]
            pub fn height(&self) -> $type {
                (self.top - self.bottom).abs()
            }

            #[inline(always)]
            pub fn area(&self) -> $type {
                self.width() * self.height()
            }

            #[inline(always)]
            pub fn is_collide(r1: Rectangle<$type>, r2: Rectangle<$type>) -> bool {
                !(r1.left > r2.right
                    || r1.right < r2.left
                    || r1.bottom < r2.top
                    || r1.top > r2.bottom)
            }

            #[inline(always)]
            pub fn is_point_inside(rect: Rectangle<$type>, point: Point2<$type>) -> bool {
                rect.left < point.x
                    && point.x < rect.right
                    && rect.top < point.y
                    && point.y < rect.bottom
            }
        }
    };
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

    macro_rules! test_base {
        ($type: ident) => {
            paste::item! {
                #[test]
                fn [<base_ $type>]() {
                    let r1 = Rectangle::<$type>::new(10 as $type, 20 as $type, 30 as $type, 40 as $type);
                    assert_eq!(r1.left, 10 as $type);
                    assert_eq!(r1.top, 20 as $type);
                    assert_eq!(r1.right, 30 as $type);
                    assert_eq!(r1.bottom, 40 as $type);
                }
            }
        };
    }

    macro_rules! test_size {
        ($type: ident) => {
            paste::item! {
                #[test]
                fn [<size_ $type>]() {
                    let r1 = Rectangle::<$type>::new(-10  as $type, -10  as $type, 10  as $type, 10  as $type);
                    assert_eq!(r1.width(), 20 as $type);
                    assert_eq!(r1.height(), 20 as $type);

                    let r1 = Rectangle::<$type>::new(10  as $type, 10  as $type, -10  as $type, -10  as $type);
                    assert_eq!(r1.width(), 20 as $type);
                    assert_eq!(r1.height(), 20 as $type);

                    let r1 = Rectangle::<$type>::new(-10  as $type, -15  as $type, 10  as $type, 15  as $type);
                    assert_eq!(r1.width(), 20 as $type);
                    assert_eq!(r1.height(), 30 as $type);
                }
            }
        };
    }

    macro_rules! test_all {
        ($($x: ident), *) => {
            $(
                test_base!($x);
                test_size!($x);
            )*
        };
    }

    test_all!(i8, i16, i32, i64, i128, f32, f64);
}
