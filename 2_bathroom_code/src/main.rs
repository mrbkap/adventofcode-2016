use std::io;
use std::io::BufRead;

#[derive(Copy, Clone)]
struct Position {
    x: usize,
    y: usize,
}

fn main() {
    let keypad = vec![vec!['\0', '\0', '1', '\0', '\0'],
                      vec!['\0',  '2', '3',  '4', '\0'],
                      vec![ '5',  '6', '7',  '8',  '9'],
                      vec!['\0',  'A', 'B',  'C', '\0'],
                      vec!['\0', '\0', 'D', '\0', '\0']];
    let mut cur_pos = Position { x: 0, y: 2 };
    let stdin = io::stdin();

    for line in stdin.lock().lines().filter_map(|l| l.ok()).filter(|l| !l.is_empty()) {
        for inst in line.chars() {
            let orig_pos = cur_pos;
            match inst {
                'U' => if cur_pos.y > 0 { cur_pos.y -= 1; },
                'D' => if cur_pos.y < keypad.len() - 1 { cur_pos.y += 1; },
                'L' => if cur_pos.x > 0 { cur_pos.x -= 1; },
                'R' => if cur_pos.x < keypad.len() - 1 { cur_pos.x += 1; },
                _ => panic!("bad instruction {}", inst),
            }

            if keypad[cur_pos.y][cur_pos.x] == '\0' {
                cur_pos = orig_pos;
            }
        }

        print!("{}", keypad[cur_pos.y][cur_pos.x]);
    }
    print!("\n");
}
