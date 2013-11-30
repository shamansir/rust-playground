use std::num;

fn main() {
    struct Point {
        x: f64,
        y: f64
    }

    let on_the_stack : Point  =  Point { x: 3.0, y: 4.0 };
    //let managed_box  : @Point = @Point { x: 5.0, y: 1.0 };
    let owned_box    : ~Point = ~Point { x: 7.0, y: 9.0 };

    fn compute_distance(p1: &Point, p2: &Point) -> f64 {
        let x_d = p1.x - p2.x;
        let y_d = p1.y - p2.y;
        num::sqrt(x_d * x_d + y_d * y_d)
    }

    //println!("{}", compute_distance(&on_the_stack, managed_box));
    //println!("{}", compute_distance(managed_box, owned_box));
    println!("{}", compute_distance(&on_the_stack, owned_box));
}
