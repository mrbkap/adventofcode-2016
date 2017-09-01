extern crate regex;

use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Eq, PartialEq)]
struct Node {
    x: usize,
    y: usize,
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
    let mut rval: Vec<Vec<Node>> = (0..35).map(|_| Vec::with_capacity(26)).collect();
    for line in file.lines().filter_map(|l| l.ok()) {
        if first {
            // skip header line
            first = false;
            continue;
        }

        let caps = re.captures(&line).unwrap();
        let x = caps[1].parse::<usize>().unwrap();
        let y = caps[2].parse::<usize>().unwrap();
        let used = caps[4].parse::<u32>().unwrap();
        let avail = caps[5].parse::<u32>().unwrap();
        assert!(rval[x].len() == y);
        rval[x].push(Node {
            x: x,
            y: y,
            avail: avail,
            used: used,
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
}
