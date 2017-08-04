extern crate crypto;

use crypto::md5::Md5;
use crypto::digest::Digest;
use std::collections::{HashSet, VecDeque};
use std::io::Read;

#[derive(PartialEq, Eq, Hash, Clone)]
struct Point {
    x: u32,
    y: u32,
    path: String,
}

impl Point {
    fn new(x: u32, y: u32, path: &str) -> Point {
        Point { x: x, y: y, path: String::from(path), }
    }
}

struct Maze {
    gen: Md5,
}

impl Maze {
    fn new() -> Maze {
        Maze {
            gen: Md5::new(),
        }
    }

    fn get_exits(&mut self, point: &Point) -> String {
        self.gen.input_str(&point.path);

        let result = self.gen.result_str();
        let output = result.as_bytes();
        self.gen.reset();

        fn push_exit(s: &mut String, c: char, e: char) {
            fn is_exit(c: char) -> bool {
                match c {
                    'b'...'f' => true,
                    _ => false,
                }
            }

            if is_exit(c) {
                s.push(e);
            }
        }

        let mut exits = String::new();
        push_exit(&mut exits, output[0] as char, 'U');
        push_exit(&mut exits, output[1] as char, 'D');
        push_exit(&mut exits, output[2] as char, 'L');
        push_exit(&mut exits, output[3] as char, 'R');
        exits
    }

    fn solve(&mut self, passcode: &str) -> u32 {
        let mut visited: HashSet<Point> = HashSet::new();
        let mut work_list: VecDeque<Point> = VecDeque::new();
        let mut max_len = 0usize;
        work_list.push_back(Point::new(0, 0, passcode));

        loop {
            if work_list.is_empty() {
                return max_len as u32;
            }

            let cur_pos = work_list.pop_front().unwrap();
            if cur_pos.x == 3 && cur_pos.y == 3 {
                if cur_pos.path.len() - passcode.len() > max_len {
                    max_len = cur_pos.path.len() - passcode.len();
                }
                continue;
            }

            visited.insert(cur_pos.clone());
            let exits = self.get_exits(&cur_pos);
            for door in exits.chars() {
                let mut path = cur_pos.path.clone();
                path.push(door);
                let next = match door {
                    'U' => {
                        if cur_pos.y > 0 {
                            Some(Point::new(cur_pos.x, cur_pos.y - 1, &path))
                        } else {
                            None
                        }
                    }
                    'D' => {
                        if cur_pos.y < 3 {
                            Some(Point::new(cur_pos.x, cur_pos.y + 1, &path))
                        } else {
                            None
                        }
                    }
                    'L' => {
                        if cur_pos.x > 0 {
                            Some(Point::new(cur_pos.x - 1, cur_pos.y, &path))
                        } else {
                            None
                        }
                    }
                    'R' => {
                        if cur_pos.x < 3 {
                            Some(Point::new(cur_pos.x + 1, cur_pos.y, &path))
                        } else {
                            None
                        }
                    }
                    _ => panic!("bad exit"),
                };

                if next.is_some() && !visited.contains(&next.as_ref().unwrap()) {
                    work_list.push_back(next.unwrap());
                }
            }
        }
    }
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let passcode = input.trim();
    let mut maze = Maze::new();
    let rv = maze.solve(&passcode);
    println!("{}", rv);
}

#[test]
fn do_tests() {
    let mut maze = Maze::new();

    assert_eq!(maze.solve("ihgpwlah"), 370);
    assert_eq!(maze.solve("kglvqrro"), 492);
    assert_eq!(maze.solve("ulqzkmiv"), 830);
    assert_eq!(maze.solve("vkjiggvb"), 392);
}
