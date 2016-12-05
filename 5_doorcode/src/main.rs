extern crate crypto;

use std::io::{self, Read};
use crypto::md5;
use crypto::digest::Digest;
use std::collections::HashSet;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    let buffer = buffer.trim();

    let mut generator = md5::Md5::new();
    let mut x = 0u64;
    let mut seen = HashSet::new();
    let mut answer: Vec<char> = vec!['\0'; 8];

    loop {
        generator.input_str(&buffer);
        generator.input_str(&x.to_string());

        let result = generator.result_str();
        if result.starts_with("00000") {
            if let Some(pos) = result.chars().nth(5).unwrap().to_digit(10) {
                if pos <= 7 && !seen.contains(&pos) {
                    seen.insert(pos);
                    answer[pos as usize] = result.chars().nth(6).unwrap();
                    if seen.len() == 8 {
                        break;
                    }
                }
            }
        }
        generator.reset();
        x += 1;
    }

    let as_str : String = answer.into_iter().collect();
    println!("{}", as_str);
}
