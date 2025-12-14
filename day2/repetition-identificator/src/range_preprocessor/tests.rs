use super::*;

#[test]
fn preprocess_bound_lower_no_change() {
    let input: u64 = 2222;
    let expected: u64 = 2222;

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
fn preprocess_bound_upper_change_2() {
    let input: u64 = 115;
    let expected: u64 = 99;

    let result = Range::preprocess_bound(input, Bound::UPPER);

    assert_eq!(expected, result);
}

#[test]
fn preprocess_bound_lower_change_2() {
    let input: u64 = 115;
    let expected: u64 = 1000;

    let result = Range::preprocess_bound(input, Bound::LOWER);

    assert_eq!(expected, result);
}

#[test]
fn preprocess_part2_11_22() {
    let input = Range {
        lower_bound: 11,
        upper_bound: 22,
    };
    let expected = vec![
        Range {
            lower_bound: 11,
            upper_bound: 22,
        },
    ];

    let result = input.preprocess_part2();

    assert_eq!(expected, result);
}

#[test]
fn preprocess_part2_11_222() {
    let input = Range {
        lower_bound: 11,
        upper_bound: 222,
    };
    let expected = vec![
        Range {
            lower_bound: 11,
            upper_bound: 99,
        },
        Range {
            lower_bound: 100,
            upper_bound: 222,
        },
    ];

    let result = input.preprocess_part2();

    assert_eq!(expected, result);
}

#[test]
fn preprocess_part2_115_45676() {
    let input = Range {
        lower_bound: 115,
        upper_bound: 45676,
    };
    let expected = vec![
        Range {
            lower_bound: 115,
            upper_bound: 999,
        },
        Range {
            lower_bound: 1000,
            upper_bound: 9999,
        },
        Range {
            lower_bound: 10000,
            upper_bound: 45676,
        },
    ];

    let result = input.preprocess_part2();

    assert_eq!(expected, result);
}
