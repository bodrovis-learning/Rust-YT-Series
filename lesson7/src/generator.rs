use crate::prelude::*;
use random_number::RandomNumber;

mod random_number;

pub fn generate() -> RandomNumber {
    super::shared();

    let random_num = rand::thread_rng().gen_range(LOW..=HIGH);
    RandomNumber::new(random_num)
}

pub fn other_fn() {
    println!("other fn");
}
