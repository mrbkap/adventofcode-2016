extern crate regex;

use std::io::BufRead;
use std::collections::HashMap;
use std::cmp::Ordering;

use regex::Regex;

fn main() {
    let stdin = std::io::stdin();
    let re = Regex::new(r"([a-z-]+)([0-9]+)\[([a-z]+)\]").unwrap();
    let mut sums = 0u32;
    'nextline: for line in stdin.lock().lines().filter_map(|l| l.ok()) {
        let mut letters = HashMap::new();
        let caps = re.captures(&line).unwrap();
        if let Some(encoded) = caps.at(1) {
            for c in encoded.chars() {
                match c {
                    '-' => continue,
                    _ => {
                        let cnt = letters.entry(c).or_insert(0);
                        *cnt += 1;
                    }
                }
            }
        }

        if let Some(checksum) = caps.at(3) {
            let mut counts: Vec<_> = letters.into_iter().collect();
            counts.sort_by(|&(c1, i1), &(c2, i2)| {
                match i2.cmp(&i1) {
                    Ordering::Equal => c1.cmp(&c2),
                    r @ _ => r,
                }
            });

            let mut counts_iter = counts.iter();
            for c in checksum.chars() {
                loop {
                    match counts_iter.next() {
                        Some(character) => {
                            if character.0 == c {
                                break;
                            }
                        }
                        None => {
                            // bad checksum.
                            continue 'nextline;
                        }
                    }
                }
            }

            sums += caps.at(2).unwrap().parse::<u32>().unwrap();
        }
    }

    println!("{}", sums);
}
