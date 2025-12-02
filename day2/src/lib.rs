pub fn is_invalid_id(id: u64) -> bool {
    let s = id.to_string();
    let len = s.len();

    if !len.is_multiple_of(2) {
        return false;
    }

    let mid = len / 2;
    let first_half = &s[..mid];
    let second_half = &s[mid..];

    first_half == second_half
}

pub struct Range {
    pub start: u64,
    pub end: u64,
}

pub fn parse_range(input: &str) -> Result<Range, String> {
    let parts: Vec<&str> = input.split('-').collect();
    if parts.len() != 2 {
        return Err("Invalid range format".to_string());
    }

    let start = parts[0]
        .parse::<u64>()
        .map_err(|_| "Invalid start number".to_string())?;
    let end = parts[1]
        .parse::<u64>()
        .map_err(|_| "Invalid end number".to_string())?;

    Ok(Range { start, end })
}

pub fn find_invalid_ids_in_range(range: &Range) -> Vec<u64> {
    let mut invalid_ids = Vec::new();
    for id in range.start..=range.end {
        if is_invalid_id(id) {
            invalid_ids.push(id);
        }
    }
    invalid_ids
}

pub fn solve(input: &str) -> u64 {
    let mut total = 0;

    for range_str in input.split(',') {
        let range_str = range_str.trim();
        if let Ok(range) = parse_range(range_str) {
            let invalid_ids = find_invalid_ids_in_range(&range);
            total += invalid_ids.iter().sum::<u64>();
        }
    }

    total
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn detects_simple_invalid_id() {
        assert!(is_invalid_id(55));
    }

    #[test]
    fn detects_four_digit_invalid_id() {
        assert!(is_invalid_id(6464));
    }

    #[test]
    fn detects_six_digit_invalid_id() {
        assert!(is_invalid_id(123123));
    }

    #[test]
    fn valid_id_is_not_invalid() {
        assert!(!is_invalid_id(101));
    }

    #[test]
    fn parses_simple_range() {
        let range = parse_range("11-22").unwrap();
        assert_eq!(range.start, 11);
        assert_eq!(range.end, 22);
    }

    #[test]
    fn finds_invalid_ids_in_range() {
        let range = Range { start: 11, end: 22 };
        let invalid_ids = find_invalid_ids_in_range(&range);
        assert_eq!(invalid_ids, vec![11, 22]);
    }

    #[test]
    fn finds_99_in_range_95_to_115() {
        let range = Range {
            start: 95,
            end: 115,
        };
        let invalid_ids = find_invalid_ids_in_range(&range);
        assert_eq!(invalid_ids, vec![99]);
    }

    #[test]
    fn finds_1010_in_range_998_to_1012() {
        let range = Range {
            start: 998,
            end: 1012,
        };
        let invalid_ids = find_invalid_ids_in_range(&range);
        assert_eq!(invalid_ids, vec![1010]);
    }

    #[test]
    fn finds_no_invalid_ids_in_range_1698522_to_1698528() {
        let range = Range {
            start: 1698522,
            end: 1698528,
        };
        let invalid_ids = find_invalid_ids_in_range(&range);
        assert_eq!(invalid_ids, Vec::<u64>::new());
    }

    #[test]
    fn solves_example() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        assert_eq!(solve(input), 1227775554);
    }
}
