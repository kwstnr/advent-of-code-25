#[cfg(test)]
mod tests;

/// Converts a number into a vector of its digits.
pub fn vector_of_digits(n: u64) -> Vec<u8> {
    // nod = number of digits
    let nod = n.checked_ilog10().unwrap_or(0) + 1;
    (1..=nod)
        .rev()
        .map(|x| n.rem_euclid(10_u64.pow(x)) / 10_u64.pow(x - 1))
        .map(|x| x as u8)
        .collect::<Vec<u8>>()
}
