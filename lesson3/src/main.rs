#![warn(clippy::all, clippy::pedantic)]

use std::io;

fn process(str: &str) -> u8 {
    str.trim().parse::<u8>().expect("Please type a number!")
}

fn main() {
    let mut user_choice: String = String::new();

    io::stdin().read_line(&mut user_choice).unwrap();

    let n_choice = process(&user_choice);
    process("test");
    
    println!("Number: {n_choice}");
    println!("String: {user_choice}");
}