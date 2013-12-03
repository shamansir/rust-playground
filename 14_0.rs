//use std::rc::Rc;

fn main() {

    {
        use std::rc::Rc;

        let x = Rc::new([1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
        let y = x.clone();
        let z = x;

        assert_eq!(*z.borrow(), [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);

        let mut a = Rc::new([10, 9, 8, 7, 6, 5, 4, 3, 2, 1]);
        a = z;
    }

    {
        use std::gc::Gc;

        let x = Gc::new([1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
        let y = x;
        let z = x;

        assert_eq!(*z.borrow(), [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }

}
