use std::io;

fn parse_input() -> Vec<Vec<char>> {
    let mut math_matrix: Vec<Vec<char>> = Vec::new();

    println!("Input cephalopod math:");

    loop {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input.");

        if input.trim().is_empty() {
            break;
        }

        math_matrix.push(input.chars().collect());
    }

    return math_matrix;
}

pub fn main() {
    let math_matrix = parse_input();
    let mut total_sum: i64 = 0;

    let char_length = math_matrix[0].len();

    let mut new_numbers: Vec<i64> = Vec::new();
    for i in (0..char_length).rev() {
        let mut new_number_str: String = String::new();
        let mut operator: char = ' ';

        for j in 0..math_matrix.len() {
            let new_char = math_matrix[j][i];
            if new_char.is_numeric() {
                new_number_str.push(new_char);
            } else {
                if new_char != ' ' {
                    operator = new_char;
                }
            }
        }

        if !new_number_str.is_empty() {
            let new_number: i64 = new_number_str.parse().expect("Failed to convert to number");
            new_numbers.push(new_number);
        }

        if operator != ' ' {
            match operator {
                '+' => total_sum += new_numbers.iter().sum::<i64>(),
                '*' => total_sum += new_numbers.iter().product::<i64>(),
                _ => println!("Invaled math operation"),
            }

            new_numbers = Vec::new();
        }
    }

    println!("Result: {total_sum}");
}
