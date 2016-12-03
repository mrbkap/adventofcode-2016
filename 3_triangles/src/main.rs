use std::io;
use std::io::BufRead;

fn main() {
    let stdin = io::stdin();

    let mut possible = 0u32;
    for line in stdin.lock().lines().filter_map(|l| l.ok()) {
        let mut nums : Vec<_> = line.split_whitespace().map(|s| s.parse::<i32>().unwrap()).collect();
        nums.sort();

        if nums[0] + nums[1] > nums [2] {
            possible += 1;
        }
    }

    println!("{}", possible);
}
