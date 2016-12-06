use std::io;
use std::io::BufRead;
use std::collections::HashMap;

fn char_for_map(map: &HashMap<char, u32>) -> char {
    let mut vec : Vec<_> = map.iter().collect();
    vec.sort_by_key(|e| e.1);
    *vec.first().unwrap().0
}

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

    let decoded : String = frequences.iter().map(|map| char_for_map(map)).collect();
    println!("{}", decoded);
}
