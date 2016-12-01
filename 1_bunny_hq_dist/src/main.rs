use std::io::{self, Read};
use std::collections::HashSet;

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
struct Point {
    x: i32,
    y: i32,
}

enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn turn(self, dir: char) -> Direction {
        let mut as_int = self as u32;
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
            _ => panic!("inconceivable"),
        }
    }
}

const PART_1 : bool = true;

fn main() {
    let mut cur_pos = Point { x: 0, y: 0 };
    let mut cur_dir = Direction::North;
    let mut visited = HashSet::new();

    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    visited.insert(cur_pos);

    'outer: for instr in buffer.trim().split_whitespace() {
        let mut chars = instr.chars();
        let turn_dir = chars.next().unwrap();
        let dist = chars.as_str().trim_right_matches(',').parse::<i32>().unwrap();

        cur_dir = cur_dir.turn(turn_dir);

        for _ in 0..dist {
            match cur_dir {
                Direction::North => cur_pos.y += 1,
                Direction::South => cur_pos.y -= 1,
                Direction::East => cur_pos.x += 1,
                Direction::West => cur_pos.x -= 1,
            }

            if !PART_1 && visited.contains(&cur_pos) {
                break 'outer;
            }
            visited.insert(cur_pos);
        }
    }

    println!("{}", cur_pos.x.abs() + cur_pos.y.abs());
}
