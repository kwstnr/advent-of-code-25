use super::*;

#[test]
fn match_vector_lengths_1111_3333() {
    let lower: Vec<u64> = vec![1, 1, 1, 1];
    let upper: Vec<u64> = vec![3, 3, 3, 3];

    let expected: Vec<u64> = vec![1, 1, 1, 1];

    let result = match_vector_lengths(lower, &upper);

    assert_eq!(expected, result);
}

#[test]
fn match_vector_lengths_11_3333() {
    let lower: Vec<u64> = vec![1, 1];
    let upper: Vec<u64> = vec![3, 3, 3, 3];

    let expected: Vec<u64> = vec![0, 0, 1, 1];

    let result = match_vector_lengths(lower, &upper);

    assert_eq!(expected, result);
}

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
