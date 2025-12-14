#[cfg(test)]
mod tests;

use crate::parser::Range;
use crate::utils::vector_of_digits;

impl Range {
    pub fn find_repetitions_part2(&self) -> Vec<u64> {
        vec![]
    }

    fn find_split_length(&self) -> Vec<u8> {
        let lower_bound_digits = vector_of_digits(self.lower_bound);
        let lower_bound_digits_length = lower_bound_digits.len();
        let half_length = lower_bound_digits_length / 2;

        (1..=half_length)
            .filter(|digit_count| {
                lower_bound_digits_length.rem_euclid(*digit_count) == 0
            })
            .map(|digit_count| digit_count as u8)
            .collect::<Vec<u8>>()
    }
}