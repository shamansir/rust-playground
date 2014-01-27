fn main() {
    let mut cake_amount = 8;
    while cake_amount > 0 {
        cake_amount -= 1;
        println!("{}", cake_amount);
    }

    println!("__");

    let mut x = 5u;
    loop {
        x += x - 3;
        if x % 5 == 0 { break; }
        println!(x.to_str());
    }
}
