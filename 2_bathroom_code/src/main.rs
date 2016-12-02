use std::io;
use std::io::BufRead;

struct Position {
    x: usize,
    y: usize,
}

fn main() {
    let keypad = vec![vec![1, 2, 3],
                      vec![4, 5, 6],
                      vec![7, 8, 9]];
    let mut cur_pos = Position { x: 1, y: 1 };
    let stdin = io::stdin();

    for wline in stdin.lock().lines() {
        let uline = wline.unwrap();
        let line = uline.trim();
        for inst in line.chars() {
            match inst {
                'U' => if cur_pos.y > 0 { cur_pos.y -= 1; },
                'D' => if cur_pos.y < 2 { cur_pos.y += 1; },
                'L' => if cur_pos.x > 0 { cur_pos.x -= 1; },
                'R' => if cur_pos.x < 2 { cur_pos.x += 1; },
                _ => panic!("bad instruction {}", inst),
            }
        }

        print!("{}", keypad[cur_pos.y][cur_pos.x]);
    }
    print!("\n");
}
