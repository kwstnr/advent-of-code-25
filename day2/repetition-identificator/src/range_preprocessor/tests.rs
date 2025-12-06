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
