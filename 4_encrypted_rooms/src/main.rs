extern crate regex;

use std::io::BufRead;
use std::collections::HashMap;
use std::cmp::Ordering;

use regex::Regex;

struct Room {
    name: String,
    checksum: String,
    sector: u32,
}

impl Room {
    fn parse(captures: &regex::Captures) -> Room {
        return Room {
            name: captures.at(1).unwrap().into(),
            checksum: captures.at(3).unwrap().into(),
            sector: captures.at(2).unwrap().parse::<u32>().unwrap(),
        }
    }

    fn char_occurances(&self) -> HashMap<char, isize> {
        let mut letters = HashMap::new();
        for c in self.name.chars().filter(|c| *c != '-') {
            *letters.entry(c).or_insert(0) += 1;
        }

        letters
    }

    fn is_valid(&self) -> bool {
        let letters = self.char_occurances();
        let mut counts: Vec<_> = letters.into_iter().collect();
        counts.sort_by(|&(c1, i1), &(c2, i2)| {
            match i2.cmp(&i1) {
                Ordering::Equal => c1.cmp(&c2),
                r @ _ => r,
            }
        });

        let counts_iter = counts.iter().take(5);
        return self.checksum.chars().zip(counts_iter).all(|pair| (pair.0) == (pair.1).0);
    }

    fn decoded_name(&self) -> String {
        const A : u32 = 'a' as u32;
        let shift = self.sector % 26;
        let shifted : Vec<u8> = self.name.chars().map(|c| {
            match c {
                '-' => ' ' as u8,
                c1 @ _ => {
                    let mut i = c1 as u32;
                    i -= A;
                    i += shift;
                    i = i % 26;
                    i += A;
                    i as u8
                }
            }
        }).collect();

        String::from_utf8(shifted).unwrap()
    }
}

fn main() {
    let stdin = std::io::stdin();
    let re = Regex::new(r"([a-z-]+)([0-9]+)\[([a-z]+)\]").unwrap();
    for line in stdin.lock().lines().filter_map(|l| l.ok()) {
        let caps = re.captures(&line).unwrap();
        let room = Room::parse(&caps);

        if !room.is_valid() {
            continue;
        }

        let decoded = room.decoded_name();
        if let Some(_) = decoded.find("north") {
            println!("{}{}", decoded, room.sector);
        }
    }
}
