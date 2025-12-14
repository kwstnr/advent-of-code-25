#[cfg(test)]
mod tests;

use crate::parser::Range;
use crate::utils;

/// Functions for preprocessing ranges
impl Range {
    /// Normalize the range to have both bounds with even number of digits.
    /// Takes ownership of self and returns a Result of a new Range.
    /// If both bounds already have even number of digits, returns self.
    /// if the resulting bounds no longer overlap, returns an Err with a message.
    pub fn normalize_uneven(self) -> Result<Self, String> {
        // lower bound
        let number_of_digits = self.lower_bound.checked_ilog10().unwrap_or(0) + 1;
        let new_lower_bound = if number_of_digits.rem_euclid(2) != 0 {
            10u64.pow(number_of_digits - 1)
        } else {
            self.lower_bound
        };

        // upper bound
        let number_of_digits = self.upper_bound.checked_ilog10().unwrap_or(0) + 1;
        let new_upper_bound = if number_of_digits.rem_euclid(2) != 0 {
            10u64.pow(number_of_digits - 1) - 1
        } else {
            self.upper_bound
        };

        let range = Range {
            lower_bound: new_lower_bound,
            upper_bound: new_upper_bound,
        };

        if range.is_valid() {
            Ok(range)
        } else {
            Err("Cannot normalize range: resulting bounds are invalid".to_string())
        }
    }

    /// normalize the range to have both bounds with same number of digits
    /// Takes ownership of self and returns a Vec of Ranges.
    /// If both bounds already have same number of digits, returns a Vec with self.
    /// If not, returns a Vec of Ranges with same number of digits covering the original range.
    pub fn normalize_digit_significance(self) -> Vec<Range> {
        let lower_digits = utils::vector_of_digits(self.lower_bound);
        let upper_digits = utils::vector_of_digits(self.upper_bound);

        if lower_digits.len() == upper_digits.len() {
            return vec![self];
        }

        (lower_digits.len()..=upper_digits.len())
            .enumerate()
            .map(|(i, digit_count)| Range {
                lower_bound: if i == 0 {
                    self.lower_bound
                } else {
                    10u64.pow((digit_count - 1) as u32)
                },
                upper_bound: if digit_count == upper_digits.len() {
                    self.upper_bound
                } else {
                    10u64.pow(digit_count as u32) - 1
                },
            })
            .collect::<Vec<Range>>()
    }

    /// Check if the range is valid (lower_bound <= upper_bound)
    pub fn is_valid(&self) -> bool {
        self.lower_bound <= self.upper_bound
    }
}