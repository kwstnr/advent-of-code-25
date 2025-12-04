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
        // TODO: add check for not 0 or length of two digit-lengths
        if lower_digits[0] < upper_digits[1] {
            let inner_range = range_first[1..=range_first.len()-2].to_vec();
            println!("inner_n: {:#?}", inner_range);

            for x in inner_range {
                let y: u32 = (lower_first_half.len()-1).try_into().unwrap();
                let base = x*10_u64.pow(y);

                for pow in 0..y {
                    //0..1 -> 0
                    for z in 0_u64..=9 {
                        let repetition_half = base + (z * 10_u64.pow(pow as u32));
                        let repetition = repetition_half * (10_u64.pow(y+1) + 1);
                        println!("repetition: {}", repetition);
                        results.push(repetition);
                    }
                }
            }
            return results;
            //inner_range
            //    .iter()
            //    .flat_map(|x| {
            //        let y = lower_first_half.len()-1;
            //        let base = x*10_u64.pow(y);
            //        (0..y)
            //            .map()
            //    });
        }
        vec![0]
    }
}

fn vector_of_digits(n: u64) -> Vec<u64> {
    // nod = number of digits
    let nod = n.checked_ilog10().unwrap_or(0) + 1;
    (1..=nod)
        .rev()
        .map(|x| n.rem_euclid(10_u64.pow(x)) / 10_u64.pow(x-1))
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
    fn find_repetitions_11_22() {
        let input = Range {
            lower_bound: 1111,
            upper_bound: 3333
        };
        let expected = vec![
            1111,
            1212,
            1313,
            1414,
            1515,
            1616,
            1717,
            1818,
            1919,
            2020,
            2121,
            2222,
            2323,
            2424,
            2525,
            2626,
            2727,
            2828,
            2929,
            3030,
            3131,
            3232,
            3333
        ];

        let result = input.find_repetitions();

        assert_eq!(expected, result);
    }
}
