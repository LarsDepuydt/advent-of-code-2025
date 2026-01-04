use std::io;

fn parse_input() -> String {
    println!("Input string:");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input.");

    return input;
}

fn is_pattern(s: &str) -> bool {
    let len = s.len();
    if len < 2 {
        return false;
    }

    for p_len in 1..=(len / 2) {
        if len % p_len == 0 {
            let pattern = &s[0..p_len];
            if pattern.repeat(len / p_len) == s {
                return true;
            }
        }
    }

    return false;
}

pub fn main() {
    let input_data = parse_input();
    let mut sum: i128 = 0;

    for item in input_data.split(",") {
        if let Some((start_str, end_str)) = item.split_once('-') {
            let start: i128 = start_str.trim().parse().expect("Invalid start");
            let end: i128 = end_str.trim().parse().expect("Invalid end");

            for i in start..=end {
                let s = i.to_string();

                if is_pattern(&s) {
                    sum += i;
                }
            }
        }
    }

    println!("Sum: {sum}");
}
