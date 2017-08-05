fn parse_row(row: &str) -> Vec<u8> {
    row.chars().map(|c| if c == '^' { 1u8 } else { 0u8 }).collect()
}

fn row_to_printable(row: &[u8]) -> String {
    row.iter().map(|i| if *i == 1 { '^' } else { '.' }).collect()
}

fn count_safe(row: &[u8]) -> u32 {
    let mut safe = 0;
    for c in row {
        if *c == 0 {
            safe += 1;
        }
    }
    safe
}

fn tile_num(floor: &[u8], idx: usize) -> u8 {
    let mut rv = floor[idx] << 1;
    if idx > 0 {
        rv |= floor[idx - 1] << 2;
    }
    if idx < floor.len() - 1 {
        rv |= floor[idx + 1];
    }
    rv
}

fn next_row(floor: &[u8]) -> Vec<u8> {
    floor.iter().enumerate().map(|(ref idx, _)| {
        match tile_num(&floor, *idx) {
            0b110 | 0b011 | 0b100 | 0b001 => 1,
            _ => 0
        }
    }).collect()
}

fn main() {
    let input = ".^.^..^......^^^^^...^^^...^...^....^^.^...^.^^^^....^...^^.^^^...^^^^.^^.^.^^..^.^^^..^^^^^^.^^^..^";

    let mut cur_row = parse_row(&input);
    let mut safe = count_safe(&cur_row);

    for _ in 1..400000 {
        cur_row = next_row(&cur_row);
        safe += count_safe(&cur_row);
    }


    println!("I found {} safe", safe);
}
