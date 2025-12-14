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
