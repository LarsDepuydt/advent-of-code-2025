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

fn find_largest_int(digits: &[i32], start_position: usize, end_offset: usize) -> (i32, usize) {
    let str_len = digits.len();

    if start_position >= str_len - end_offset {
        return (0, 0);
    }

    let search_end = str_len - end_offset;
    let prefix = &digits[start_position..search_end];

    if let Some(&max_val) = prefix.iter().max() {
        let idx = prefix.iter().position(|x| *x == max_val).unwrap();
        return (max_val, idx);
    }

    (0, 0)
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

        let mut start_position = 0;
        let mut biggest_joltage_str = String::new();
        for end_offset in (0..=11).rev() {
            let (biggest_joltage, pos) = find_largest_int(&digits, start_position, end_offset);
            biggest_joltage_str += &biggest_joltage.to_string();
            start_position += pos + 1;
        }
        eprintln!("{biggest_joltage_str}");

        let biggest_joltage_int: i64 = biggest_joltage_str.parse().expect("int conversion failed");
        joltage_sum += biggest_joltage_int;
    }

    println!("Biggest Joltage possible: {joltage_sum}");
}
