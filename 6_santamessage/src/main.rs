use std::io;
use std::io::BufRead;
use std::collections::HashMap;

fn main() {
    let mut frequences : Vec<HashMap<char, u32>> = Vec::new();
    let stdin = io::stdin();
    for line in stdin.lock().lines().filter_map(|l| l.ok()) {
        if line.len() > frequences.len() {
            assert!(frequences.len() == 0);
            frequences.resize(line.len(), HashMap::new());
        }
        for (i, c) in line.char_indices() {
            *frequences[i].entry(c).or_insert(0) += 1;
        }
    }

    let decoded : String = frequences.iter().map(|map| *map.iter().min_by_key(|&(_, c)| c).unwrap().0).collect();
    println!("{}", decoded);
}
