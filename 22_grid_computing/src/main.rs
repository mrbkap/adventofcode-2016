extern crate regex;

use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Eq, PartialEq)]
struct Node {
    x: usize,
    y: usize,
    size: u32,
    avail: u32,
    used: u32,
}

fn parse() -> Vec<Vec<Node>> {
    let f = File::open("input.txt").unwrap();
    let file = BufReader::new(f);
    let re = Regex::new(
        r"/dev/grid/node-x(\d+)-y(\d+)\s+(\d+)T\s+(\d+)T\s+(\d+)T\s+(\d+)%",
    ).unwrap();
    let mut first = true;
    let mut rval: Vec<Vec<Node>> = (0..27).map(|_| Vec::with_capacity(36)).collect();
    for line in file.lines().filter_map(|l| l.ok()) {
        if first {
            // skip header line
            first = false;
            continue;
        }

        let caps = re.captures(&line).unwrap();
        let x = caps[1].parse::<usize>().unwrap();
        let y = caps[2].parse::<usize>().unwrap();
        let size = caps[3].parse::<u32>().unwrap();
        let used = caps[4].parse::<u32>().unwrap();
        let avail = caps[5].parse::<u32>().unwrap();
        assert!(rval[y].len() == x);
        rval[y].push(Node {
            x: x,
            y: y,
            avail: avail,
            used: used,
            size: size,
        });
    }

    rval
}

fn main() {
    let nodes = parse();
    let viable = nodes.iter().fold(0, |v, r| {
        v + r.iter().filter(|n| n.used != 0).fold(0, |vv, n| {
            vv + nodes.iter().fold(0, |v, r| {
                v + r.iter().filter(|nn| *nn != n && n.used <= nn.avail).count()
            })
        })
    });
    println!("{}", viable);

    // part 2
    for x in 0..nodes[0].len() + 1 {
        print!("{:02} ", x);
    }
    println!("");
    for (y, row) in nodes.iter().enumerate() {
        print!("{:2}", y);
        for (x, node) in row.iter().enumerate() {
            if y == 0 && x == row.len() - 1 {
                print!(" G ");
            } else if node.used == 0 {
                print!(" _ ");
            } else if node.size > 100 {
                print!(" # ");
            } else {
                print!(" . ");
            }
        }
        println!("");
    }
}
