use std::io;
use std::io::BufRead;

fn has_abba(l: &[u8]) -> bool {
    for w in l.windows(4) {
        if w[0] != w[1] && w[1] == w[2] && w[0] == w[3] {
            return true;
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
    fn split(s: &'a [u8]) -> Splitter {
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
        let mut maybe_valid = false;
        let bytes = line.as_bytes();
        for (in_bracket, s) in Splitter::split(&bytes) {
            if has_abba(s) {
                if in_bracket {
                    continue 'next;
                }

                maybe_valid = true;
            }
        }

        if maybe_valid {
            cnt += 1;
        }
    }
    println!("{}", cnt);
}
