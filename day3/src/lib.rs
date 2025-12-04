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

/// Calculates the maximum joltage from a bank by picking exactly n batteries.
/// Uses a greedy approach: at each position, pick the largest digit that
/// leaves enough remaining digits to complete the selection.
pub fn max_joltage_n(bank: &str, n: usize) -> u64 {
    let digits: Vec<u64> = bank
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u64)
        .collect();
    let mut result: u64 = 0;
    let mut start = 0;

    for remaining in (1..=n).rev() {
        // We need to pick `remaining` more digits
        // The latest position we can pick from is len - remaining
        let end = digits.len() - remaining;

        // Find the maximum digit in range [start, end]
        let mut max_idx = start;
        for i in start..=end {
            if digits[i] > digits[max_idx] {
                max_idx = i;
            }
        }

        result = result * 10 + digits[max_idx];
        start = max_idx + 1;
    }

    result
}

/// Solves the puzzle by summing the maximum joltage from each bank.
pub fn solve(input: &str) -> u32 {
    input.lines().map(max_joltage).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn max_joltage_first_two_batteries_are_largest() {
        // In 987654321111111, the largest joltage is 98 (first two batteries)
        assert_eq!(max_joltage("987654321111111"), 98);
    }

    #[test]
    fn max_joltage_largest_digits_at_opposite_ends() {
        // In 811111111111119, the 8 is first and 9 is last, producing 89
        assert_eq!(max_joltage("811111111111119"), 89);
    }

    #[test]
    fn max_joltage_last_two_batteries_are_largest() {
        // In 234234234234278, the last two batteries (7 and 8) produce 78
        assert_eq!(max_joltage("234234234234278"), 78);
    }

    #[test]
    fn max_joltage_largest_digits_in_middle() {
        // In 818181911112111, the 9 and 2 somewhere in the middle produce 92
        assert_eq!(max_joltage("818181911112111"), 92);
    }

    #[test]
    fn solve_example_input() {
        let input = "987654321111111\n811111111111119\n234234234234278\n818181911112111";
        // 98 + 89 + 78 + 92 = 357
        assert_eq!(solve(input), 357);
    }

    // Part 2 tests
    #[test]
    fn max_joltage_n_first_example() {
        // In 987654321111111, pick 12 batteries -> 987654321111
        assert_eq!(max_joltage_n("987654321111111", 12), 987654321111);
    }

    #[test]
    fn max_joltage_n_second_example() {
        // In 811111111111119, pick 12 batteries -> 811111111119
        assert_eq!(max_joltage_n("811111111111119", 12), 811111111119);
    }

    #[test]
    fn max_joltage_n_third_example() {
        // In 234234234234278, pick 12 batteries -> 434234234278
        assert_eq!(max_joltage_n("234234234234278", 12), 434234234278);
    }

    #[test]
    fn max_joltage_n_fourth_example() {
        // In 818181911112111, pick 12 batteries -> 888911112111
        assert_eq!(max_joltage_n("818181911112111", 12), 888911112111);
    }
}
