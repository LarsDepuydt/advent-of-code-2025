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

fn accessable_rolls(matrix: &Vec<Vec<char>>) -> (i32, Vec<Vec<char>>) {
    let mut reachable_count = 0;
    let mut new_matrix: Vec<Vec<char>> = Vec::new();

    for (y, row) in matrix.iter().enumerate() {
        new_matrix.push(Vec::new());

        for (x, &c) in row.iter().enumerate() {
            if c != '@' {
                new_matrix[y].push('.');
                continue;
            }

            let rolls_count = get_neighbours_count(&matrix, y, x);
            if rolls_count < 4 {
                reachable_count += 1;
                new_matrix[y].push('x');
            } else {
                new_matrix[y].push('@');
            }
        }
    }

    return (reachable_count, new_matrix);
}

pub fn main() {
    let mut position_matrix = parse_input();
    let mut total_reachable_count = 0;

    let mut r = 1;
    while r > 0 {
        let (reachable_count, new_matrix) = accessable_rolls(&position_matrix);
        total_reachable_count += reachable_count;
        r = reachable_count;
        position_matrix = new_matrix;
    }

    println!("Reachable paper rolls: {total_reachable_count}");
}
