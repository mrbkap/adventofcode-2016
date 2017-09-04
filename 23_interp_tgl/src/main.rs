use std::io::{BufRead, BufReader};
use std::fs::File;

type Register = i32;

#[derive(Debug, Copy, Clone)]
enum RegOrImm {
    Register(Register),
    Imm(i32),
}

#[derive(Debug)]
enum Isn {
    Cpy(RegOrImm, RegOrImm),
    Inc(RegOrImm),
    Dec(RegOrImm),
    Mul(RegOrImm, RegOrImm, RegOrImm),
    Jnz(RegOrImm, RegOrImm),
    Tgl(RegOrImm),
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
        fn parse_reg_or_imm(w: &str) -> RegOrImm {
            match w.chars().next().unwrap() {
                '-' | '0'...'9' => RegOrImm::Imm(w.parse::<i32>().unwrap()),
                _ => RegOrImm::Register(CPU::to_register(w)),
            }
        }

        let words : Vec<_> = l.split_whitespace().collect();
        let isn = match words[0] {
            "cpy" => {
                let source = parse_reg_or_imm(words[1]);
                let dest = parse_reg_or_imm(words[2]);
                Isn::Cpy(source, dest)
            }
            "inc" => {
                let reg = parse_reg_or_imm(words[1]);
                Isn::Inc(reg)
            }
            "dec" => {
                let reg = parse_reg_or_imm(words[1]);
                Isn::Dec(reg)
            }
            "mul" => {
                let left = parse_reg_or_imm(words[1]);
                let right = parse_reg_or_imm(words[2]);
                let dest = parse_reg_or_imm(words[3]);
                Isn::Mul(left, right, dest)
            }
            "jnz" => {
                let source = parse_reg_or_imm(words[1]);
                let dist = parse_reg_or_imm(words[2]);
                Isn::Jnz(source, dist)
            }
            "tgl" => {
                let target = parse_reg_or_imm(words[1]);
                Isn::Tgl(target)
            }
            _ => { panic!("bad keyword {}", words[0]); }
        };
        self.isns.push(isn);
    }

    fn execute(&mut self) -> i32 {
        if self.isns.is_empty() {
            panic!("executing empty program");
        }

        let mut regs: Vec<i32> = vec![12, 0, 0, 0];
        fn resolve_copysrc(s: &RegOrImm, regs: &[i32]) -> i32 {
            match *s {
                RegOrImm::Imm(i) => i,
                RegOrImm::Register(r) => regs[r as usize],
            }
        }

        let mut pc = 0usize;
        loop {
            let (delta, change) = match self.isns[pc] {
                Isn::Cpy(ref s, RegOrImm::Register(d)) => {
                    regs[d as usize] = resolve_copysrc(s, &regs);
                    (1, None)
                }
                Isn::Inc(RegOrImm::Register(r)) => {
                    regs[r as usize] += 1;
                    (1, None)
                }
                Isn::Dec(RegOrImm::Register(r)) => {
                    regs[r as usize] -= 1;
                    (1, None)
                }
                Isn::Mul(ref left, ref right, RegOrImm::Register(dest)) => {
                    regs[dest as usize] = resolve_copysrc(left, &regs) * resolve_copysrc(right, &regs);
                    (1, None)
                }
                Isn::Jnz(ref s, ref dest) => {
                    let dist = resolve_copysrc(dest, &regs);
                    (if resolve_copysrc(s, &regs) != 0 {
                        dist as isize
                    } else {
                        1
                    }, None)
                }
                Isn::Tgl(RegOrImm::Register(targetreg)) => {
                    let target = regs[targetreg as usize] as isize;
                    let ipc = pc as isize;
                    if ipc + target >= 0 && ((ipc + target) as usize) < self.isns.len() {
                        let new_isn = match self.isns.get((ipc + target as isize) as usize).unwrap() {
                            &Isn::Dec(ref r) => Isn::Inc(r.clone()),
                            &Isn::Tgl(ref t) => Isn::Inc(t.clone()),
                            &Isn::Inc(ref r) => Isn::Dec(r.clone()),
                            &Isn::Cpy(ref cs, ref dest) => Isn::Jnz(cs.clone(), dest.clone()),
                            &Isn::Jnz(ref cs, ref dest) => Isn::Cpy(cs.clone(), dest.clone()),
                            &Isn::Mul(_, _, _) => panic!("uh oh"),
                        };
                        (1, Some(((ipc + target as isize) as usize, new_isn)))
                    } else {
                        (1, None)
                    }
                }
                _ => (1, None), // invalid instructions
            };

            if let Some((target, isn)) = change {
                self.isns[target] = isn;
            }
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
    let input = File::open("input.txt").unwrap();
    let br = BufReader::new(input);
    let mut cpu = CPU::new();
    for l in br.lines().filter_map(|l| l.ok()) {
        cpu.parse_line(&l);
    }

    let result = cpu.execute();
    println!("Final result: {:?}", result);
}
