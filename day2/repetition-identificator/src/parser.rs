#[derive(PartialEq, Debug)]
pub struct Range {
    lower_bound: u64,
    upper_bound: u64
}

pub fn parse(text: &str) -> Vec<Range> {
    text
        .lines()
        .collect::<Vec<&str>>()[0]
        .split(',')
        .map(|unparsed_range| unparsed_range.split('-'))
        .map(|bounds| if let [lower, upper] = bounds.collect::<Vec<&str>>()[0..2] {
            Range {
                lower_bound: lower.parse().unwrap(),
                upper_bound: upper.parse().unwrap()
            }
        } else {
            panic!("range could not be parsed!");
        })
        .collect::<Vec<Range>>()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_1() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224";
        let expected_ranges = vec![
            Range {
                lower_bound: 11,
                upper_bound: 22
            },
            Range {
                lower_bound: 95,
                upper_bound: 115
            },
            Range {
                lower_bound: 998,
                upper_bound: 1012
            },
            Range {
                lower_bound: 1188511880,
                upper_bound: 1188511890
            },
            Range {
                lower_bound: 222220,
                upper_bound: 222224
            }
        ];

        let result = parse(input);

        assert_eq!(result, expected_ranges);
    }
}
