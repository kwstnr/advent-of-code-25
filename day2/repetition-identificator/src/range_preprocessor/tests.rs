use super::*;

#[test]
fn normalize_uneven_no_change() {
    let input = Range {
        lower_bound: 2200,
        upper_bound: 3300,
    };
    let expected = Range {
        lower_bound: 2200,
        upper_bound: 3300,
    };

    let result = input.normalize_uneven().unwrap();

    assert_eq!(expected, result);
}

#[test]
fn normalize_uneven_change_lower() {
    let input = Range {
        lower_bound: 222,
        upper_bound: 3333,
    };
    let expected = Range {
        lower_bound: 1000,
        upper_bound: 3333,
    };

    let result = input.normalize_uneven().unwrap();

    assert_eq!(expected, result);
}

#[test]
fn normalize_uneven_change_upper() {
    let input = Range {
        lower_bound: 2222,
        upper_bound: 33333,
    };
    let expected = Range {
        lower_bound: 2222,
        upper_bound: 9999,
    };

    let result = input.normalize_uneven().unwrap();

    assert_eq!(expected, result);
}

#[test]
fn normalize_uneven_change_both() {
    let input = Range {
        lower_bound: 222,
        upper_bound: 33333,
    };

    let result = input.normalize_uneven();

    assert!(result.is_err());
}

#[test]
fn normalize_uneven_invalid_resulting_range() {
    let input = Range {
        lower_bound: 111,
        upper_bound: 999,
    };

    let result = input.normalize_uneven();

    assert!(result.is_err());
}

#[test]
fn normalize_digit_significance_11_22() {
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

    let result = input.normalize_digit_significance();

    assert_eq!(expected, result);
}

#[test]
fn normalize_digit_significance_11_222() {
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

    let result = input.normalize_digit_significance();

    assert_eq!(expected, result);
}

#[test]
fn normalize_digit_significance_115_45676() {
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

    let result = input.normalize_digit_significance();

    assert_eq!(expected, result);
}
