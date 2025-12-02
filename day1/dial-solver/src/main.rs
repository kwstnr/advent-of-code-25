use std::env;
mod parser;
mod dial;

use dial::Dial;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        std::process::exit(1);
    }
    let file_path = &args[1];
    let file_content = std::fs::read_to_string(file_path).expect("Failed to read the file");
    let lines = file_content.lines().collect::<Vec<&str>>();
    let instructions = parser::parse(lines);
    println!("len of instructions: {0}", instructions.len());

    let result_part1 = instructions
        .iter()
        .fold(Dial::new(50), |acc, instruction| acc.rotate(instruction));
    let result_part2 = instructions
        .iter()
        .fold(Dial::new(50), |acc, instruction| acc.rotate_part2(instruction));

    println!("[Part 1] result: {0}", result_part1.zero_counter);
    println!("[Part 2] result: {0}", result_part2.zero_counter);
}

