fn main() {

    {
        trait Printable {
            fn print(&self);
        }

        impl Printable for int {
            fn print(&self) { println!("{:?}", *self) }
        }

        impl Printable for ~str {
            fn print(&self) { println!("{}", *self) }
        }

        5.print();
        (~"foo").print();

    }

    {
        trait Seq<T> {
            fn length(&self) -> uint;
        }

        impl<T> Seq<T> for ~[T] {
            fn length(&self) -> uint { self.len() }
        }
    }

    {
        trait Eq {
            fn equals(&self, other: &Self) -> bool;
        }

        impl Eq for int {
            fn equals(&self, other: &int) -> bool { *other == *self }
        }
    }

    {
        trait Shape { fn new(area: f64) -> Self; }
        struct Circle { radius: f64 }
        struct Square { length: f64 }

        impl Shape for Circle {
            fn new(area: f64) -> Circle {
                Circle { radius: (area / 3.14).sqrt() }
            }
        }

        impl Shape for Square {
            fn new(area: f64) -> Square {
                Square { length: area.sqrt() }
            }
        }

        let area = 42.5;
        let c: Circle = Shape::new(area);
        let s: Square = Shape::new(area);

        println!("{}", c.radius);
        println!("{}", s.length);
    }

}
