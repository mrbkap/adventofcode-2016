extern crate regex;

use std::io;
use std::io::BufRead;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use regex::Regex;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
enum Item {
    Chip(u16),
    Generator(u16),
}

#[derive(Clone, Debug)]
struct State {
    elevator: usize,
    floors: Vec<Vec<Item>>,
    ident: Vec<(u32, u32)>, // (# of chips, # of gens)
}

// The trick: This solution is a dumb BFS: for each level of the tree, we generate
// all legal solutions that we haven't seen yet -- but we prune (with the help of
// a comment on the Reddit thread) equivalent states: namely for two legal states
// where the number of chips and generators per floor is the same, we can consider
// those states equal.
impl std::hash::Hash for State {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.ident.hash(state);
        self.elevator.hash(state);
    }
}

impl PartialEq for State {
    fn eq(&self, other: &State) -> bool {
        self.ident == other.ident && self.elevator == other.elevator
    }
}

impl Eq for State {}

enum GenResult {
    Continue(Vec<State>),
    Complete,
}

impl State {
    fn gen_id(&mut self) {
        let mut map = vec![(0,0), (0,0), (0,0), (0,0)];
        for (idx, floor) in self.floors.iter().enumerate() {
            for &item in floor.iter() {
                match item {
                    Item::Generator(_) => map[idx].0 += 1,
                    Item::Chip(_) => map[idx].1 += 1,
                };
            }
        }
        self.ident = map;
    }
    fn is_complete(&self) -> bool {
        // All items on the last floor.
        return self.floors[0..self.floors.len() - 1].iter().fold(true, |rr, i| rr && i.is_empty());
    }
    fn is_valid(&self) -> bool {
        fn floor_is_valid(rr: bool, floor: &Vec<Item>) -> bool {
            if !rr {
                return false;
            }
            if floor.is_empty() {
                return true;
            }

            let mut chips = BinaryHeap::new();
            let mut gens = BinaryHeap::new();
            for i in floor.iter() {
                match i {
                    &Item::Chip(id) => chips.push(id),
                    &Item::Generator(id) => gens.push(id),
                }
            }

            if gens.is_empty() || chips.is_empty() {
                return true;
            }
            if chips.len() > gens.len() {
                return false;
            }

            let mut last_chip = chips.pop();
            let mut last_gen = gens.pop();
            loop {
                if last_gen != last_chip {
                    if gens.is_empty() {
                        return false;
                    }

                    last_gen = gens.pop();
                    continue;
                }

                if chips.is_empty() {
                    return true;
                }
                if gens.is_empty() {
                    return false;
                }
                last_chip = chips.pop();
                last_gen = gens.pop();
            }
        }

        let result = self.floors.iter().fold(true, floor_is_valid);
        return result;
    }

    fn gen_successors(&self, seen: &mut HashSet<State>) -> GenResult {
        fn single_elt(orig: &State, dir: isize,
                      seen: &mut HashSet<State>) -> GenResult {
            let mut succs = Vec::new();
            let floor_no = orig.elevator;
            let floor = &orig.floors[floor_no];
            for i in 0..floor.len() {
                let mut next = orig.clone();
                let elt = next.floors[floor_no].swap_remove(i);
                next.elevator = (next.elevator as isize + dir) as usize;
                next.floors[next.elevator].push(elt);
                next.gen_id();
                if !next.is_valid() || !seen.insert(next.clone()) {
                    continue;
                }
                seen.insert(next.clone());
                if next.is_complete() {
                    return GenResult::Complete;
                }
                succs.push(next);
            }

            return GenResult::Continue(succs);
        }
        fn double_elts(orig: &State, dir: isize,
                       seen: &mut HashSet<State>) -> GenResult {
            let mut succs = Vec::new();
            let floor_no = orig.elevator;
            let floor = &orig.floors[floor_no];
            for i in 0..floor.len() - 1 {
                for j in (i + 1)..floor.len() {
                    let mut next = orig.clone();
                    let elt = next.floors[floor_no].remove(i);
                    let elt2 = next.floors[floor_no].remove(j - 1);

                    next.elevator = (next.elevator as isize + dir) as usize;
                    next.floors[next.elevator].push(elt);
                    next.floors[next.elevator].push(elt2);
                    next.gen_id();
                    if !next.is_valid() || !seen.insert(next.clone()) {
                        continue;
                    }
                    if next.is_complete() {
                        return GenResult::Complete;
                    }
                    succs.push(next);
                }
            }

            return GenResult::Continue(succs);
        }

        let mut succs = Vec::new();
        if self.elevator < self.floors.len() - 1 {
            assert!(!self.floors[self.elevator].is_empty());
            match single_elt(self, 1, seen) {
                GenResult::Complete => { return GenResult::Complete; }
                GenResult::Continue(mut succ) => succs.append(&mut succ),
            }

            if self.floors[self.elevator].len() > 1 {
                match double_elts(self, 1, seen) {
                    GenResult::Complete => { return GenResult::Complete; }
                    GenResult::Continue(mut succ) => succs.append(&mut succ),
                }
            }
        }
        if self.elevator > 0 {
            assert!(!self.floors[self.elevator].is_empty());
            match single_elt(self, -1, seen) {
                GenResult::Complete => { return GenResult::Complete; }
                GenResult::Continue(mut succ) => succs.append(&mut succ),
            }

            if self.floors[self.elevator].len() > 1 {
                match double_elts(self, -1, seen) {
                    GenResult::Complete => { return GenResult::Complete; }
                    GenResult::Continue(mut succ) => succs.append(&mut succ),
                }
            }
        }

        GenResult::Continue(succs)
    }
}

fn parse() -> State {
    let gens_re = Regex::new(r"(\w+) generator").unwrap();
    let chips_re = Regex::new(r"(\w+)-compatible microchip").unwrap();

    let stdin = io::stdin();

    let mut initial_state = State { elevator: 0,
                                    floors: vec![Vec::new(),
                                                 Vec::new(),
                                                 Vec::new(),
                                                 Vec::new()],
                                    ident: Vec::new() };
    let mut floor = 0;
    let mut elt_names = HashMap::new();
    let mut num_elts = 0u16;
    for l in stdin.lock().lines().filter_map(|l| l.ok()) {
        let ref mut cur_floor = initial_state.floors[floor];
        floor += 1;

        for gens in gens_re.captures_iter(&l) {
            let elt = String::from(&gens[1]);
            let id = elt_names.entry(elt).or_insert_with(|| { num_elts += 1; num_elts - 1 });
            cur_floor.push(Item::Generator(*id));
        }

        for chips in chips_re.captures_iter(&l) {
            let elt = String::from(&chips[1]);
            let id = elt_names.entry(elt).or_insert_with(|| { num_elts += 1; num_elts - 1 });
            cur_floor.push(Item::Chip(*id));
        }
    }

    initial_state.gen_id();
    initial_state
}

fn main() {
    let initial = parse();
    let mut seen = HashSet::new();
    seen.insert(initial.clone());

    let mut todo = VecDeque::new();
    todo.push_back(initial);

    let mut iterations = 0;

    'outer: loop {
        iterations += 1;
        println!("{} {}", iterations, seen.len());
        let mut next_gen = Vec::new();
        while !todo.is_empty() {
            match todo.pop_front().unwrap().gen_successors(&mut seen) {
                GenResult::Complete => {
                    println!("{} iterations", iterations);
                    break 'outer;
                }
                GenResult::Continue(ref mut succs) => {
                    next_gen.append(succs);
                }
            }
        }

        assert!(!next_gen.is_empty());
        todo.extend(next_gen.drain(..));
    }
}
