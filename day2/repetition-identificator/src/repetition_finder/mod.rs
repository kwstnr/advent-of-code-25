#[cfg(test)]
mod tests;

use crate::parser::Range;

enum Restriction {
    LOWER(Vec<u64>),
    UPPER(Vec<u64>),
    BOTH(Vec<u64>, Vec<u64>),
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

    fn first_digit_range(&self) -> Vec<u64> {
        match self {
            Restriction::LOWER(v) => (v[0]..=9).collect::<Vec<u64>>(),
            Restriction::UPPER(v) => (0..=v[0]).collect::<Vec<u64>>(),
            Restriction::BOTH(v1, v2) => (v1[0]..=v2[0]).collect::<Vec<u64>>(),
        }
    }

    fn restricted_index(&self, first_digit_range: &Vec<u64>) -> usize {
        match self {
            Restriction::LOWER(_) => 0,
            Restriction::UPPER(_) => first_digit_range.len() - 1,
            Restriction::BOTH(v1, _) => panic!("ruh roh"),
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
    pub fn count_repetitions(&self) -> u64 {
        self.find_repetitions().len() as u64
    }

    pub fn find_repetitions(&self) -> Vec<u64> {
        let lower_digits = vector_of_digits(self.lower_bound);
        let upper_digits = vector_of_digits(self.upper_bound);

        let mut additional_results = (lower_digits.len()..upper_digits.len())
            .enumerate()
            .map(|(i, _)| Range {
                lower_bound: if i == 0 {
                    self.lower_bound
                } else {
                    10u64.pow((lower_digits.len() + i - 1) as u32)
                },
                upper_bound: 10u64.pow((lower_digits.len() + i) as u32) - 1,
            })
            .filter(|r| vector_of_digits(r.lower_bound).len().rem_euclid(2) == 0)
            .flat_map(|r| r.find_repetitions())
            .collect::<Vec<u64>>();

        let (lower_half, _) = lower_digits.split_at(lower_digits.len() / 2);
        let (upper_half, _) = upper_digits.split_at(upper_digits.len() / 2);

        let lower_half = match lower_half.len() {
            l if l < upper_half.len() => (0..upper_half.len())
                .into_iter()
                .enumerate()
                .map(|(i, _)| if i == 0 { 1 } else { 0 })
                .collect::<Vec<u64>>(),
            _ => lower_half.to_vec(),
        };

        let mut results: Vec<u64> = if lower_half.len() == 1 {
            (lower_half[0]..=upper_half[0])
                .map(|digit| format!("{}", digit))
                .collect::<Vec<String>>()
        } else {
            (lower_half[0]..=upper_half[0])
                .collect::<Vec<u64>>()
                .clone()
                .into_iter()
                .enumerate()
                .flat_map(|(i, possible_digit)| {
                    let inner_results = if i == 0 && possible_digit != upper_half[0] {
                        Range::find_restricted_repetition_rec_string(Restriction::LOWER(
                            lower_half[1..].to_vec(),
                        ))
                    } else if i == 0 && possible_digit == upper_half[0] {
                        Range::find_restricted_repetition_rec_string(Restriction::BOTH(
                            lower_half[1..].to_vec(),
                            upper_half[1..].to_vec(),
                        ))
                    } else if i == (upper_half[0] - lower_half[0]) as usize {
                        Range::find_restricted_repetition_rec_string(Restriction::UPPER(
                            upper_half[1..].to_vec(),
                        ))
                    } else {
                        Range::find_repetition_unrestricted_rec_string(
                            (lower_half.len() - 1).try_into().unwrap(),
                        )
                    };
                    inner_results
                        .into_iter()
                        .map(|inner_result| format!("{}{}", possible_digit, inner_result))
                        .collect::<Vec<String>>()
                })
                .collect::<Vec<String>>()
        }
        .into_iter()
        .map(|half_repetition| format!("{}{}", half_repetition, half_repetition))
        .map(|repetition| repetition.parse::<u64>().unwrap())
        .filter(|repetition| *repetition >= self.lower_bound && *repetition <= self.upper_bound)
        .collect::<Vec<u64>>();

        results.append(&mut additional_results);

        results
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
                        match &restriction {
                            Restriction::BOTH(lower, upper) => {
                                // TODO: lots of duplicate code between here and below
                                if lower[0] == upper[0] {
                                    Range::find_restricted_repetition_rec_string(Restriction::BOTH(
                                        lower[1..].to_owned(),
                                        upper[1..].to_owned(),
                                    ))
                                } else if i == 0 {
                                    Range::find_restricted_repetition_rec_string(Restriction::LOWER(
                                        lower[1..].to_owned(),
                                    ))
                                } else if i == (first_digit_range.len() - 1) {
                                    Range::find_restricted_repetition_rec_string(Restriction::UPPER(
                                        upper[1..].to_owned(),
                                    ))
                                } else {
                                    Range::find_repetition_unrestricted_rec_string(
                                        (restriction.len() - 1).try_into().unwrap(),
                                    )
                                }
                            }
                            _ => {
                                if i == restriction.restricted_index(&first_digit_range) {
                                    Range::find_restricted_repetition_rec_string(
                                        restriction.popped(),
                                    )
                                } else {
                                    Range::find_repetition_unrestricted_rec_string(
                                        (restriction.len() - 1).try_into().unwrap(),
                                    )
                                }
                            }
                        },
                    )
                })
                .flat_map(|(possible_digit, inner_results)| {
                    inner_results
                        .into_iter()
                        .map(|x| format!("{}{}", possible_digit, x))
                        .collect::<Vec<String>>()
                })
                .collect();
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

fn match_vector_lengths(lower: Vec<u64>, upper: &Vec<u64>) -> Vec<u64> {
    if lower.len() < upper.len() {
        let mut new_lower = vec![0; upper.len() - lower.len()];
        new_lower.extend(lower);
        return new_lower;
    }
    lower
}
