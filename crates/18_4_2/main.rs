mod plants;

mod animals {
    pub mod fish;
    pub mod mammals {
        pub mod humans;
    }
}

#[path="animals/mammals/aliens.rs"]
mod unknown;

fn main() {
    //let foo_flower  = plants::Flower { name: ~"Foo-Flower" };
    //let foo_salmon  = animals::fish::Salmon { name: ~"Foo-Salmon" };
    //let foo_man     = animals::mammals::humans::Man { name: ~"Foo-Man" };
    //let foo_martian = unknown::Martian { name: ~"Foo-Martian" };

    println!("{}", (plants::Flower { name: ~"Foo-Flower" }).name);
    println!("{}", (animals::fish::Salmon { name: ~"Foo-Salmon" }).name);
    println!("{}", (animals::mammals::humans::Man { name: ~"Foo-Man" }).name);
    println!("{}", (unknown::Martian { name: ~"Foo-Martian" }).name);
}
