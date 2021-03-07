#[derive(Debug, Clone, Copy)]
pub struct Rectangle {
    pub left: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
}

impl Rectangle {
    #[inline(always)]
    pub fn new(left: f32, top: f32, right: f32, bottom: f32) -> Rectangle {
        Rectangle {
            left,
            top,
            right,
            bottom,
        }
    }

    #[inline(always)]
    pub fn width(&self) -> f32 {
        (self.left - self.right).abs()
    }

    #[inline(always)]
    pub fn height(&self) -> f32 {
        (self.top - self.bottom).abs()
    }

    #[inline(always)]
    pub fn area(&self) -> f32 {
        self.width() * self.height()
    }

    #[inline(always)]
    pub fn is_collide(r1: Rectangle, r2: Rectangle) -> bool {
        !(r1.left > r2.right || r1.right < r2.left || r1.bottom < r2.top || r1.top > r2.bottom)
    }
}
