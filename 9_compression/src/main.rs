use std::io;
use std::io::Read;

#[derive(Debug)]
struct Expander {
    charsleft: usize,
    multiplier: usize,
    subchars: usize,
}

impl Expander {
    fn parse(input: &str) -> (usize, Expander) {
        let x = input.find('x').unwrap();
        let len = input[..x].parse::<usize>().unwrap();

        let paren = input.find(')').unwrap();
        let num_times = input[(x + 1)..paren].parse::<usize>().unwrap();

        (paren + 1, Expander { charsleft: len, multiplier: num_times, subchars: 0 })
    }
}

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.lock().read_to_string(&mut input).unwrap();

    let mut stack : Vec<Expander> = Vec::new();
    let mut numchars = 0;
    let mut i = input.trim().chars();
    let mut advance: usize = 0;
    loop {
        match i.nth(advance) {
            Some('(') => {
                let (n, e) = Expander::parse(i.as_str());
                advance = n;
                for e2 in stack.iter_mut().rev() {
                    e2.charsleft -= n + 1;
                    assert!(e2.charsleft > 0);
                }
                stack.push(e);
                continue;
            }
            Some(_) => {
                advance = 0;
            }
            None => break,
        }

        if stack.is_empty() {
            numchars += 1;
            continue;
        }

        stack.last_mut().unwrap().subchars += 1;

        for e in stack.iter_mut().rev() {
            e.charsleft -= 1;
        }

        let mut k = stack.len() - 1;
        loop {
            if stack[k].charsleft != 0 {
                break;
            }

            let curchars = stack[k].multiplier * stack[k].subchars;
            if k == 0 {
                numchars += curchars;
            } else {
                stack[k - 1].subchars += curchars;
            }
            stack.pop();
            if stack.is_empty() {
                break;
            }
            k -= 1;
        }
    }

    assert!(stack.is_empty());
    println!("{}", numchars);
}
