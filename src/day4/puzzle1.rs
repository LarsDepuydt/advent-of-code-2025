use std::io;

fn parse_input() -> Vec<Vec<char>> {
    println!("Input the rolls of paper layout:");

    let mut matrix: Vec<Vec<char>> = Vec::new();
    let mut i = 0;
    loop {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input.");

        let trimmed = input.trim();
        if trimmed.is_empty() {
            break;
        }

        matrix.push(Vec::new());
        for c in trimmed.chars() {
            matrix[i].push(c);
        }

        i += 1;
    }

    return matrix;
}

fn get_neighbours_count(matrix: &Vec<Vec<char>>, y: usize, x: usize) -> i32 {
    let mut count = 0;
    let y_len = matrix.len() as i32;
    let x_len = if y_len > 0 { matrix[0].len() as i32 } else { 0 };

    for i in -1..=1 {
        for j in -1..=1 {
            if i == 0 && j == 0 {
                continue;
            }

            let target_y = y as i32 + i;
            let target_x = x as i32 + j;

            if target_y >= 0 && target_y < y_len && target_x >= 0 && target_x < x_len {
                let c = matrix[target_y as usize][target_x as usize];
                if c == '@' {
                    count += 1;
                }
            }
        }
    }

    count
}
pub fn main() {
    let position_matrix = parse_input();
    let mut reachable_count = 0;

    for (y, row) in position_matrix.iter().enumerate() {
        for (x, &c) in row.iter().enumerate() {
            if c != '@' {
                continue;
            }

            let rolls_count = get_neighbours_count(&position_matrix, y, x);
            if rolls_count < 4 {
                reachable_count += 1;
            }
        }
    }

    println!("Reachable paper rolls: {reachable_count}");
}
