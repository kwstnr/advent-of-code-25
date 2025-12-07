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

    let sum: u64 = parser::parse(&file_content)
        .into_iter()
        .map(|range| range.preprocess())
        .filter(|range| range.is_valid())
        .flat_map(|range| {
            range
                .find_repetitions()
                .into_iter()
                .map(|rep| rep.parse::<u64>().unwrap())
                .filter(move |rep| *rep >= range.lower_bound && *rep <= range.upper_bound)
        })
        .sum();

    println!("sum of invalid-id's: {}", sum);
}
