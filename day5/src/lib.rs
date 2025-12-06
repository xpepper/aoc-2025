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

impl FromStr for Range {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('-').collect();
        if parts.len() != 2 {
            return Err(format!("Invalid range format: {}", s));
        }
        let start = parts[0]
            .parse()
            .map_err(|_| format!("Invalid start: {}", parts[0]))?;
        let end = parts[1]
            .parse()
            .map_err(|_| format!("Invalid end: {}", parts[1]))?;
        Ok(Range { start, end })
    }
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
}
