struct Point {
    x: f64,
    y: f64
}

fn main() {

    fn point_info(pt: Point) {
        match pt {
            Point { x: 0.0, y: yy } => { println!("{}", yy); }
            Point { x: xx,  y: yy } => { println!("{}, {}", xx, yy); }
        }
    }

    fn x_info(pt: Point) {
        match pt {
            Point { x, .. } => { println!("{}", x); }
        }
    }

    point_info(Point { x: 0.0, y: 5 as f64 });
    point_info(Point { x: 0.1, y: 7.1 });

    x_info(Point { x: 0.0, y: 5.0 });
    x_info(Point { x: 0.1, y: 7.1 });
}
