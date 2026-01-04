use std::io;
mod day1;

fn main() {
    println!("Choose the program to run:");

    let mut program_number = String::new();
    io::stdin()
        .read_line(&mut program_number)
        .expect("Failed to read input.");

    match program_number.trim() {
        "11" => day1::puzzle1::main(),
        "12" => day1::puzzle2::main(),
        _ => println!("Invalid choice"),
    }
}
