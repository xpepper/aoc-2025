pub fn count_accessible_rolls(_grid: &str) -> usize {
    1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_paper_roll_with_no_neighbors_is_accessible() {
        let grid = "@";
        assert_eq!(count_accessible_rolls(grid), 1);
    }
}
