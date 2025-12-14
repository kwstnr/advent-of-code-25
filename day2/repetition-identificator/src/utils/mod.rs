#[cfg(test)]
mod tests;

pub fn vector_of_digits(n: u64) -> Vec<u64> {
    // nod = number of digits
    let nod = n.checked_ilog10().unwrap_or(0) + 1;
    (1..=nod)
        .rev()
        .map(|x| n.rem_euclid(10_u64.pow(x)) / 10_u64.pow(x - 1))
        .collect()
}

pub fn match_vector_lengths(lower: Vec<u64>, upper: &Vec<u64>) -> Vec<u64> {
    if lower.len() < upper.len() {
        let mut new_lower = vec![0; upper.len() - lower.len()];
        new_lower.extend(lower);
        return new_lower;
    }
    lower
}
