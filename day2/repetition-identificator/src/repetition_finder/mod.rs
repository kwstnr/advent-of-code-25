#[cfg(test)]
mod tests;

use crate::parser::Range;

enum Restriction {
    LOWER(Vec<u64>),
    UPPER(Vec<u64>)
}

impl Restriction {
    fn len(&self) -> usize {
        match self {
            Restriction::LOWER(v) => v.len(),
            Restriction::UPPER(v) => v.len(),
        }
    }

    fn first_digit_range(&self) -> Vec<u64> {
        match self {
            Restriction::LOWER(v) => (v[0]..=9).collect::<Vec<u64>>(),
            Restriction::UPPER(v) => (0..=v[0]).collect::<Vec<u64>>(),
        }
    }

    fn vector(&self) -> &Vec<u64> {
        match self {
            Restriction::LOWER(v) => v,
            Restriction::UPPER(v) => v,
        }
    }

    fn restricted_index(&self, first_digit_range: &Vec<u64>) -> usize {
        match self {
            Restriction::LOWER(_) => 0,
            Restriction::UPPER(_) => first_digit_range.len() - 1,
        }
    }

    fn popped(&self) -> Self {
        match self {
            Restriction::LOWER(v) => Restriction::LOWER(v[1..].to_owned()),
            Restriction::UPPER(v) => Restriction::UPPER(v[1..].to_owned()),
        }
    }
}

impl Range {
    pub fn find_repetitions(&self) -> Vec<String> {
        let lower_digits = vector_of_digits(self.lower_bound);
        let upper_digits = vector_of_digits(self.upper_bound);
        let (lower_half, _) = lower_digits.split_at(lower_digits.len() / 2);
        let (upper_half, _) = upper_digits.split_at(upper_digits.len() / 2);

        (lower_half[0]..=upper_half[0]).collect::<Vec<u64>>()
            .clone().into_iter().enumerate()
            .flat_map(|(i, possible_digit)| {
                let inner_results = if i == 0 {
                    Range::find_restricted_repetition_rec_string(Restriction::LOWER(lower_half[1..].to_vec()))
                } else if i == (upper_half[0] - lower_half[0]) as usize {
                    Range::find_restricted_repetition_rec_string(Restriction::UPPER(upper_half[1..].to_vec()))
                } else {
                    Range::find_repetition_unrestricted_rec_string((lower_half.len() - 1).try_into().unwrap())
                };
                inner_results
                    .into_iter()
                    .map(|inner_result| format!("{}{}", possible_digit, inner_result))
                    .collect::<Vec<String>>()
            })
            .map(|half_repetition| format!("{}{}", half_repetition, half_repetition))
            .collect()
    }

    fn find_repetition_unrestricted_rec_string(remaining_half_length: u64) -> Vec<String> {
        if remaining_half_length == 0 {
            return vec![];
        }

        if remaining_half_length > 1 {
            return Range::find_repetition_unrestricted_rec_string(remaining_half_length - 1)
                .into_iter()
                .flat_map(|x| {
                    (0..=9)
                        .map(|possible_digit| possible_digit.to_string())
                        .map(|y| format!("{}{}", y, x))
                        .collect::<Vec<String>>()
                })
                .collect();
        }

        (0..=9).map(|x| x.to_string()).collect::<Vec<String>>()
    }

    fn find_restricted_repetition_rec_string(restriction: Restriction) -> Vec<String> {
        if restriction.len() == 0 {
            return vec![];
        }

        let first_digit_range = restriction.first_digit_range();

        if restriction.len() > 1 {
            return first_digit_range
                .clone()
                .into_iter()
                .enumerate()
                .map(|(i, possible_digit)| {
                    (
                        possible_digit,
                        if i == restriction.restricted_index(&first_digit_range) {
                            Range::find_restricted_repetition_rec_string(
                                restriction.popped(),
                            )
                        } else {
                            Range::find_repetition_unrestricted_rec_string(
                                (restriction.len() - 1).try_into().unwrap(),
                            )
                        },
                    )
                })
                .flat_map(|(possible_digit, inner_results)| {
                    inner_results
                        .into_iter()
                        .map(|x| format!("{}{}", possible_digit, x))
                        .collect::<Vec<String>>()
                })
                .collect()
        }

        first_digit_range
            .into_iter()
            .map(|x| x.to_string())
            .collect()
    }
}

fn vector_of_digits(n: u64) -> Vec<u64> {
    // nod = number of digits
    let nod = n.checked_ilog10().unwrap_or(0) + 1;
    (1..=nod)
        .rev()
        .map(|x| n.rem_euclid(10_u64.pow(x)) / 10_u64.pow(x - 1))
        .collect()
}

fn match_vector_lengths(lower: Vec<u64>, upper: Vec<u64>) -> Vec<u64> {
    // TODO: fill up lower vector with leading zeros to match upper vector length
    vec![0]
}
