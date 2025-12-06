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
