pub mod parser;
pub mod range_preprocessor;
pub mod repetition_finder;
pub mod repetition_finder_part2;
pub mod utils;

pub fn solve_part1(input: &str) -> u64 {
    parser::parse(input)
        .into_iter()
        .map(|range| range.normalize_uneven())
        .flat_map(|result| match result {
            Ok(normalized_range) => vec![normalized_range],
            Err(_) => vec![],
        })
        // .flat_map(|range| range.normalize_digit_significance())
        .flat_map(|range| {
            println!("original (already normalized uneven): {:?}", range);
            let normalized = range.normalize_digit_significance();
            println!("normalized digit significance: {:?}", normalized);
            normalized
        })
        .flat_map(|range| range.find_repetitions())
        .sum()
}

pub fn solve_part2(input: &str) -> u64 {
    parser::parse(input)
        .into_iter()
        .flat_map(|range| range.normalize_digit_significance())
        .flat_map(|range| range.find_repetitions_part2())
        .sum()
}
