fn main() {

    {
        fn map<T, U>(vector: &[T], function: |v: &T| -> U) -> ~[U] {
            let mut accumulator = ~[];
            for element in vector.iter() {
                accumulator.push(function(element));
            }
            return accumulator;
        }

        map(&[20, 14, 25], |x: &int| -> uint { (*x * *x) as uint });

    }

    {
        use std::hashmap::HashMap;

        type Set<T> = HashMap<T, ()>;

        struct Stack<T> {
            elements: ~[T]
        }

        enum Option<T> {
            Some(T),
            None
        }
    }

    {

        /*fn radius(shape: Shape) -> Option<f64> {
            match shape {
                Circle(_, radius) => Some(radius),
                Rectangle(..)     => None
            }
        }*/

    }

}
