use std::io;

fn parse_input() -> Vec<String> {
    println!("Input the battery banks input:");

    let mut battery_banks: Vec<String> = Vec::new();
    loop {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input.");

        let trimmed = input.trim();
        if trimmed.is_empty() {
            break;
        }

        battery_banks.push(trimmed.to_string());
    }

    return battery_banks;
}

pub fn main() {
    let battery_banks = parse_input();

    let mut joltage_sum: i64 = 0;

    for battery_string in &battery_banks {
        let digits: Vec<i32> = battery_string
            .chars()
            .filter_map(|c| c.to_digit(10))
            .map(|d| d as i32)
            .collect();

        if digits.len() < 2 {
            continue;
        }

        let str_len = digits.len();
        let prefix = &digits[..str_len - 1];

        if let Some(max_val) = prefix.iter().max() {
            let idx = prefix.iter().position(|x| x == max_val).unwrap();

            let next_val = digits[idx + 1..].iter().max();

            if let Some(next) = next_val {
                let biggest_joltage = format!("{max_val}{next}");
                let biggest_joltage_int: i64 = biggest_joltage
                    .parse()
                    .expect("Could not convert to number!");

                joltage_sum += biggest_joltage_int;
            }
        }
    }

    println!("Biggest Joltage possible: {joltage_sum}");
}
