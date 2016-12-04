extern crate regex;

use std::io::BufRead;
use std::collections::HashMap;
use std::cmp::Ordering;

use regex::Regex;

fn main() {
    let stdin = std::io::stdin();
    let re = Regex::new(r"([a-z-]+)([0-9]+)\[([a-z]+)\]").unwrap();
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

                const A : u32 = 'a' as u32;
                const Z : u32 = 'z' as u32;
                let sector_id = caps.at(2).unwrap().parse::<u32>().unwrap();
                let shift = sector_id % 26;
                let shifted : Vec<u8> = encoded.chars().map(|c| {
                    match c {
                        '-' => ' ' as u8,
                        c1 @ _ => {
                            let mut i = c1 as u32;
                            i -= A;
                            i += shift;
                            i = i % (Z - A + 1);
                            i += A;
                            i as u8
                        }
                    }
                }).collect();

                let decoded = String::from_utf8(shifted).unwrap();
                if let Some(_) = decoded.find("north") {
                    println!("{}{}", decoded, sector_id);
                }
            }
        }
    }
}
