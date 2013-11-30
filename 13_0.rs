fn main() {

    {
        let numbers = [1, 2, 3];
        let more_numbers = numbers;

        //println!("{}", numbers);
        //println!("{}", more_numbers);

        let five_zeroes: [int, ..5] = [0, ..5];

        //println!("{}", five_zeroes);
    }

    {
        let mut numbers = ~[1, 2, 3];
        numbers.push(4);
        numbers.push(5);

        //println!("{}", numbers);

        let more_numers: ~[int] = numbers;

        //println!("{}", more_numbers);
    }

    {   let mut string = ~"fo";
        string.push_char('o');

        println!("{}", string); }

    {
        let xs = &[1, 2, 3];
        let ys: &[int] = xs;

        let three = [1, 2, 3];
        let zs: &[int] = three;

        let string = "foobar";
        let view: &str = string.slice(0, 3);
        println!("{} {}", string, view);
    }

    {
        let mut xs = [1, 2, 3];
        let view = xs.mut_slice(0, 2);
        view[0] = 5;

        let ys: &mut [int] = &mut [1, 2, 3];
    }

    {
        enum Crayon { BananaMania, Beaver, Bittersweet };
        let crayons: [Crayon, ..3] = [ BananaMania,
                                       Beaver,
                                       Bittersweet ];
        match crayons[0] {
            BananaMania => println!("bananamania"),
            //Bittersweet => println!("bittersweet")
            _ => ()
        }
    }

    {
        let numbers: &[int] = &[1, 2, 3];
        let score = match numbers {
            [] => 0,
            [a] => a * 10,
            [a, b] => a * 6 + b * 4,
            [a, b, c, ..rest] => a * 5 + b * 3 + c * 2 + rest.len() as int

        };
        println!("{}", score);
    }

}
