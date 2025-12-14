#[cfg(test)]
mod tests;

use crate::parser::Range;
use crate::utils::vector_of_digits;

impl Range {
    pub fn find_repetitions_part2(&self) -> Vec<u64> {
        let lower_bound_digits = vector_of_digits(self.lower_bound);
        let upper_bound_digits = vector_of_digits(self.upper_bound);

        let split_lengths = self.find_split_lengths();

        for split_length in split_lengths {
            // the amount of times the found repetition has to be "duplicated" for it to fill the whole length (111111 -> 11 -> 3)
            let amount_of_repetition_replications = lower_bound_digits.len() / split_length as usize;
            println!("any found repetition has to be replicated {} times", amount_of_repetition_replications);

            let lower_bound_digits = lower_bound_digits[..split_length as usize].to_vec();
            let upper_bound_digits = upper_bound_digits[..split_length as usize].to_vec();
            println!("lower_bound_digits (to look at): {:?}", lower_bound_digits);
            println!("upper_bound_digits (to look at): {:?}", upper_bound_digits);

            let results = Self::find_split_repetitions(&lower_bound_digits, &upper_bound_digits)
                .into_iter()
                .map(|result| result.repeat(amount_of_repetition_replications as usize))
                .map(|result| result.parse::<u64>().unwrap())
                .filter(|result| self.result_is_valid(*result))
                .collect::<Vec<u64>>();
            println!("results: {:?}", results);

            return results;
        }

        vec![]
    }

    fn result_is_valid(&self, result: u64) -> bool {
        self.lower_bound <= result && self.upper_bound >= result
    }

    fn find_split_repetitions(lower_bound: &Vec<u64>, upper_bound: &Vec<u64>) -> Vec<String> {
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