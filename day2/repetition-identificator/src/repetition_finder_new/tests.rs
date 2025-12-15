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
fn half_split_length_11() {
    let range = Range {
        lower_bound: 11,
        upper_bound: 99,
    };

    let expected: u8 = 1;
    let result = range.half_split_lengths().unwrap();

    assert_eq!(expected, result);
}

#[test]
fn half_split_length_111_err() {
    let range = Range {
        lower_bound: 111,
        upper_bound: 999,
    };

    let result = range.half_split_lengths();

    assert!(result.is_err());
}

#[test]
fn half_split_length_12345678() {
    let range = Range {
        lower_bound: 12345678,
        upper_bound: 87654321,
    };

    let expected: u8 = 4;
    let result = range.half_split_lengths().unwrap();

    assert_eq!(expected, result);
}
