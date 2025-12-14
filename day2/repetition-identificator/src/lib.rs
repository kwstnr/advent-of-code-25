pub mod parser;
pub mod range_preprocessor;
pub mod repetition_finder;
pub mod repetition_finder_part2;
pub mod utils;

pub fn solve_part1(input: &str) -> u64 {
    parser::parse(input)
        .into_iter()
        .map(|range| range.preprocess())
        .filter(|range| range.is_valid())
        .flat_map(|range| range.find_repetitions())
        .sum()
}

pub fn solve_part2(input: &str) -> u64 {
    parser::parse(input)
        .into_iter()
        .flat_map(|range| range.preprocess_part2())
        .flat_map(|range| range.find_repetitions_part2())
        .sum()
}
