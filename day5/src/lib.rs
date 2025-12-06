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
}
