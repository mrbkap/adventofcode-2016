extern crate crypto;

use std::io::{self, Read};
use crypto::md5;
use crypto::digest::Digest;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    let buffer = buffer.trim();

    let mut generator = md5::Md5::new();
    let mut x = 0u64;
    let mut answer = String::new();

    loop {
        generator.input_str(&buffer);
        generator.input_str(&x.to_string());

        let result = generator.result_str();
        if result.starts_with("00000") {
            answer.push(result.chars().nth(5).unwrap());
            if answer.len() == 8 {
                break;
            }
        }
        generator.reset();
        x += 1;
    }

    println!("{}", answer);
}
