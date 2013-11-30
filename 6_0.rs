fn main() {

    fn line_ret(a: int, b: int, x: int) -> int {
        return a * x + b;
    }

    println!("{}", line_ret(2, 5, 17));

    fn line_noret(a: int, b: int, x: int) -> int { a * x + b }

    println!("{}", line_noret(2, 5, 17));

    fn do_nothing_the_hard_way() -> () { return (); }

    //println!("{}", do_nothing_the_hard_way());

    fn do_nothing_the_easy_way() {}

    //println!("{}", do_nothing_the_easy_way());

    fn line(a: int, b: int, x: int) -> int { a * x + b }
    fn oops(a: int, b: int, x: int) -> ()  { a * x + b; }

    assert!(8 == line(5, 3, 1));
    assert!(() == oops(5, 3, 1));

    fn first((value, _): (int, f64)) -> int { value }

    println!("{}", first((60, 10.0)));
    println!("{}", first((15.3 as int, 10 as f64)));

}
