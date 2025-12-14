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
        }

        vec![]
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