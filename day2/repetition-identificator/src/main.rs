use std::env;

mod parser;
mod range_preprocessor;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        std::process::exit(1);
    }
    let file_path = &args[1];
    let file_content = std::fs::read_to_string(file_path).expect("Failed to read the file");

    let ranges = parser::parse(&file_content)
        .into_iter()
        .map(|range| range.preprocess());
}

