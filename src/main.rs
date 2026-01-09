use std::io;
mod day1;
mod day2;
mod day3;

fn main() {
    println!("Choose the program to run:");

    let mut program_number = String::new();
    io::stdin()
        .read_line(&mut program_number)
        .expect("Failed to read input.");

    match program_number.trim() {
        "11" => day1::puzzle1::main(),
        "12" => day1::puzzle2::main(),
        "21" => day2::puzzle1::main(),
        "22" => day2::puzzle2::main(),
        "31" => day3::puzzle1::main(),
        "32" => day3::puzzle2::main(),
        _ => println!("Invalid choice"),
    }
}
