fn main() {

    {
        fn call_it(op: proc(v: int)) {
            op(10)
        }

        call_it(proc(n) {
            println!("{}", n);
        });

        do call_it() |n| {
            println!("{}", n);
        }
    }

    {
        use std::task::spawn;

        do spawn() || {
            debug!("I'm a task, whatever");
        }
    }

    {
        use std::task::spawn;
        do spawn {
            debug!("Kablam!");
        }
    }

}
