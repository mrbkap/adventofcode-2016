extern crate permutohedron;

use std::collections::{VecDeque, HashSet};
use std::io::{BufRead, BufReader};
use std::fs::File;
use permutohedron::LexicalPermutation;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Pos {
    x: isize,
    y: isize,
}

impl Pos {
    fn new(x: isize, y: isize) -> Pos {
        Pos { x: x, y: y }
    }

    fn unew(x: usize, y: usize) -> Pos {
        Pos { x: x as isize, y: y as isize }
    }

    fn add(&self, p: &Pos) -> Option<Pos> {
        if let Some(x) = self.x.checked_add(p.x) {
            if let Some(y) = self.y.checked_add(p.y) {
                return Some(Pos::new(x, y));
            }
        }
        None
    }
}

const DIRS: [Pos; 4] = [
    Pos { x: -1, y:  0 },
    Pos { x: 0, y: -1 },
    Pos { x: 0, y: 1 },
    Pos { x: 1, y: 0 },
];

#[derive(Debug)]
struct Maze {
    data: Vec<Vec<char>>,
    wires: Vec<Pos>,
}

impl Maze {
    fn parse(input: &str) -> Maze {
        let mut maze = Maze {
            data: Vec::new(),
            wires: Vec::new(),
        };
        let f = File::open(input).unwrap();
        let input = BufReader::new(f);
        for l in input.lines().filter_map(|l| l.ok()) {
            let mut row = Vec::with_capacity(l.len());
            for c in l.chars() {
                row.push(c);
                if let Some(digit) = c.to_digit(10) {
                    let num = digit as usize;
                    let pos = Pos::unew(row.len() - 1, maze.data.len());
                    if num >= maze.wires.len() {
                        maze.wires.resize(num + 1, pos);
                    } else {
                        maze.wires[num] = pos;
                    }
                }
            }
            maze.data.push(row);
        }
        
        maze
    }

    fn get(&self, p: &Pos) -> char {
        self.data[p.y as usize][p.x as usize]
    }

    fn len_between(&self, start: usize, end: usize) -> u32 {
        let mut todo = VecDeque::new();
        let mut visited = HashSet::new();
        let goal = self.wires[end].clone();
        todo.push_back((self.wires[start].clone(), 0));
        while !todo.is_empty() {
            let (cur, len) = todo.pop_front().unwrap();
            for dir in DIRS.iter() {
                if let Some(next) = cur.add(dir) {
                    if next == goal {
                        return len + 1;
                    }
                    if self.get(&next) != '#' && !visited.contains(&next) {
                        visited.insert(next.clone());
                        todo.push_back((next, len + 1));
                    }
                }
            }
        }

        panic!("help?");
    }
}

fn main() {
    let maze = Maze::parse("input.txt");
    let mut weights = (0..maze.wires.len()).map(|_| vec![0; maze.wires.len()]).collect::<Vec<_>>();
    for i in 0..maze.wires.len() {
        for j in (i + 1)..maze.wires.len() {
            let len = maze.len_between(i, j);
            weights[i][j] = len;
            weights[j][i] = len;
        }
    }

    let mut best = std::u32::MAX;
    let mut p = (0..weights.len()).collect::<Vec<_>>();
    loop {
        if p[0] == 0 {
            let cur = (0..(p.len() - 1)).fold(0, |rr, c| rr + weights[p[c]][p[c + 1]]);
            if cur < best {
                best = cur;
            }
        }

        if !p.next_permutation() {
            break;
        }
    }

    println!("The shortest path was {}", best);
}
