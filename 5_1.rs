struct Point {
    x: f64,
    y: f64
}

fn main() {

    fn point_info(pt: Point) {
        match pt {
            Point { x: 0.0, y: yy } => { println(yy.to_str()); }
            Point { x: xx,  y: yy } => { println(xx.to_str() +
                                           "/" + yy.to_str()); }
        }
    }

    fn x_info(pt: Point) {
        match pt {
            Point { x, _ } => { println(x.to_str()); }
        }
    }

    point_info(Point { x: 0.0, y: 5 as f64 });
    point_info(Point { x: 0.1, y: 7.1 });

    x_info(Point { x: 0.0, y: 5.0 });
    x_info(Point { x: 0.1, y: 7.1 });
}
