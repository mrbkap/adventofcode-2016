use std::io::BufRead;
use std::io;

type Register = u16;

#[derive(Debug)]
enum CopySource {
    Register(Register),
    Imm(i32),
}

#[derive(Debug)]
enum Isn {
    Cpy(CopySource, Register),
    Inc(Register),
    Dec(Register),
    Jnz(CopySource, i16),
}

#[derive(Debug)]
struct CPU {
    isns: Vec<Isn>,
}

impl CPU {
    fn new() -> CPU {
        CPU { isns: Vec::new() }
    }

    fn to_register(r: &str) -> Register {
        match r {
            "a" => 0,
            "b" => 1,
            "c" => 2,
            "d" => 3,
            _ => { panic!("bad register"); }
        }
    }

    fn parse_line(&mut self, l: &str) {
        fn parse_copysource(w: &str) -> CopySource {
            match w.chars().next().unwrap() {
                '0'...'9' => CopySource::Imm(w.parse::<i32>().unwrap()),
                _ => CopySource::Register(CPU::to_register(w)),
            }
        }

        let words : Vec<_> = l.split_whitespace().collect();
        let isn = match words[0] {
            "cpy" => {
                let source = parse_copysource(words[1]);
                let dest = CPU::to_register(words[2]);
                Isn::Cpy(source, dest)
            }
            "inc" => {
                let reg = CPU::to_register(words[1]);
                Isn::Inc(reg)
            }
            "dec" => {
                let reg = CPU::to_register(words[1]);
                Isn::Dec(reg)
            }
            "jnz" => {
                let source = parse_copysource(words[1]);
                let dist = words[2].parse::<i16>().unwrap();
                Isn::Jnz(source, dist)
            }
            _ => { panic!("bad keyword"); }
        };
        self.isns.push(isn);
    }

    fn execute(&mut self) -> i32 {
        if self.isns.is_empty() {
            panic!("executing empty program");
        }

        let mut regs: Vec<i32> = vec![0, 0, 0, 0];
        fn resolve_copysrc(s: &CopySource, regs: &[i32]) -> i32 {
            match *s {
                CopySource::Imm(i) => i,
                CopySource::Register(r) => regs[r as usize],
            }
        }

        let mut pc = 0usize;
        loop {
            let delta = match self.isns[pc] {
                Isn::Cpy(ref s, d) => {
                    regs[d as usize] = resolve_copysrc(s, &regs);
                    1
                }
                Isn::Inc(r) => {
                    regs[r as usize] += 1;
                    1
                }
                Isn::Dec(r) => {
                    regs[r as usize] -= 1;
                    1
                }
                Isn::Jnz(ref s, dist) => {
                    if resolve_copysrc(s, &regs) != 0 {
                        dist as isize
                    } else {
                        1
                    }
                }
            };

            pc = ((pc as isize) + delta) as usize;
            if pc >= self.isns.len() {
                break;
            }
        }

        println!("Registers at the end of execution: {:?}", regs);
        regs[0]
    }
}

fn main() {
    let stdin = io::stdin();
    let mut cpu = CPU::new();
    for l in stdin.lock().lines().filter_map(|l| l.ok()) {
        cpu.parse_line(&l);
    }

    let result = cpu.execute();
    println!("Final result: {:?}", result);
}
