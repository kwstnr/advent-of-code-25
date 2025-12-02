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

    let result = instructions
        .iter()
        .fold(Dial::new(), |acc, instruction| acc.rotate(instruction));

    println!("result: {0}", result.zero_counter);
}

