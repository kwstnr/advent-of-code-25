pub mod parser;
pub mod range_preprocessor;
pub mod repetition_finder_new;
pub mod utils;

pub fn solve_part1(input: &str) -> u64 {
    parser::parse(input)
        .into_iter()
        .map(|range| range.normalize_uneven())
        .flat_map(|result| match result {
            Ok(normalized_range) => vec![normalized_range],
            Err(_) => vec![],
        })
        .flat_map(|range| range.normalize_digit_significance())
        .flat_map(|range| range.find_repetitions(vec![range.half_split_length().unwrap()]))
        .sum()
}

pub fn solve_part2(input: &str) -> u64 {
    parser::parse(input)
        .into_iter()
        .flat_map(|range| range.normalize_digit_significance())
        .flat_map(|range| range.find_repetitions(range.find_split_lengths()))
        .sum()
}
