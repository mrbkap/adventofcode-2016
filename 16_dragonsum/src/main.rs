fn curve(a: String) -> String {
    let mut b = String::with_capacity(a.len());
    let mut rval = String::with_capacity(a.len() * 2 + 1);
    rval.push_str(&a);
    for c in a.chars().rev() {
        b.push(if c == '1' { '0' } else { '1' });
    }

    rval.push('0');
    rval.push_str(&b);
    rval
}

fn csum(s: &str) -> String {
    let mut csum = String::from(s);
    loop {
        let bytes : Vec<_> = csum.bytes().collect();
        csum.clear();
        for i in 0..(bytes.len() / 2) {
            csum.push(if bytes[i*2] == bytes[i*2+1] { '1' } else { '0' });
        }

        if csum.len() % 2 != 0 {
            break;
        }
    }
    csum
}

fn main() {
    let mut state = String::from("10111011111001111");

    while state.len() < 35651584 {
        state = curve(state);
    }

    println!("{}", csum(&state[0..35651584]));
}
