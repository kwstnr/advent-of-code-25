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
            return vec![]
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

        (0..=9)
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
    }

    fn find_repetition_lower_restriction_rec_string(lower_first_half: Vec<u64>) -> Vec<String> {
        vec![]
    }

    pub fn find_repetition_lower_restriction(lower_first_half: Vec<u64>, n: u64) -> Vec<u64> {
        // n might be the digit that was determined as restricted in the previous step (just needed to build repetitions actually -> might be removed)
        // lower_first_half and upper_first_half should have already been shortened by now (first digit removed)
        // find the lower and upper bound for the restricted next digit (first digit in the array) (the lower bound will always be the digit in lower_first_half at position 0, and the upper 9)
        // do the same again for the lower_restriction of the next digit with lower_first_half and upper_first_half with removed first item = recursive
        // do the inner range repetition finder (same as base function above -> should be moved to its own function) -> might have to check with upper function

        println!("n: {}", n);
        println!("lower_first_half: {:#?}", lower_first_half);

        // recursive base case -> just for safety
        if lower_first_half.len() == 0 {
            return vec![];
        }

        let range = (lower_first_half[0]..=9).collect::<Vec<u64>>();
        println!("range: {:#?}", range);
        if lower_first_half.len() == 1 {
            return range
                .into_iter()
                .map(|x| {
                    let repetition_half = n * 10;
                    let repetition = (repetition_half + x) * 101; // this upwards multiplication (duplication of half) does not work for higher digit counts
                    println!("repetition base: {}", repetition);
                    repetition
                })
                .collect();
        }

        let mut results: Vec<u64> = vec![];

        // first element
        let inner_results =
            Range::find_repetition_lower_restriction(lower_first_half[1..].to_owned(), range[0]);
        inner_results
            .into_iter()
            .map(|x| {
                let repetition_half = x + (n * 10_u64.pow(lower_first_half.len() as u32));
                let repetition = repetition_half
                    * (10_u64.pow((lower_first_half.len() + 1).try_into().unwrap()) + 1); // this upwards multiplication (duplication of half) does not work for higher digit counts
                println!("repetition inner: {}", repetition);
                repetition
            })
            .for_each(|x| results.push(x));

        // rest of elements
        let amount_of_additional_digits = lower_first_half.len() - 1;
        let base_of_repetition = n * 10_u64.pow(amount_of_additional_digits as u32);
        for y in range[1..].into_iter() {
            for x in 0..amount_of_additional_digits {
                for z in 0_u64..=9 {
                    let repetition_half = base_of_repetition + (y * 10_u64.pow(x as u32));
                    let repetition_half_with_n_added_at_the_beginning =
                        repetition_half + (n * 10_u64.pow(lower_first_half.len() as u32));
                    let repetition = repetition_half_with_n_added_at_the_beginning
                        * (10_u64.pow((lower_first_half.len() + 1).try_into().unwrap()) + 1);
                    println!("repetition: {}", repetition);
                    results.push(repetition);
                }
            }
        }

        results
    }

    pub fn find_repetition_upper_restriction(
        lower_first_half: Vec<u64>,
        upper_first_half: Vec<u64>,
        n: u64,
    ) -> Vec<u64> {
        // this is the same as lower restriction but kind of mirrored
        // n is the digit that was determined as restricted in the previous step (just needed to build repetitions actually -> might be removed)
        // lower_first_half and upper_first_half should have already been shortened by now (first digit removed)
        // find the lower and upper bound for the restricted next digit (first digit in the array) (the lower will always be 0, and the upper the digit in upper_first_half at position 0)
        // do the same again for the upper_restriction of the next digit with lower_first_half and upper_first_half with removed first item = recursive
        // do the inner range repetition finder (same as base function above -> should be moved to its own function) -> might have to check with upper function
        vec![]
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
