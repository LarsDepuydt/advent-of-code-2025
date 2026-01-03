use std::io;

fn parse_input() -> Vec<(char, i32)> {
    let mut movements: Vec<(char, i32)> = Vec::new();
    loop {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input.");

        let trimmed = input.trim();
        if trimmed.is_empty() {
            break;
        }

        let direction = trimmed.chars().next().unwrap();
        let value: i32 = trimmed[1..].parse().expect("Invalid number");

        movements.push((direction, value));
    }

    return movements;
}

fn main() {
    println!("Input the given lock combination (Press Enter on an empty line to finish):");

    const START_POSITION: i32 = 50;
    let movements = parse_input();

    let mut current_position = START_POSITION;
    let mut password = 0;
    for (direction, value) in &movements {
        let new_position = if *direction == 'L' {
            (current_position - value).rem_euclid(100)
        } else {
            (current_position + value).rem_euclid(100)
        };

        let passed_zero_count = if *direction == 'L' {
            let distance_to_zero = if current_position == 0 {
                100
            } else {
                current_position
            };
            if *value >= distance_to_zero {
                1 + (*value - distance_to_zero) / 100
            } else {
                0
            }
        } else {
            let distance_to_zero = 100 - current_position;
            if *value >= distance_to_zero {
                1 + (*value - distance_to_zero) / 100
            } else {
                0
            }
        };
        password = password + passed_zero_count;

        current_position = new_position;
    }

    println!("The password is: {password}")
}
