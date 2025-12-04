pub fn count_accessible_rolls(grid: &str) -> usize {
    grid.chars().filter(|&c| c == '@').count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_paper_roll_with_no_neighbors_is_accessible() {
        let grid = "@";
        assert_eq!(count_accessible_rolls(grid), 1);
    }

    #[test]
    fn empty_grid_has_no_accessible_rolls() {
        let grid = ".";
        assert_eq!(count_accessible_rolls(grid), 0);
    }
}
