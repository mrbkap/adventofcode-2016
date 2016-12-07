use std::io;
use std::io::BufRead;
use std::collections::HashMap;

fn process_aba(l: &[u8], abas: &mut HashMap<[u8; 3], (bool, bool)>, negate: bool) -> bool {
    for w in l.windows(3) {
        if w[0] != w[1] && w[0] == w[2] {
            if negate {
                let bab = [w[1], w[0], w[1]];
                let entry = abas.entry(bab).or_insert((false, true));
                entry.1 = true;
                if entry.0 && entry.1 {
                    return true;
                }
            } else {
                let aba = [w[0], w[1], w[2]];
                let entry = abas.entry(aba).or_insert((true, false));
                entry.0 = true;
                if entry.0 && entry.1 {
                    return true;
                }
            }
        }
    }

    return false;
}

struct Splitter<'a> {
    pos: usize,
    string: &'a [u8],
    in_bracket: bool,
}

impl<'a> Splitter<'a> {
    fn split(s: &'a [u8]) -> Splitter<'a> {
        Splitter { pos: 0, string: s, in_bracket: false }
    }
}

impl<'a> Iterator for Splitter<'a> {
    type Item = (bool, &'a [u8]);
    fn next(&mut self) -> Option<Self::Item> {
        let bytes = self.string;
        if self.pos == bytes.len() {
            return None;
        }

        let mut end = self.pos + 1;
        let start = self.pos;
        let search = if self.in_bracket { ']' } else { '[' } as u8;
        loop {
            if end == bytes.len() || bytes[end] == search {
                self.pos = end;
                self.in_bracket = !self.in_bracket;
                return Some((!self.in_bracket, &bytes[start..end]));
            }

            end += 1;
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let mut cnt = 0u32;
    'next: for line in stdin.lock().lines().filter_map(|l| l.ok()) {
        let mut abas = HashMap::new();
        for (in_bracket, s) in Splitter::split(&line.as_bytes()) {
            if process_aba(&s, &mut abas, in_bracket) {
                cnt += 1;
                break;
            }
        }
    }
    println!("{}", cnt);
}
