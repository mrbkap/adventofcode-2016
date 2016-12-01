use std::io::{self, Read};
use std::str;
use std::collections::HashSet;

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(PartialEq)]
enum Direction {
    North,
    East,
    South,
    West,
}

fn turn(current: Direction, dir: char) -> Direction {
    let mut as_int = current as u32;
    match dir {
        'R' => as_int = (as_int + 1) % 4,
        'L' => as_int = as_int.wrapping_sub(1) % 4,
        _ => panic!("bad dir!"),
    }

    match as_int {
        0 => Direction::North,
        1 => Direction::East,
        2 => Direction::South,
        3 => Direction::West,
        _ => panic!("inconceivable {}", as_int),
    }
}

fn main() {
    let mut cur_pos = Point { x: 0, y: 0 };
    let mut cur_dir = Direction::North;
    let mut visited = HashSet::new();
    let mut done = false;

    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    visited.insert(cur_pos);

    for instr in buffer.trim().split_whitespace() {
        let bytes = instr.as_bytes();
        let turn_dir = bytes[0] as char;

        let dist_str = str::from_utf8(&bytes[1..]).unwrap();
        let dist = dist_str.trim_right_matches(',').parse::<i32>().unwrap();
        cur_dir = turn(cur_dir, turn_dir);

        for _ in 0..dist {
            match cur_dir {
                Direction::North | Direction::South => {
                    if cur_dir == Direction::North { cur_pos.y += 1; } else { cur_pos.y -= 1; }
                }
                Direction::East | Direction::West => {
                    if cur_dir == Direction::East { cur_pos.x += 1; } else { cur_pos.x -= 1; }
                }
            }

            if visited.contains(&cur_pos) {
                done = true;
                break;
            }
            visited.insert(cur_pos);
        }

        if done {
            break;
        }
    }

    println!("{}", cur_pos.x.abs() + cur_pos.y.abs());
}
