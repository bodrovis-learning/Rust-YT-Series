#![warn(clippy::all, clippy::pedantic)]

use bytes::Bytes;
use std::cmp::Ordering;
use std::io;

fn main() {
    // XOR cipher
    cipher();
    decipher();
}

fn cipher() {
    println!("Enter your string: ");
    let input = read_str();

    println!("Enter your password: ");

    let password = pad_or_trim(input.len(), read_str().as_bytes());

    let result = zip_and_xor(&password, Bytes::from(input));

    println!("Ciphered string:");
    println!("{result:?}");
}

fn decipher() {
    println!("Enter your ciphered data: ");
    let input: Vec<u8> = read_str()
        .split(',')
        .map(|el| el.trim().parse::<u8>().expect("invalid int"))
        .collect();

    println!("Enter your password: ");
    let password = pad_or_trim(input.len(), read_str().as_bytes());

    let back = zip_and_xor(&password, Bytes::from(input));
    let back_converted = String::from_utf8(back).unwrap();

    println!("Initial string: {back_converted:?}");
}

fn zip_and_xor(password: &[u8], bytes_data: Bytes) -> Vec<u8> {
    password
        .iter()
        .zip(bytes_data)
        .map(|(&a, b)| a ^ b)
        .collect()
}

fn pad_or_trim(limit: usize, original_pass: &[u8]) -> Vec<u8> {
    let password_len = original_pass.len();

    match limit.cmp(&password_len) {
        Ordering::Less => original_pass.iter().copied().take(limit).collect(),

        Ordering::Greater => {
            let mut as_bytes: Vec<u8> = original_pass.to_vec();

            for i in password_len..limit {
                as_bytes.push(as_bytes[i % password_len]);
            }

            as_bytes
        }

        Ordering::Equal => original_pass.to_vec(),
    }
}

fn read_str() -> String {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();

    input.trim().into()
}
