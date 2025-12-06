#[cfg(test)]
mod tests;

#[derive(PartialEq, Debug)]
pub struct Range {
    pub lower_bound: u64,
    pub upper_bound: u64
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
