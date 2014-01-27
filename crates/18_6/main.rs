mod farm {
    pub use self::barn::hay;

    pub fn chicken() { println!("cluck cluck"); }
    pub fn cow() { println!("mooo"); }

    pub mod barn {
        pub fn hay() { println!("..."); }
    }
}

fn main() {
    println!("Hello farm!");

    farm::chicken();
    farm::cow();
    farm::hay();
}
