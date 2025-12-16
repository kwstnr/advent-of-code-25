#[cfg(test)]
mod tests;

use crate::parser::Range;
use crate::utils::vector_of_digits;
use std::collections::HashSet;

enum Restriction {
    LOWER(Vec<u8>),
    UPPER(Vec<u8>),
    BOTH(Vec<u8>, Vec<u8>),
    NONE(u8),
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
            },
            Restriction::NONE(len) => *len as usize,
        }
    }

    fn first_digit_range(&self) -> Vec<u8> {
        match self {
            Restriction::LOWER(v) => (v[0]..=9).collect::<Vec<u8>>(),
            Restriction::UPPER(v) => (0..=v[0]).collect::<Vec<u8>>(),
            Restriction::BOTH(v1, v2) => (v1[0]..=v2[0]).collect::<Vec<u8>>(),
            Restriction::NONE(_) => (0..9).collect::<Vec<u8>>(),
        }
    }

    fn popped_from_digit(&self, digit: u8) -> Self {
        match self {
            Restriction::BOTH(v_lower, v_upper) =>
                if v_lower[0] == v_upper[0] && digit == v_lower[0] { self.popped() }
                else if digit == v_lower[0] { Restriction::LOWER(v_lower[1..].to_vec()) }
                else if digit == v_upper[0] { Restriction::UPPER(v_upper[1..].to_vec()) }
                else { Restriction::NONE((v_lower.len() - 1).try_into().unwrap()) },
            Restriction::LOWER(v) if digit == v[0] => self.popped(),
            Restriction::UPPER(v) if digit == v[1] => self.popped(),
            Restriction::NONE(_) => self.popped(),
            _ => Restriction::NONE((self.len() - 1).try_into().unwrap())
        }
    }

    fn popped(&self) -> Self {
        match self {
            Restriction::LOWER(v) => Restriction::LOWER(v[1..].to_owned()),
            Restriction::UPPER(v) => Restriction::UPPER(v[1..].to_owned()),
            Restriction::BOTH(v1, v2) => Restriction::BOTH(v1[1..].to_owned(), v2[1..].to_owned()),
            Restriction::NONE(len) => Restriction::NONE(len - 1),
        }
    }

    fn from_index_digit_and_bounds(index: usize, digit: u8, lower_bound: &Vec<u8>, upper_bound: &Vec<u8>) -> Self {
            if index == 0 {
                if digit != upper_bound[0] {
                    Restriction::LOWER(lower_bound[1..].to_vec())
                } else {
                    Restriction::BOTH(
                        lower_bound[1..].to_vec(),
                        upper_bound[1..].to_vec()
                    )
                }
            } else if index == (upper_bound[0] - lower_bound[0]).into() {
                Restriction::UPPER(upper_bound[1..].to_vec())
            } else {
                Restriction::NONE((lower_bound.len() - 1).try_into().unwrap())
            }
    }
}

impl Range {
    fn result_is_valid(&self, result: u64) -> bool {
        self.lower_bound <= result && self.upper_bound >= result
    }

    pub fn half_split_length(&self) -> Result<u8, String> {
        let lower_bound_digits = vector_of_digits(self.lower_bound);
        let lower_bound_digits_len = lower_bound_digits.len();
        if lower_bound_digits_len.rem_euclid(2) != 0 {
            return Err(String::from("lower_bound_digits must have a even number of digits"));
        }

        Ok((lower_bound_digits_len / 2).try_into().unwrap())
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

    pub fn find_repetitions(&self, split_lengths: Vec<u8>) -> Vec<u64> {
        let lower_bound_digits = vector_of_digits(self.lower_bound);
        let upper_bound_digits = vector_of_digits(self.upper_bound);
        split_lengths
            .into_iter()
            .flat_map(|split_length| {
                let amount_of_repetition_replications =
                    lower_bound_digits.len() / split_length as usize;

                let lower_bound_digits = lower_bound_digits[..split_length as usize].to_vec();
                let upper_bound_digits = upper_bound_digits[..split_length as usize].to_vec();

                Self::find_split_repetitions(&lower_bound_digits, &upper_bound_digits)
                    .into_iter()
                    .map(|result| result.repeat(amount_of_repetition_replications))
                    .map(|result| result.parse::<u64>().unwrap())
                    .filter(|result| self.result_is_valid(*result))
                    .collect::<Vec<u64>>()
            })
            // deduplicate using HashSet
            .collect::<HashSet<u64>>()
            .into_iter()
            .collect::<Vec<u64>>()
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
                    Self::find_restricted_repetition(Restriction::from_index_digit_and_bounds(i, possible_digit, &lower_bound, &upper_bound))
                        .into_iter()
                        .map(|inner_result| format!("{}{}", possible_digit, inner_result))
                        .collect::<Vec<String>>()
                })
                .collect::<Vec<String>>()
        }
    }

    fn find_restricted_repetition(restriction: Restriction) -> Vec<String> {
        match restriction.len() {
            0 => vec![],
            1 => restriction.first_digit_range()
                .into_iter()
                .map(|digit| digit.to_string())
                .collect::<Vec<String>>(),
            _ => restriction.first_digit_range()
                .into_iter()
                .flat_map(|digit|
                    Range::find_restricted_repetition(restriction.popped_from_digit(digit))
                        .into_iter()
                        .map(|inner_result| format!("{}{}", digit, inner_result))
                        .collect::<Vec<String>>()
                )
                .collect::<Vec<String>>()
        }
    }
}