use geom::{point, Point2};

fn main() {
    /*println!("{:?}", point!(3.0, 0.0) - point!(1.0, 0.0));
    println!("{:?}", point!(1.0, 1.0) * 0.5);
    println!("{:?}", point!(1.0, 1.0) / 2.0);
    println!("{:?}", point!(3.0, 0.0) + (1.0, 0.0));

    let mut a = point!(1.0, 1.0);
    // a *= 2.0;
    println!("{:?}", a);*/

    let mut v_32_1 = Point2::<i8>::new(1, 1);
    let mut v_32_2 = Point2::<i8>::new(1, 1);
    v_32_1 += (1, 0);

    // let v_32_1 += (1, 1);
    /*let mut v_32_1 = Point2::<i32>::new(1 as i32, 1 as i32);
    let mut v_32_2 = Point2::<i32>::new(1 as i32, 1 as i32);
    let mut v_16 = Point2::<i16>::new(1 as i16, 1 as i16);*/

    println!("{:?}", v_32_1);
}
