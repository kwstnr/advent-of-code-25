#[cfg(test)]
mod tests;

use crate::parser::Range;

enum Bound {
    UPPER,
    LOWER
}

impl Range {
    fn preprocess_bound(n: u64, bound: Bound) -> u64 {
        let number_of_digits = n.checked_ilog10().unwrap_or(0) + 1;
        if number_of_digits.rem_euclid(2) == 0 {
            return n
        }
        match bound {
            Bound::UPPER => 10u64.pow(number_of_digits - 1) - 1,
            Bound::LOWER => 10u64.pow(number_of_digits),
        }
    }

    pub fn preprocess(self) -> Self {
        Range {
            lower_bound: Range::preprocess_bound(self.lower_bound, Bound::LOWER),
            upper_bound: Range::preprocess_bound(self.upper_bound, Bound::UPPER),
        }
    }

    pub fn preprocess_part2(self) -> Vec<Self> {
        let lower_digits = num_to_vec_of_digits(self.lower_bound);
        let upper_digits = num_to_vec_of_digits(self.upper_bound);

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

    pub fn is_valid(&self) -> bool {
        self.lower_bound <= self.upper_bound
    }
}

fn num_to_vec_of_digits(n: u64) -> Vec<u8> {
    n.to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect()
}