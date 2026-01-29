use std::io;
use std::ops::RangeInclusive;

fn parse_input() -> Vec<RangeInclusive<u64>> {
    let mut ranges: Vec<RangeInclusive<u64>> = Vec::new();

    loop {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input.");

        let trimmed = input.trim();
        if trimmed.is_empty() {
            break;
        }

        if let Some((start_str, end_str)) = trimmed.split_once("-") {
            let start: u64 = start_str.parse().expect("Failed to parse start ID");
            let end: u64 = end_str.parse().expect("Failed to parse end ID");

            let new_range = start..=end;
            ranges.push(new_range);
        }
    }

    return ranges;
}

fn merge_ranges(mut ranges: Vec<RangeInclusive<u64>>) -> Vec<RangeInclusive<u64>> {
    ranges.sort_by_key(|r| *r.start());

    let mut merged_ranges: Vec<RangeInclusive<u64>> = Vec::new();

    let mut current = ranges[0].clone();
    for next in ranges.into_iter().skip(1) {
        if next.start() <= current.end() {
            current = *current.start()..=(*current.end().max(next.end()));
        } else {
            merged_ranges.push(current);
            current = next;
        }
    }

    merged_ranges.push(current);
    merged_ranges
}

pub fn main() {
    println!("Input ingredients etc...");
    let mut fresh_ingredients_count = 0;

    let input_ranges = parse_input();
    let merged_ranges = merge_ranges(input_ranges);
    for range in merged_ranges {
        let range_count = (range.end() - range.start()) + 1;
        fresh_ingredients_count += range_count;
    }

    println!("Number of fresh ingredients: {fresh_ingredients_count}");
}
