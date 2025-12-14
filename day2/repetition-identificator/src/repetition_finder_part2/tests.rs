use super::*;

#[test]
fn find_split_length_11() {
let range = Range {
    lower_bound: 11,
    upper_bound: 99,
};

let expected: Vec<u8> = vec![1];

let result = range.find_split_lengths();

assert_eq!(expected, result);
}
#[test]
fn find_split_length_111() {
    let range = Range {
        lower_bound: 111,
        upper_bound: 999,
    };

    let expected: Vec<u8> = vec![1];

    let result = range.find_split_lengths();

    assert_eq!(expected, result);
}

#[test]
fn find_split_length_1111() {
    let range = Range {
        lower_bound: 1111,
        upper_bound: 9999,
    };

    let expected: Vec<u8> = vec![1, 2];

    let result = range.find_split_lengths();

    assert_eq!(expected, result);
}

#[test]
fn find_split_length_11111() {
    let range = Range {
        lower_bound: 11111,
        upper_bound: 99999,
    };

    let expected: Vec<u8> = vec![1];

    let result = range.find_split_lengths();

    assert_eq!(expected, result);
}

#[test]
fn find_split_length_111111() {
    let range = Range {
        lower_bound: 111111,
        upper_bound: 999991,
    };

    let expected: Vec<u8> = vec![1, 2, 3];

    let result = range.find_split_lengths();

    assert_eq!(expected, result);
}

#[test]
fn find_split_length_111111111() {
    let range = Range {
        lower_bound: 111111111,
        upper_bound: 999999999,
    };

    let expected: Vec<u8> = vec![1, 3];

    let result = range.find_split_lengths();

    assert_eq!(expected, result);
}

#[test]
fn find_repetitions_part2_11_22() {
    let range = Range {
        lower_bound: 11,
        upper_bound: 22,
    };

    let expected = vec![11, 22];
    let result = range.find_repetitions_part2();

    assert_eq!(expected, result);
}
