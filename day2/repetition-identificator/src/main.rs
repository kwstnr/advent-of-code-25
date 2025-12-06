use std::env;

mod parser;
mod range_preprocessor;
mod repetition_finder;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        std::process::exit(1);
    }
    let file_path = &args[1];
    let file_content = std::fs::read_to_string(file_path).expect("Failed to read the file");

    let range_count: u64 = parser::parse(&file_content)
        .into_iter()
        .map(|range| range.preprocess())
        .filter(|range| range.is_valid())
        .map(|range|range.count_repetitions())
        .sum();
    let ranges: Vec<String> = parser::parse(&file_content)
        .into_iter()
        .map(|range| range.preprocess())
        .filter(|range| range.is_valid())
        .flat_map(|range| {
            let repetitions = range.find_repetitions();
            println!("Found {} repetitions in range {}-{}", repetitions.len(), range.lower_bound, range.upper_bound);
            repetitions
        })
        .collect();

    println!("Total repetitions found: {}", range_count);
}

