fn main() {

    trait Eq {
        fn equals(&self, other: &Self) -> bool;
    }

    // FIXME: make deriving work
    // #[deriving(Eq)]
    struct Circle { radius: f64 }

    impl Eq for Circle {
        fn equals(&self, other: &Circle) -> bool {
            self.radius == other.radius
        }
    }

    //#[deriving(Rand, ToStr)]
    //enum ABC { A, B, C }

    println!("{}", (Circle { radius: 6.0 }).equals(&Circle { radius: 4.0 }));
    println!("{}", (Circle { radius: 7.0 }).equals(&Circle { radius: 7.0 }));

}
