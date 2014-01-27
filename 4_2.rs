//use std::f64;
//use std::f64::num::atan;

fn main() {
    let my_number = 7;
    match my_number {
        0     => println!("zero"),
        1 | 2 => println!("one or two"),
        3..10 => println!("three to ten"),
        _     => println!("something else")
    }

    match my_number {
        0 => { println!("zero") }
        _ => { println!("something else") }
    }

}

/*fn angle(vector: (f64, f64)) -> f64 {
    let pi = f64::consts::PI;
    match vector {
        (0.0, y) if y < 0.0 => 1.5 * pi,
        (0.0, y) => 0.5 * pi,
        (x, y) => atan(y / x)
    }
}*/

fn angle(vector: (f64, f64)) -> f64 {
    let pi = 3.14;
    match vector {
        (0.0, y) if y < 0.0 => 1.5 * pi,
        (0.0, _) => 0.5 * pi,
        (x, y) => y / x
    }
}
