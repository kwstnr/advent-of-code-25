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
                //0..1 -> 0
                for z in 0_u64..=9 {
                    let repetition_half = base + (z * 10_u64.pow(pow as u32));
                    let repetition = repetition_half * (10_u64.pow(y + 1) + 1);
                    println!("repetition: {}", repetition);
                    results.push(repetition);
                }
            }
        }
        results
    }

    pub fn find_repetition_lower_restriction(
        lower_first_half: Vec<u64>,
        upper_first_half: Vec<u64>,
        n: u64,
    ) -> Vec<u64> {
        // n might be the digit that was determined as restricted in the previous step (just needed to build repetitions actually -> might be removed)
        // lower_first_half and upper_first_half should have already been shortened by now (first digit removed)
        // find the lower and upper bound for the restricted next digit (first digit in the array) (the lower bound will always be the digit in lower_first_half at position 0, and the upper 9)
        // do the same again for the lower_restriction of the next digit with lower_first_half and upper_first_half with removed first item = recursive
        // do the inner range repetition finder (same as base function above -> should be moved to its own function) -> might have to check with upper function

        println!("n: {}", n);
        println!("lower_first_half: {:#?}", lower_first_half);
        println!("upper_first_half: {:#?}", upper_first_half);

        // recursive base case -> just for safety
        if lower_first_half.len() == 0 {
            return vec![];
        }

        let range = (lower_first_half[0]..=9).collect::<Vec<u64>>();
        println!("range: {:#?}", range);
        if lower_first_half.len() == 1 {
            return range
                .iter()
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
        let inner_results = Range::find_repetition_lower_restriction(
            lower_first_half[1..].to_owned(),
            upper_first_half[1..].to_owned(),
            range[0],
        );
        inner_results
            .iter()
            .map(|x| {
                let repetition_half = x + (n * 10_u64.pow(lower_first_half.len() as u32));
                let repetition = repetition_half * (10_u64.pow((lower_first_half.len() + 1).try_into().unwrap()) + 1); // this upwards multiplication (duplication of half) does not work for higher digit counts
                println!("repetition inner: {}", repetition);
                repetition
            })
            .for_each(|x| results.push(x));

        // rest of elements
        let amount_of_additional_digits = lower_first_half.len() - 1;
        let base_of_repetition = n * 10_u64.pow(amount_of_additional_digits as u32);
        for y in range[1..].iter() {
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn vector_of_digits_0() {
        let input = 0;
        let expected = vec![0];

        let result = vector_of_digits(input);

        assert_eq!(expected, result);
    }

    #[test]
    fn vector_of_digits_5() {
        let input = 5;
        let expected = vec![5];

        let result = vector_of_digits(input);

        assert_eq!(expected, result);
    }

    #[test]
    fn vector_of_digits_10() {
        let input = 10;
        let expected = vec![1, 0];

        let result = vector_of_digits(input);

        assert_eq!(expected, result);
    }

    #[test]
    fn vector_of_digits_12360() {
        let input = 12360;
        let expected = vec![1, 2, 3, 6, 0];

        let result = vector_of_digits(input);

        assert_eq!(expected, result);
    }

    #[test]
    fn find_repetitions_1111_3333() {
        let input = Range {
            lower_bound: 1111,
            upper_bound: 3333,
        };
        let expected = vec![
            1111, 1212, 1313, 1414, 1515, 1616, 1717, 1818, 1919, 2020, 2121, 2222, 2323, 2424,
            2525, 2626, 2727, 2828, 2929, 3030, 3131, 3232, 3333,
        ];

        let result = input.find_repetitions();

        assert_eq!(expected, result);
    }

    #[test]
    fn find_repetition_lower_restriction_1111_3333() {
        let lower_first_half = vec![1];
        let upper_first_half = vec![3];
        let n = 1;

        let expected = vec![1111, 1212, 1313, 1414, 1515, 1616, 1717, 1818, 1919];

        let result =
            Range::find_repetition_lower_restriction(lower_first_half, upper_first_half, n);

        assert_eq!(expected, result);
    }

    #[test]
    fn find_repetition_lower_restriction_111111_333333() {
        let lower_first_half = vec![1, 1];
        let upper_first_half = vec![3, 3];
        let n = 1;

        let expected = vec![
            111111, 112112, 113113, 114114, 115115, 116116, 117117, 118118, 119119,
            121121, 122122, 123123, 124124, 125125, 126126, 127127, 128128, 129129,
            131131, 132132, 133133, 134134, 135135, 136136, 137137, 138138, 139139,
            141141, 142142, 143143, 144144, 145145, 146146, 147147, 148148, 149149,
            151151, 152152, 153153, 154154, 155155, 156156, 157157, 158158, 159159,
            161161, 162162, 163163, 164164, 165165, 166166, 167167, 168168, 169169,
            171171, 172172, 173173, 174174, 175175, 176176, 177177, 178178, 179179,
            181181, 182182, 183183, 184184, 185185, 186186, 187187, 188188, 189189,
            191191, 192192, 193193, 194194, 195195, 196196, 197197, 198198, 199199,
        ];

        let result =
            Range::find_repetition_lower_restriction(lower_first_half, upper_first_half, n);

        assert_eq!(expected, result);
    }

    #[test]
    fn find_repetition_upper_restriction_1111_3333() {
        let lower_first_half = vec![1];
        let upper_first_half = vec![3];
        let n = 3;

        let expected = vec![3030, 3131, 3232, 3333];

        let result =
            Range::find_repetition_lower_restriction(lower_first_half, upper_first_half, n);

        assert_eq!(expected, result);
    }
}
