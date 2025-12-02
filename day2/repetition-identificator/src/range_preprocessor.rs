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
            Bound::UPPER => n / 10,
            Bound::LOWER => n * 10
        }
    }

    pub fn preprocess(self) -> Self {
        Range {
            lower_bound: Range::preprocess_bound(self.lower_bound, Bound::LOWER),
            upper_bound: Range::preprocess_bound(self.upper_bound, Bound::UPPER),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn preprocess_bound_lower_no_change() {
        let input: u64 = 2222;
        let expected: u64 = 2222;

        let result = Range::preprocess_bound(input, Bound::LOWER);

        assert_eq!(expected, result);
    }

    #[test]
    fn preprocess_bound_lower_change() {
        let input: u64 = 22222;
        let expected: u64 = 222220;

        let result = Range::preprocess_bound(input, Bound::LOWER);

        assert_eq!(expected, result);
    }

    #[test]
    fn preprocess_bound_upper_no_change() {
        let input: u64 = 2222;
        let expected: u64 = 2222;

        let result = Range::preprocess_bound(input, Bound::UPPER);

        assert_eq!(expected, result);
    }

    #[test]
    fn preprocess_bound_upper_change() {
        let input: u64 = 22222;
        let expected: u64 = 2222;

        let result = Range::preprocess_bound(input, Bound::UPPER);

        assert_eq!(expected, result);
    }
}
