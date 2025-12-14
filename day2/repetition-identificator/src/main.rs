use std::env;
use repetition_identificator::{solve_part1, solve_part2};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        std::process::exit(1);
    }
    let file_path = &args[1];
    let file_content = std::fs::read_to_string(file_path).expect("Failed to read the file");

    //19574776074
    println!("[PART-1] sum of invalid-id's: {}", solve_part1(&file_content));

    //25912654282
    println!("[PART-2] sum of invalid-id's: {}", solve_part2(&file_content));
}
