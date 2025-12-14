#[cfg(test)]
mod tests;

use std::collections::HashSet;
use crate::parser::Range;
use crate::utils::vector_of_digits;

impl Range {
    pub fn find_repetitions_part2(&self) -> Vec<u64> {
        let lower_bound_digits = vector_of_digits(self.lower_bound);
        let upper_bound_digits = vector_of_digits(self.upper_bound);

        self.find_split_lengths()
            .into_iter()
            .flat_map(|split_length| {
                let amount_of_repetition_replications =
                    lower_bound_digits.len() / split_length as usize;

                let lower_bound_digits = lower_bound_digits[..split_length as usize].to_vec();
                let upper_bound_digits = upper_bound_digits[..split_length as usize].to_vec();

                Self::find_split_repetitions(&lower_bound_digits, &upper_bound_digits)
                    .into_iter()
                    .map(|result| result.repeat(amount_of_repetition_replications as usize))
                    .map(|result| result.parse::<u64>().unwrap())
                    .filter(|result| self.result_is_valid(*result))
                    .collect::<Vec<u64>>()
            })
            .collect::<HashSet<u64>>()
            .into_iter()
            .collect::<Vec<u64>>()
    }

    fn result_is_valid(&self, result: u64) -> bool {
        self.lower_bound <= result && self.upper_bound >= result
    }

    fn find_split_repetitions(lower_bound: &Vec<u8>, upper_bound: &Vec<u8>) -> Vec<String> {
        let possible_digits = lower_bound[0]..=upper_bound[0];

        if lower_bound.len() == 1 {
            possible_digits
                .map(|digits| digits.to_string())
                .collect::<Vec<String>>()
        } else {
            possible_digits
                .into_iter()
                .enumerate()
                .flat_map(|(i, possible_digit)| {
                    Self::find_repetitions_for_next_characters(
                        i,
                        possible_digit.into(),
                        &lower_bound,
                        &upper_bound,
                    )
                    .into_iter()
                    .map(|inner_result| format!("{}{}", possible_digit, inner_result))
                    .collect::<Vec<String>>()
                })
                .collect::<Vec<String>>()

        }
    }

    fn find_split_lengths(&self) -> Vec<u8> {
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