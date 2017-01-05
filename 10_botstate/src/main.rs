extern crate regex;

use regex::Regex;
use std::collections::HashMap;
use std::cmp::{max,min};
use std::io;
use std::io::BufRead;

#[derive(Debug)]
struct Bot {
    n: u32,
    a: Option<u32>,
    b: Option<u32>,
}

impl Bot {
    fn new(n: u32) -> Bot {
        Bot { n: n, a: None, b: None }
    }

    fn set(&mut self, chip: u32) {
        if self.a.is_none() {
            self.a = Some(chip);
            return;
        }

        self.b = Some(chip);
    }

    fn complete(&self) -> bool {
        return self.a.is_some() && self.b.is_some();
    }
}

#[derive(Copy, Clone)]
enum Target {
    Bot(u32),
    Output(u32),
}

#[derive(Copy, Clone)]
struct Isn {
    bot: u32,
    hi: Target,
    lo: Target,
}

struct BotState {
    state: HashMap<u32, Bot>,
    output: HashMap<u32, u32>,
    assn_re: Regex,
    set_re: Regex,
}

impl BotState {
    fn new() -> BotState {
        BotState { state: HashMap::new(), output: HashMap::new(),
                   assn_re: Regex::new(r"^bot (\d+) gives low to (bot|output) (\d+) and high to (bot|output) (\d+)$").unwrap(),
                   set_re: Regex::new(r"^value (\d+) goes to bot (\d+)$").unwrap() }
    }

    fn handle_line(&mut self, l: &str) -> Option<Isn> {
        if let Some(matches) = self.set_re.captures(&l) {
            let value = matches.get(1).unwrap().as_str().parse::<u32>().unwrap();
            let bot = matches.get(2).unwrap().as_str().parse::<u32>().unwrap();
            self.state.entry(bot).or_insert(Bot::new(bot)).set(value);
            return None;
        }
        if let Some(matches) = self.assn_re.captures(&l) {
            let giver = matches.get(1).unwrap().as_str().parse::<u32>().unwrap();
            let lo = if matches.get(2).unwrap().as_str() == "bot" {
                Target::Bot(matches.get(3).unwrap().as_str().parse::<u32>().unwrap())
            } else {
                Target::Output(matches.get(3).unwrap().as_str().parse::<u32>().unwrap())
            };

            let hi = if matches.get(4).unwrap().as_str() == "bot" {
                Target::Bot(matches.get(5).unwrap().as_str().parse::<u32>().unwrap())
            } else {
                Target::Output(matches.get(5).unwrap().as_str().parse::<u32>().unwrap())
            };
            return Some(Isn { bot: giver, lo: lo, hi: hi });
        }
        return None;
    }
}

fn main() {
    let stdin = io::stdin();
    let mut state = BotState::new();
    let mut isns = Vec::new();
    for l in stdin.lock().lines().filter_map(|l| l.ok()) {
        if let Some(isn) = state.handle_line(&l) {
            isns.push(isn);
        }
    }

    loop {
        let mut new_isns: Vec<Isn> = Vec::new();
        for isn in isns.iter() {
            let (a, b) = {
                let b = match state.state.get(&isn.bot) {
                    Some(b) if b.complete() => b,
                    _ => {
                       new_isns.push(isn.clone());
                        continue;
                    }
                };
                (b.a.unwrap(), b.b.unwrap())
            };

            let hi = max(a, b);
            let lo = min(a, b);

            if hi == 61 && lo == 17 {
                println!("bot {} compared the two", isn.bot);
            }
            match isn.lo {
                Target::Bot(b) => {
                    state.state.entry(b).or_insert(Bot::new(b)).set(lo);
                }
                Target::Output(o) => {
                    state.output.insert(o, lo);
                }
            }

            match isn.hi {
                Target::Bot(b) => {
                    state.state.entry(b).or_insert(Bot::new(b)).set(hi);
                }
                Target::Output(o) => {
                    state.output.insert(o, hi);
                }
            }
        }

        if new_isns.len() == 0 {
            break;
        }
        isns = new_isns;
    }
    println!("{:?}", state.output);
}
