fn main() {

    /* {
        let foo = 10;

        fn bar() -> int {
            return foo; // ILLEGAL
        }

        println!("{}", bar());
    } */

    {
        fn call_closure_with_ten(b: |int|) { b(12); }

        let captured_var = 20;
        let closure = |arg| /*{*/ println!("captured_var={}, arg={}",
                                            captured_var,    arg); /*};*/

        call_closure_with_ten(closure);
    }

    {
        let usquare = |x: uint| -> uint { (x * x) as uint };
        let square = |x: int| -> uint { (x * x) as uint };

        fn deep_f(f: |uint| -> uint, x: uint) -> uint { f(f(x)) }

        println!("{}", deep_f(usquare, 4));
    }

    {
        let mut max = 0;
        [1, 2, 3].map(|x| if *x > max { max = *x; });
    }

}
