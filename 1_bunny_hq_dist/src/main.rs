use std::io::{self, Read};
use std::str;

fn main() {
    let mut x = 0i32;
    let mut y = 0i32;

    let mut cur_dir = 0u32;

    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    for instr in buffer.trim().split_whitespace() {
        let bytes = instr.as_bytes();
        let turn = bytes[0] as char;

        let dist_str = str::from_utf8(&bytes[1..]).unwrap();
        let dist = dist_str.trim_right_matches(',').parse::<i32>().unwrap();
        match turn {
            'R' => cur_dir = (cur_dir + 1) % 4,
            'L' => cur_dir = (cur_dir.wrapping_sub(1)) % 4,
            _ => panic!("help!"),
        }

        match cur_dir {
            0 | 2 => {
                if cur_dir == 0 { y += dist; } else { y -= dist; }
            }
            1 | 3 => {
                if cur_dir == 1 { x += dist; } else { x -= dist; }
            }
            _ => panic!("help! {}", cur_dir),
        }
    }

    println!("{}", x.abs() + y.abs());
}
