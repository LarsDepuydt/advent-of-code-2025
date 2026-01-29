use std::io;
use std::ops::RangeInclusive;

fn parse_input() -> (Vec<RangeInclusive<u64>>, Vec<u64>) {
    let mut ranges: Vec<RangeInclusive<u64>> = Vec::new();
    let mut input_ids: Vec<u64> = Vec::new();
    let mut ranges_parsed = false;

    loop {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input.");

        let trimmed = input.trim();
        if trimmed.is_empty() {
            if ranges_parsed {
                break;
            } else {
                ranges_parsed = true;
                continue;
            }
        }

        if ranges_parsed == false {
            if let Some((start_str, end_str)) = trimmed.split_once("-") {
                let start: u64 = start_str.parse().expect("Failed to parse start ID");
                let end: u64 = end_str.parse().expect("Failed to parse end ID");

                let new_range = start..=end;
                ranges.push(new_range);
            }
        } else {
            let new_input_id: u64 = trimmed.parse().expect("Failed parsing ID");
            input_ids.push(new_input_id);
        }
    }

    return (ranges, input_ids);
}

pub fn main() {
    println!("Input ingredients etc...");
    let mut fresh_ingredients_count = 0;

    let (input_ranges, input_ids) = parse_input();
    for input_id in input_ids {
        let is_fresh = input_ranges.iter().any(|range| range.contains(&input_id));
        if is_fresh {
            fresh_ingredients_count += 1;
        }
    }

    println!("Number of fresh ingredients: {fresh_ingredients_count}");
}
