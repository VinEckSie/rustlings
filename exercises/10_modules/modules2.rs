// You can bring module paths into scopes and provide new names for them with
// the `use` and `as` keywords.
use delicious_snacks::fruits::PEAR as fruit;
use delicious_snacks::veggies::CUCUMBER as veggie;

pub mod delicious_snacks {

    pub mod fruits {
        pub const PEAR: &str = "Pear";
        pub const APPLE: &str = "Apple";
    }

    pub mod veggies {
        pub const CUCUMBER: &str = "Cucumber";
        pub const CARROT: &str = "Carrot";
    }
}

fn main() {
    println!(
        "favorite snacks: {} and {}",
        fruit,
        veggie,
    );
}
