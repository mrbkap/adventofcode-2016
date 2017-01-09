#[derive(Clone, Debug)]
struct Disk {
    max_pos: u16,
    cur_pos: u16,
}

impl Disk {
    fn new(max: u16, cur: u16) -> Disk {
        Disk { max_pos: max, cur_pos: cur }
    }
    fn step(&mut self) -> bool {
        self.cur_pos += 1;
        self.cur_pos %= self.max_pos;
        return self.cur_pos == 0;
    }
}

#[derive(Clone, Debug)]
struct State {
    t: u32,
    disks: Vec<Disk>,
}

impl State {
    fn new(v: Vec<Disk>) -> State {
        State { t: 0, disks: v }
    }

    fn step(&mut self) {
        self.t += 1;
        for d in self.disks.iter_mut() {
            d.step();
        }
    }

    fn initial_open(&self) -> bool {
        self.disks[0].cur_pos == 0
    }
}

fn find_answer(cur: State) -> u32 {
    fn wins(cur: &State) -> bool {
        let mut next = cur.clone();
        for i in 0..next.disks.len() {
            if next.disks[i].cur_pos != 0 {
                return false;
            }
            next.step();
        }

        return true;
    }

    let mut cur = cur.clone();
    loop {
        cur.step();
        if !cur.initial_open() {
            continue;
        }
        if wins(&cur) {
            return cur.t - 1;
        }
    }
}

fn main() {
    let initial = State::new(vec![ Disk::new(17, 1),
                                   Disk::new(7, 0),
                                   Disk::new(19, 2),
                                   Disk::new(5, 0),
                                   Disk::new(3, 0),
                                   Disk::new(13, 5),
                                   Disk::new(11, 0) ]);
    let winner = find_answer(initial);
    println!("found {}", winner);
}
