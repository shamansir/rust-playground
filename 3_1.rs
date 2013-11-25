fn main() {
    let price;
    let item = "muffin";
    if item == "salad" {
        price = 3.50;
    } else if item == "muffin" {
        price = 2.25;
    } else {
        price = 2.00;
    }
    println!("{}", price);

    let price2 = (if item == "salad"  { 3.50 }
             else if item == "muffin" { 2.25 }
             else                     { 2.00 });
    println!("{}", price2);


    fn is_four(x: int) -> bool {
        x == 4
    }
    println!("5 is four: {}", is_four(5));
}
