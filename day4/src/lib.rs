pub fn count_accessible_rolls(grid: &str) -> usize {
    let lines: Vec<&str> = grid.lines().collect();
    let rows = lines.len();
    if rows == 0 {
        return 0;
    }
    let cols = lines[0].len();

    let mut count = 0;
    for (row, line) in lines.iter().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            if ch == '@' && count_neighbors(&lines, row, col, rows, cols) < 4 {
                count += 1;
            }
        }
    }
    count
}

fn count_neighbors(lines: &[&str], row: usize, col: usize, rows: usize, cols: usize) -> usize {
    let mut neighbors = 0;
    for dr in -1i32..=1 {
        for dc in -1i32..=1 {
            if dr == 0 && dc == 0 {
                continue;
            }
            let nr = row as i32 + dr;
            let nc = col as i32 + dc;
            if nr >= 0
                && nr < rows as i32
                && nc >= 0
                && nc < cols as i32
                && lines[nr as usize].chars().nth(nc as usize) == Some('@')
            {
                neighbors += 1;
            }
        }
    }
    neighbors
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

    #[test]
    fn roll_with_four_neighbors_is_not_accessible() {
        // Center roll has 4 neighbors (up, down, left, right)
        let grid = ".@.\n@@@\n.@.";
        assert_eq!(count_accessible_rolls(grid), 4); // only the 4 outer rolls are accessible
    }
}
