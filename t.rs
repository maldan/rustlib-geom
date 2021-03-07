use geom::{point, Point2D};

fn main() {
    println!("{:?}", point!(3.0, 0.0) - point!(1.0, 0.0));
    println!("{:?}", point!(1.0, 1.0) * 0.5);
    println!("{:?}", point!(1.0, 1.0) / 2.0);

    let mut a = point!(1.0, 1.0);
    a *= 2.0;
    println!("{:?}", a);
}
