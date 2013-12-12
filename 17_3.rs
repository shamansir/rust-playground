fn main() {

    trait Printable {
        fn print(&self);
    }

    impl Printable for int {
        fn print(&self) { println!("{}", *self); }
    }

    {

        fn print_all<T: Printable>(printable_things: ~[T]) {
            for thing in printable_things.iter() {
                thing.print();
            }
        }

        print_all(~[2, 4, 6]);

    }

    {
        fn print_all<T: Printable + Clone>(printable_things: ~[T]) {
            let mut i = 0;
            while i < printable_things.len() {
                let copy_of_thing = printable_things[i].clone();
                copy_of_thing.print();
                i += 1;
            }
        }

        print_all(~[7, 9, 18]);
    }

}
