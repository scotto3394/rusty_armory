extern crate gearlib;

use gearlib::structure::*;

fn main() {
    let mut x: Player = Player::new("Emixan".to_string());
    println!("{:?}", x);
    x.add_toon(Toon::new("Emixan".to_string()));
    println!("{:?}", x);
    println!("{}", x);
}
