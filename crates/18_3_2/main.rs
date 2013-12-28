mod farm {

    pub struct Chicken;
    pub struct Human {
        name: ~str
    }

    impl Human {
        pub fn rest(&self) {
            println!("{} is resting", self.name);
        }
    }

    pub struct Farm {
        priv chickens: ~[Chicken],
        farmer: Human
    }

    impl Farm {
        fn feed_chickens(&self) { /* ... */ }
        pub fn add_chicken(&self, c: Chicken) { /* ... */ }
        pub fn create(farmer_name: ~str) -> Farm {
            Farm { chickens: ~[],
                   farmer: Human {
                       name: farmer_name
                   } }
        }
    }

    pub fn feed_animals(farm: &Farm) {
        farm.feed_chickens();
    }

}

fn make_me_a_farm(farmer_name: ~str) -> farm::Farm {
    farm::Farm::create(farmer_name)
}

fn make_me_a_chicken() -> farm::Chicken { farm::Chicken }

fn main() {
    let f = make_me_a_farm(~"Jonny");
    f.add_chicken(make_me_a_chicken());
    farm::feed_animals(&f);
    f.farmer.rest();

    // f.feed_chickens();
    // let chicken_counter = f.chickens.len()
}
