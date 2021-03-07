#[cfg(test)]
mod tests {
    use geom::Point2D;

    #[test]
    fn point_ops() {
        let p1 = Point2D::new(10.0, 10.0);
        let p2 = Point2D::new(1.0, 1.0);
        let p3 = p1 - p2;

        assert_eq!(p3.x, 9.0);
        assert_eq!(p3.y, 9.0);
    }
}
