#[cfg(test)]
mod tests {
    use geom::Rectangle;

    #[test]
    fn base() {
        let r1 = Rectangle::new(10.0, 20.0, 30.0, 40.0);
        assert_eq!(r1.left, 10.0);
        assert_eq!(r1.top, 20.0);
        assert_eq!(r1.right, 30.0);
        assert_eq!(r1.bottom, 40.0);
    }

    #[test]
    fn size() {
        let r1 = Rectangle::new(-10.0, -10.0, 10.0, 10.0);
        assert_eq!(r1.width(), 20.0);
        assert_eq!(r1.height(), 20.0);

        let r1 = Rectangle::new(10.0, 10.0, -10.0, -10.0);
        assert_eq!(r1.width(), 20.0);
        assert_eq!(r1.height(), 20.0);

        let r1 = Rectangle::new(-10.0, -15.0, 10.0, 15.0);
        assert_eq!(r1.width(), 20.0);
        assert_eq!(r1.height(), 30.0);
    }

    #[test]
    fn collision() {
        let r1 = Rectangle::new(-10.0, -10.0, 10.0, 10.0);
        let r2 = Rectangle::new(-9.0, -9.0, 9.0, 9.0);
        assert_eq!(Rectangle::is_collide(r1, r2), true);

        let r1 = Rectangle::new(-10.0, -10.0, 0.0, 0.0);
        let r2 = Rectangle::new(1.0, 1.0, 9.0, 9.0);
        assert_eq!(Rectangle::is_collide(r1, r2), false);

        let r1 = Rectangle::new(-1.0, -1.0, 0.0, 0.0);
        let r2 = Rectangle::new(-1.0, -1.0, 0.0, 0.0);
        assert_eq!(Rectangle::is_collide(r1, r2), true);

        let r1 = Rectangle::new(-1.0, -1.0, 0.0, 0.0);
        let r2 = Rectangle::new(-0.9, -1.0, 0.1, 0.0);
        assert_eq!(Rectangle::is_collide(r1, r2), true);

        let r1 = Rectangle::new(-10.0, -10.0, 0.0, 0.0);
        let r2 = Rectangle::new(10.9, 10.0, 20.1, 20.0);
        assert_eq!(Rectangle::is_collide(r1, r2), false);
    }
}
