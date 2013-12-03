fn main() {

    {
        fn call_twice(f: ||) { f(); f(); }

        let closure = || { "I'm a closure, and it doesn't matter what type I am"; };
        fn function() { "I'm a normal function"; }

        call_twice(closure);
        call_twice(function);
    }

    {
        let int_closure = || -> int { 5 };
        fn int_function() -> int { 12 }

        fn print_twice(f: || -> int) {
            println!("{} {}", f(), f());
        }

        print_twice(int_closure);
        print_twice(int_function);
    }

}
