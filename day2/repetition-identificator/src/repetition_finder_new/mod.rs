#[cfg(test)]
mod tests;

use crate::parser::Range;
use crate::utils::vector_of_digits;

enum Restriction {
    LOWER(Vec<u8>),
    UPPER(Vec<u8>),
    BOTH(Vec<u8>, Vec<u8>),
}

impl Restriction {
    fn len(&self) -> usize {
        match self {
            Restriction::LOWER(v) => v.len(),
            Restriction::UPPER(v) => v.len(),
            Restriction::BOTH(v1, v2) => {
                if v1.len() != v2.len() {
                    panic!("ruh roh")
                } else {
                    v1.len()
                }
            }
        }
    }

    fn first_digit_range(&self) -> Vec<u8> {
        match self {
            Restriction::LOWER(v) => (v[0]..=9).collect::<Vec<u8>>(),
            Restriction::UPPER(v) => (0..=v[0]).collect::<Vec<u8>>(),
            Restriction::BOTH(v1, v2) => (v1[0]..=v2[0]).collect::<Vec<u8>>(),
        }
    }

    fn restricted_position(&self, first_digit_range: &Vec<u8>) -> usize {
        match self {
            Restriction::LOWER(_) => 0,
            Restriction::UPPER(_) => first_digit_range.len() - 1,
            Restriction::BOTH(_, _) => panic!("ruh roh"),
        }
    }

    fn popped(&self) -> Self {
        match self {
            Restriction::LOWER(v) => Restriction::LOWER(v[1..].to_owned()),
            Restriction::UPPER(v) => Restriction::UPPER(v[1..].to_owned()),
            Restriction::BOTH(v1, v2) => Restriction::BOTH(v1[1..].to_owned(), v2[1..].to_owned()),
        }
    }
}

impl Range {
    pub fn half_split_length(&self) -> Result<u8, String> {
        let lower_bound_digits = vector_of_digits(self.lower_bound);
        let lower_bound_digits_len = lower_bound_digits.len();
        if lower_bound_digits_len.rem_euclid(2) != 0 {
            return Err(String::from("lower_bound_digits must have a even number of digits"));
        }

        Ok(lower_bound_digits_len / 2)
    }

    pub fn find_split_lengths(&self) -> Vec<u8> {
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

    pub fn find_repetitions(&self, split_lengths: &Vec<u8>) -> Vec<u64> {
        // TODO: implement/copy from old repetition_finder
        vec![]
    }

    fn find_split_repetitions(lower_bound: &Vec<u8>, upper_bound: &Vec<u8>) -> Vec<String> {
        let possible_digits = lower_bound[0]..=upper_bound[0];

        if lower_bound.len() == 1 {
            possible_digits
                .map(|digit| digit.to_string())
                .collect::<Vec<String>>()
        } else {
            possible_digits
                .enumerate()
                .flat_map(|(i, possible_digit)| {
                    // TODO: implement method
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

    fn find_repetitions_for_next_characters(
        index_of_possible_digit: usize,
        current_possible_digit: u8,
        lower_half: &Vec<u8>,
        upper_half: &Vec<u8>
    ) -> Vec<String> {
        // TODO: implement/copy from old repetition_finder
        vec![]
    }

    // TODO: check if u64 is actually needed
    // TODO: add new Restriction::UNRESTRICTED(u64) with remaining half_length and unify those two functions
    fn find_unrestricted_repetition(remaining_half_length: u64) -> Vec<String> {
        // TODO: implement/copy from old repetition_finder
        vec![]
    }

    fn find_restricted_repetition(restriction: Restriction) -> Vec<String> {
        // TODO: implement/copy from old repetition_finder
        vec![]
    }
}