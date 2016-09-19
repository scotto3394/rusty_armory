extern crate gearlib;

use gearlib::structure::*;

fn main() {
    let mut x: Player = Default::default();
    println!("{:?}", x);
    x.add_toon(Toon::new("Emixan".to_string()));
    println!("{:?}", x);
}
