/// Calculates the maximum joltage from a bank of batteries.
/// Each bank is a string of digits 1-9. We need to pick exactly two batteries
/// (digits) from the bank to form a two-digit number, maximizing the result.
pub fn max_joltage(bank: &str) -> u32 {
    let digits: Vec<u32> = bank.chars().map(|c| c.to_digit(10).unwrap()).collect();
    let mut max = 0;
    for i in 0..digits.len() {
        for j in (i + 1)..digits.len() {
            let joltage = digits[i] * 10 + digits[j];
            if joltage > max {
                max = joltage;
            }
        }
    }
    max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn max_joltage_first_two_batteries_are_largest() {
        // In 987654321111111, the largest joltage is 98 (first two batteries)
        assert_eq!(max_joltage("987654321111111"), 98);
    }
}
