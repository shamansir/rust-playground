fn main() {
    if false {
        println("that's odd");
    } else if true {
        println("right");
    } else {
        println("neither true nor false");
    }

    fn signum(x: int) -> int {
        if x < 0 { -1 }
        else if x > 0 { 1 }
        else { 0 }
    }

    println!("{}", signum(0));
    println!("{}", signum(-5));
    println!("{}", signum(287171));
}
