fn main() {
    trait Drawable { fn draw(&self); }

    struct Circle { radius: f64 }
    struct Rectangle { width: f64, height: f64 }

    impl Drawable for Circle {
        fn draw(&self) {
            println!("drawing with radius {}", self.radius);
        }
    }

    impl Drawable for Rectangle {
        fn draw(&self) {
            println!("drawing with width {} and height {}",
                     self.width, self.height);
        }
    }

    {

        fn draw_all<T: Drawable>(shapes: ~[T]) {
            for shape in shapes.iter() { shape.draw(); }
        }

        draw_all(~[ Circle { radius: 4.0 },
                    Circle { radius: 10.0 } ]);
        draw_all(~[ Rectangle { width: 2.0, height: 15.0 },
                    Rectangle { width: 10.0, height: 19.0 } ]);

        // fails:
        // draw_all(~[ Circle { radius: 4.0 },
        //             Rectangle { width: 10.0, height: 19.0 } ]);
        // also fails:
        // draw_all(~[ Circle { radius: 4.0 } as Drawable,
        //             Rectangle { width: 10.0, height: 19.0 } as Drawable ]);

    }

    /*{
        fn draw_all(shapes: &[@Drawable]) {
            for shape in shapes.iter() { shape.draw(); }
        }

        let c: @circle = @new_circle();
        let r: @Rectangle = @new_rectangle();
        draw_all([ c as @Drawable, r as @Drawable ]);

    }*/

    /*{
        let boxy: @Drawable = @new_circle() as @Drawable;
        let owny: ~Drawable = ~new_circle() as ~Drawable;
        let stacky: &Drawable = &new_circle() as &Drawable;
    }*/
}
