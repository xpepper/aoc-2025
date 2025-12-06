use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub struct Range {
    start: u64,
    end: u64,
}

impl Range {
    pub fn contains(&self, id: u64) -> bool {
        id >= self.start && id <= self.end
    }
}

fn split_range_parts(s: &str) -> Result<(&str, &str), String> {
    let parts: Vec<&str> = s.split('-').collect();
    if parts.len() != 2 {
        return Err(format!("Invalid range format: {}", s));
    }
    Ok((parts[0], parts[1]))
}

fn parse_number_part(part: &str, part_name: &str) -> Result<u64, String> {
    part.parse()
        .map_err(|_| format!("Invalid {}: {}", part_name, part))
}

impl FromStr for Range {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start_str, end_str) = split_range_parts(s)?;
        let start = parse_number_part(start_str, "start")?;
        let end = parse_number_part(end_str, "end")?;
        Ok(Range { start, end })
    }
}

pub fn is_fresh(ranges: &[Range], id: u64) -> bool {
    ranges.iter().any(|range| range.contains(id))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn range_contains_id_when_id_is_within_range() {
        let range = Range { start: 3, end: 5 };
        assert_eq!(range.contains(3), true);
        assert_eq!(range.contains(4), true);
        assert_eq!(range.contains(5), true);
        assert_eq!(range.contains(2), false);
        assert_eq!(range.contains(6), false);
    }

    #[test]
    fn range_can_be_parsed_from_string() {
        let range: Range = "3-5".parse().unwrap();
        assert_eq!(range, Range { start: 3, end: 5 });
    }

    #[test]
    fn id_is_fresh_when_in_any_range() {
        let ranges = vec![Range { start: 3, end: 5 }, Range { start: 10, end: 14 }];
        assert_eq!(is_fresh(&ranges, 5), true);
        assert_eq!(is_fresh(&ranges, 11), true);
        assert_eq!(is_fresh(&ranges, 1), false);
        assert_eq!(is_fresh(&ranges, 8), false);
    }
}
