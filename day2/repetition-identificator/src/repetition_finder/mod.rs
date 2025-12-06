#[cfg(test)]
mod tests;

use crate::parser::Range;

impl Range {
    // expects a preprocessed range
    pub fn find_repetitions(&self) -> Vec<u64> {
        let lower_digits = vector_of_digits(self.lower_bound);
        let upper_digits = vector_of_digits(self.upper_bound);
        let (lower_first_half, _) = lower_digits.split_at(lower_digits.len() / 2);
        let (upper_first_half, _) = upper_digits.split_at(upper_digits.len() / 2);
        println!("lower: {:#?}", lower_first_half);
        println!("upper: {:#?}", upper_first_half);

        let range_first = (lower_first_half[0]..=upper_first_half[0]).collect::<Vec<u64>>();
        println!("range_first {:#?}", range_first);

        // TODO: refactor into functional pipe
        let mut results: Vec<u64> = vec![];

        // TODO: get repetitions for restricted lower bound
        // TODO: get repetitions for restricted upper bound

        // solve for inner range
        // TODO: move into own function (parameters to be determined)
        let inner_range = range_first[1..=range_first.len() - 2].to_vec();
        println!("inner_n: {:#?}", inner_range);

        for x in inner_range {
            let y: u32 = (lower_first_half.len() - 1).try_into().unwrap();
            let base = x * 10_u64.pow(y);

            for pow in 0..y {
                //starting with the least significant digit <- remember!
                //0..1 -> 0
                for z in 0_u64..=9 {
                    let repetition_half = base + (z * 10_u64.pow(pow));
                    let repetition = repetition_half * (10_u64.pow(y + 1) + 1);
                    println!("repetition: {}", repetition);
                    results.push(repetition);
                }
            }
        }
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

    fn find_repetition_lower_restriction_rec_string(lower_first_half: Vec<u64>) -> Vec<String> {
        if lower_first_half.len() == 0 {
            return vec![];
        }

        let first_digit_range = (lower_first_half[0]..=9).collect::<Vec<u64>>();
        if lower_first_half.len() > 1 {
            return first_digit_range
                .into_iter()
                .enumerate()
                .map(|(i, possible_digit)| {
                    (
                        possible_digit,
                        if i == 0 {
                            Range::find_repetition_lower_restriction_rec_string(
                                lower_first_half[1..].to_owned(),
                            )
                        } else {
                            Range::find_repetition_unrestricted_rec_string(
                                (lower_first_half.len() - 1).try_into().unwrap(),
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
                .collect::<Vec<String>>();
        }

        first_digit_range
            .into_iter()
            .map(|x| x.to_string())
            .collect()
    }

    fn find_repetition_upper_restriction_rec_string(upper_first_half: Vec<u64>) -> Vec<String> {
        if upper_first_half.len() == 0 {
            return vec![];
        }

        let first_digit_range = (0..=upper_first_half[0]).collect::<Vec<u64>>();

        if upper_first_half.len() > 1 {
            return first_digit_range
                .clone()
                .into_iter()
                .enumerate()
                .map(|(i, possible_digit)| {
                    (
                        possible_digit,
                        if i == first_digit_range.len() - 1 {
                            Range::find_repetition_upper_restriction_rec_string(
                                upper_first_half[1..].to_owned(),
                            )
                        } else {
                            Range::find_repetition_unrestricted_rec_string(
                                (upper_first_half.len() - 1).try_into().unwrap(),
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
                .collect::<Vec<String>>();
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
    // fill up lower vector with leading zeros to match upper vector length
    vec![0]
}
