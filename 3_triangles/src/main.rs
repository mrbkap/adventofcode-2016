use std::io;
use std::io::BufRead;

fn is_triangle(rr: u32, t : &mut [i32; 3]) -> u32 {
    t.sort();
    return rr + if t[0] + t[1] > t[2] { 1 } else { 0 }
}

fn main() {
    let stdin = io::stdin();

    let mut triangles : Vec<[i32; 3]> = vec![[0, 0, 0], [0, 0, 0], [0, 0, 0]];
    let mut possible = 0u32;
    let mut count = 0usize;
    for line in stdin.lock().lines().filter_map(|l| l.ok()) {
        let mut nums = line.split_whitespace().map(|s| s.parse::<i32>().unwrap());
        triangles[0][count] = nums.next().unwrap();
        triangles[1][count] = nums.next().unwrap();
        triangles[2][count] = nums.next().unwrap();
        count += 1;

        if count == 3 {
            possible += triangles.iter_mut().fold(0, is_triangle);
            count = 0;
        }
    }

    assert!(count == 0);
    println!("{}", possible);
}
