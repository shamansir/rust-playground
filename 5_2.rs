#[feature(struct_variant)]

struct Point {
    x: f64,
    y: f64
}

enum Shape {
    Circle(Point, f64),
    Rectangle(Point, Point)
}

enum Direction {
    North, East, South, West
}

enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff
}

/*enum ShapeStruct {
    Circle { center: Point, radius: f64 },
    Rectangle { top_left: Point, bottom_right: Point }
}*/

fn main() {

    fn area(sh: Shape) -> f64 {
        let PI = 3.14;
        match sh {
            Circle(_, size) => PI * size * size,
            Rectangle(Point { x, y }, Point { x: x2, y: y2 }) => (x2 - x) * (y2 - y)
        }
    }

    fn point_from_direction(dir: Direction) -> Point {
        match dir {
            North => Point { x: 0.0, y: 1.0 },
            East  => Point { x: 1.0, y: 0.0 },
            South => Point { x: 0.0, y: -1.0 },
            West  => Point { x: -1.0, y: 0.0 }
        }
    }

    /*fn point_str(p: Point) -> str {
        p.x.to_str() + ':' + p.y.to_str()
    }*/

    /*fn area_struct(sh: ShapeStruct) {
        match sh {
            Circle { radius: radius, _ } => PI * square(radius),
            Rectangle { top_left: tl, bottom_right: br } => {
                (br.x - tl.x) * (tl.y - br.y)
            }
        }
    }*/

    println!("{}", area(Circle(Point{x: 10.0, y: 10.0}, 20.0)));
    println!("{}", area(Rectangle(Point{x: 10.0, y: 10.0},
                                  Point{x: 10.0, y: 10.0})));
    println!("{}", area(Rectangle(Point{x: 10.0, y: 10.0},
                                  Point{x: 20.0, y: 20.0})));

    //println!("{}", point_from_direction(North));
}
