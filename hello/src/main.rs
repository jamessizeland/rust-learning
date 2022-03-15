use hello::greet;
// use std::collections::HashMap; // https://doc.rust-lang.org/std/
use rand::Rng;

fn main() {
    greet(); // import from lib.rs
    let mut rng = rand::thread_rng();
    let x: u8 = rng.gen_range(0..100);
    let y: f64 = rng.gen_range(0.0_f64..100.0_f64); // _ gets ignored, can be used to make clearer formatting
    println!("{}", x);
    println!("Hello, world!");
}
