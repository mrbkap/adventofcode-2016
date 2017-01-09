extern crate crypto;

use crypto::md5;
use crypto::digest::Digest;
use std::collections::VecDeque;

fn main() {
    let buffer = "ahsbgdzn";
    const NUM_ITERS : u32 = 2016;

    let mut generator = md5::Md5::new();
    let mut x = 0u64;
    let mut pads = 0u32;
    let mut answer: [VecDeque<u64>; 16] =
        [VecDeque::new(),
         VecDeque::new(),
         VecDeque::new(),
         VecDeque::new(),
         VecDeque::new(),
         VecDeque::new(),
         VecDeque::new(),
         VecDeque::new(),
         VecDeque::new(),
         VecDeque::new(),
         VecDeque::new(),
         VecDeque::new(),
         VecDeque::new(),
         VecDeque::new(),
         VecDeque::new(),
         VecDeque::new()];

    let mut all_pads = Vec::new();

    'outer: loop {
        generator.input_str(&buffer);
        generator.input_str(&x.to_string());

        let mut result = generator.result_str();
        generator.reset();

        for _ in 0..NUM_ITERS {
            generator.input_str(&result);
            result = generator.result_str();
            generator.reset();
        }

        let mut last_c = result.chars().next().unwrap();
        let mut cnt = 1;
        let mut found_triplet = false;
        'inner: for c in result[1..].chars() {
            if c == last_c {
                cnt += 1;
                if cnt == 3 && !found_triplet {
                    found_triplet = true;
                    let idx = last_c.to_digit(16).unwrap() as usize;
                    answer[idx].push_back(x);
                    while x - answer[idx].front().unwrap() > 1000 {
                        answer[idx].pop_front();
                    }

                    if answer[idx].len() == 1 {
                        break 'inner;
                    }
                } else if cnt == 5 {
                    let idx = last_c.to_digit(16).unwrap() as usize;
                    while answer[idx].len() != 1 {
                        let cur = answer[idx].pop_front().unwrap();
                        assert!(x - cur <= 1000);
                        all_pads.push(cur);

                        pads += 1;
                        if pads == 64 {
                            all_pads.sort();
                            println!("{:?}", &all_pads[63]);
                            return;
                        }
                    }
                }
            } else {
                last_c = c;
                cnt = 1;
            }
        }
        x += 1;
    }
}
