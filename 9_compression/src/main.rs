use std::io;
use std::io::Read;

fn explode(input: &str, output: &mut String) -> usize {
    let x = input.find('x').unwrap();
    let len = input[..x].parse::<usize>().unwrap();

    let paren = input.find(')').unwrap();
    let num_times = input[(x + 1)..paren].parse::<usize>().unwrap();
    let repeated : &str = &input[(paren + 1)..(paren + 1 + len)];
    for _ in 0..num_times {
        output.push_str(repeated);
    }

    paren + 1 + len
}

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();

    stdin.lock().read_to_string(&mut input).unwrap();
    let mut output = String::new();
    let mut i = input.trim().chars();
    let mut advance: usize = 0;
    loop {
        match i.nth(advance) {
            Some('(') => {
                advance = explode(i.as_str(), &mut output);
            }
            Some(e @ _) => {
                output.push(e);
                advance = 0;
            }
            None => break,
        }
    }

    println!("{}", output.len());
}
