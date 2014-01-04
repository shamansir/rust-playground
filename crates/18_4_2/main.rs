mod plants;

mod animals {
    mod fish;
    mod mammals {
        mod humans;
    }
}

#[path="animals/mammals/aliens.rs"]
mod unknown;

fn main() {
    println((plants::Flower { name: ~"Foo-Flower" }).name);
    println((animals::fish::Salmon { name: ~"Foo-Salmon" }).name);
    println((animals::mammals::humans::Man { name: ~"Foo-Man" }).name);
    println((unknown::Martian { name: ~"Foo-Martian" }).name);
}
