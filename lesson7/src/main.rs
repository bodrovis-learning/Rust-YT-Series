#![warn(clippy::all, clippy::pedantic)]

mod generator;

mod prelude {
    pub const LOW: u8 = 1;
    pub const HIGH: u8 = 150;
    pub use crate::generator::{generate, other_fn};
    pub use std::env as environment;
    pub use rand::Rng;
}

use prelude::*;

fn main() {
    other_fn();

    let random = generate();
    println!("Random value: {}", random.value);

    let args: Vec<String> = environment::args().collect();
    let first_arg = &args[0];
    println!("{first_arg}");
}

pub fn shared() {
    println!("shared!");
}
