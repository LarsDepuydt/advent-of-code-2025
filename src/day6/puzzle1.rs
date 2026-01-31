use std::io;

fn parse_input() -> (Vec<Vec<i64>>, Vec<String>) {
    let mut math_matrix: Vec<Vec<i64>> = Vec::new();
    let math_operations: Vec<String>;

    println!("Input cephalopod math:");

    loop {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input.");

        let trimmed = input.trim();

        let is_first_digit = trimmed.chars().next().map_or(false, |c| c.is_numeric());
        if is_first_digit {
            let splitted: Vec<i64> = trimmed
                .split_whitespace()
                .map(|s| s.parse::<i64>().unwrap())
                .collect();

            math_matrix.push(splitted);
        } else {
            math_operations = trimmed.split_whitespace().map(|s| s.to_string()).collect();
            break;
        }
    }

    return (math_matrix, math_operations);
}

pub fn main() {
    let (math_matrix, math_operations) = parse_input();
    let mut total_sum: i64 = 0;

    for i in 0..math_operations.len() {
        let mut column_numbers: Vec<i64> = Vec::new();
        for row in &math_matrix {
            let new_number = row[i];
            column_numbers.push(new_number);
        }

        let math_operation = &math_operations[i];
        match math_operation.as_str() {
            "+" => total_sum += column_numbers.iter().sum::<i64>(),
            "*" => total_sum += column_numbers.iter().product::<i64>(),
            _ => println!("Invaled math operation"),
        }
    }

    println!("Result: {total_sum}");
}
