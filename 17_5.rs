fn main() {

    trait Shape { fn area(&self) -> f64; }
    trait Circle : Shape { fn radius(&self) -> f64; }

    struct Point { x: f64, y: f64 }
    struct CircleStruct { center: Point, radius: f64 }

    impl Circle for CircleStruct {
        fn radius(&self) -> f64 { (self.area() / 3.14).sqrt() }
    }
    impl Shape for CircleStruct {
        fn area(&self) -> f64 { 3.14 * self.radius * self.radius }
    }

    fn radius_times_area<T: Circle>(c: T) -> f64 {
        c.radius() * c.area()
    }

    let concrete = @CircleStruct { center: Point { x: 3f64, y: 4f64 },
                                   radius: 5f64 };
    //let mycircle: @Circle = concrete as @Circle;
    //let nonsense = mycircle.radius() * mycircle.area();

    println!("{}", radius_times_area(*concrete));

}
