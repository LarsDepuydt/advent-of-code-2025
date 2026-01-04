use std::io;

fn parse_input() -> String {
    println!("Input string:");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input.");

    return input;
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
                let len = s.len();

                if len % 2 != 0 {
                    continue;
                }

                let half = len / 2;
                let (first_half, second_half) = s.split_at(half);

                if first_half == second_half {
                    sum += i;
                }
            }
        }
    }

    println!("Sum: {sum}");
}
