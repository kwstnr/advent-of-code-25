use crate::parser::Range;

impl Range {
    // expects a preprocessed range
    pub fn find_repetitions(&self) -> Vec<u64> {
        let lower_digits = vector_of_digits(self.lower_bound);
        let upper_digits = vector_of_digits(self.upper_bound);
        println!("lower: {:#?}", lower_digits);
        println!("upper: {:#?}", upper_digits);
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
            lower_bound: 11,
            upper_bound: 22
        };
        let expected = vec![11, 22];

        let result = input.find_repetitions();

        assert_eq!(expected, result);
    }
}
