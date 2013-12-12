fn main() {

    fn head<T: Clone>(v: &[T]) -> T {
        v[0].clone()
    }

    struct TimeBomb {
        explosivity: uint
    }

    impl Drop for TimeBomb {
        fn drop(&mut self) {
            for _ in range(0, self.explosivity) {
                println("blam!");
            }
        }
    }

    let a = TimeBomb { explosivity: 6 };
    println!("{}", a.explosivity);

}
