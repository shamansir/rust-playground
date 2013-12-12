fn main() {

    struct Point {
        x: f64,
        y: f64
    }

    {
        enum Shape {
            Circle(Point, f64),
            Rectangle(Point, Point)
        }

        impl Shape {
            fn draw(&self) {
                match *self {
                    Circle(p, f) => draw_circle(&p, f),
                    Rectangle(p1, p2) => draw_rectangle(&p1, &p2)
                }
            }
        }

        fn draw_circle(p: &Point, f: f64) {
            println!("circle {} {} {}", p.x, p.y, f);
        }

        fn draw_rectangle(p1: &Point, p2: &Point) {
            println!("rectangle {} {} {} {}", p1.x, p1.y, p2.x, p2.y);
        }

        let c = Circle(Point { x: 1.0, y: 2.0 }, 3.0);
        let r = Rectangle(Point { x: 2.0, y: 9.2 },
                          Point { x: 60.0, y: 10.0 });

        c.draw();
        r.draw();
    }

    {
        enum Shape {
            Circle(Point, f64),
            Rectangle(Point, Point)
        }

        impl Shape {
            fn draw_borrowed(&self) { println!("borrowed"); }
            fn draw_managed(@self) { println!("managed"); }
            fn draw_owned(~self) { println!("owned"); }
            fn draw_value(self) { println!("value"); }
        }

        let s = Circle(Point { x: 1.0, y: 2.0 }, 3.0);

        (@s).draw_managed();
        (~s).draw_owned();
        (&s).draw_borrowed();
        s.draw_value();

        (@s).draw_borrowed();
        (~s).draw_borrowed();

        s.draw_borrowed();

        (& &s).draw_borrowed();

        (&@~s).draw_borrowed();
    }

    {
        use std::f64::consts::PI;

        struct Circle {
            radius: f64
        }

        impl Circle {
            fn area(&self) -> f64 { (self.radius * self.radius) * PI }
            fn new(area: f64) -> Circle {
                Circle { radius: (area / PI).sqrt() }
            }
        }

        let c = Circle::new(42.5);

        println!("{}", c.radius);
        println!("{}", c.area());

    }

}
