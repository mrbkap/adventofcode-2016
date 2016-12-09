extern crate regex;

use std::{fmt, io};
use std::io::BufRead;
use std::fmt::Write;

use regex::Regex;

const WIDTH : usize = 50;
const HEIGHT : usize = 6;

struct Screen {
    data: [[bool; WIDTH]; HEIGHT],
}

impl Screen {
    fn new() -> Screen {
        Screen { data: [[false; WIDTH]; HEIGHT] }
    }

    fn rect(&mut self, a: usize, b: usize) {
        for y in 0..b {
            for x in 0..a {
                self.data[y][x] = true;
            }
        }
    }

    fn rotate_row(&mut self, r: usize, mut b: usize) {
        b = b % WIDTH;
        let copy : Vec<_> = self.data[r].iter().cloned().collect();
        for i in 0..self.data[r].len() {
            self.data[r][i] = copy[(WIDTH - b + i) % WIDTH];
        }
    }

    fn rotate_col(&mut self, c: usize, mut b: usize) {
        b = b % HEIGHT;
        let mut copy = vec![false; HEIGHT];
        for i in 0..HEIGHT {
            copy[i] = self.data[i][c];
        }
        for i in 0..HEIGHT {
            self.data[i][c] = copy[(HEIGHT - b + i) % HEIGHT];
        }
    }
}

impl fmt::Display for Screen {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for r in 0..HEIGHT {
            for c in 0..WIDTH {
                let c = if self.data[r][c] { '#' } else { '.' };
                try!(f.write_char(c));
            }
            try!(f.write_char('\n'));
        }

        Ok(())
    }
}

struct CommandParser {
    rect_regex: Regex,
    rotate_col_regex: Regex,
    rotate_row_regex: Regex,
}

#[derive(Debug)]
enum Command {
    Rect(usize, usize),
    RotateCol(usize, usize),
    RotateRow(usize, usize),
}

impl<'a> CommandParser {
    fn new() -> CommandParser {
        CommandParser {
            rect_regex: Regex::new(r"rect (\d+)x(\d+)").unwrap(),
            rotate_col_regex: Regex::new(r"rotate column x=(\d+) by (\d+)").unwrap(),
            rotate_row_regex: Regex::new(r"rotate row y=(\d+) by (\d+)").unwrap(),
        }
    }

    fn parse(&self, line: &str) -> Command {
        if let Some(captures) = self.rect_regex.captures(&line) {
            let x = captures.at(1).unwrap().parse::<usize>().unwrap();
            let y = captures.at(2).unwrap().parse::<usize>().unwrap();
            return Command::Rect(x, y);
        }
        if let Some(captures) = self.rotate_col_regex.captures(&line) {
            let x = captures.at(1).unwrap().parse::<usize>().unwrap();
            let y = captures.at(2).unwrap().parse::<usize>().unwrap();

            return Command::RotateCol(x, y);
        }
        if let Some(captures) = self.rotate_row_regex.captures(&line) {
            let x = captures.at(1).unwrap().parse::<usize>().unwrap();
            let y = captures.at(2).unwrap().parse::<usize>().unwrap();

            return Command::RotateRow(x, y);
        }

        panic!("unrecognized line: {}", line);
    }
}

fn main() {
    let mut screen = Screen::new();
    let parser = CommandParser::new();
    let stdin = io::stdin();

    for line in stdin.lock().lines().filter_map(|l| l.ok()) {
        match parser.parse(&line) {
            Command::Rect(x, y) => screen.rect(x, y),
            Command::RotateCol(x, y) => screen.rotate_col(x, y),
            Command::RotateRow(x, y) => screen.rotate_row(x, y),
        }
    }

    println!("{}", screen);
    let mut cnt = 0;
    for r in screen.data.iter() {
        for c in r.iter() {
            if *c {
                cnt += 1;
            }
        }
    }
    println!("{}", cnt);
}
