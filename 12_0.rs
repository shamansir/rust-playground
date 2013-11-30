fn main() {

    {
        let managed = @10;
        let owned = ~20;
        let borrowed = &30;

        println!("{}", *managed + *owned + *borrowed);

    }

    {
        let managed = @mut 10;
        let mut owned = ~20;

        let mut value = 30;
        let borrowed = &mut value;

        *managed = *owned + 10;
        *owned = *borrowed + 100;
        *borrowed = *managed + 1000;

        println!("{} {} {}", *managed, *owned, *borrowed);
    }

    {
        struct Point { x: f64, y: f64 };
        // FIXME: make it a class
        //struct Rectangle { start: Point, end: Point };

        let start = @Point { x: 10.0, y: 20.0 };
        let end = ~Point { x: (*start).x + 100.0,
                           y: (*start).y + 100.0 };
        // FIXME: make it work
        //let rect = &Rectangle(*start, *end);
        //let area = (*rect).area();

        println!("{} {} / {} {}", (*start).x, (*start).y,
                                    (*end).x,   (*end).y);
        //println!("{}", area);
    }

    {
        struct Point { x: f64, y: f64 };
        // FIXME: make it a class
        struct Rectangle { start: Point, end: Point };

        let start = @Point { x: 10.0, y: 20.0 };
        let end = ~Point { x: start.x + 100.0,
                           y: start.y + 100.0 };
        // FIXME: make it work
        //let rect = &Rectangle(*start, *end);
        //let area = rect.area();

        println!("{} {} / {} {}", start.x, start.y,
                                    end.x,   end.y);
        //println!("{}", area);
    }

}
