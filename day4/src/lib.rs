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

pub fn count_total_removable_rolls(grid: &str) -> usize {
    let mut grid: Vec<Vec<char>> = grid.lines().map(|line| line.chars().collect()).collect();
    let rows = grid.len();
    if rows == 0 {
        return 0;
    }
    let cols = grid[0].len();

    let mut total_removed = 0;

    loop {
        let accessible = find_accessible_positions(&grid, rows, cols);
        if accessible.is_empty() {
            break;
        }
        for (row, col) in &accessible {
            grid[*row][*col] = '.';
        }
        total_removed += accessible.len();
    }

    total_removed
}

fn find_accessible_positions(grid: &[Vec<char>], rows: usize, cols: usize) -> Vec<(usize, usize)> {
    let mut accessible = Vec::new();
    for row in 0..rows {
        for col in 0..cols {
            if grid[row][col] == '@' && count_neighbors_grid(grid, row, col, rows, cols) < 4 {
                accessible.push((row, col));
            }
        }
    }
    accessible
}

fn count_neighbors_grid(
    grid: &[Vec<char>],
    row: usize,
    col: usize,
    rows: usize,
    cols: usize,
) -> usize {
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
                && grid[nr as usize][nc as usize] == '@'
            {
                neighbors += 1;
            }
        }
    }
    neighbors
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

    #[test]
    fn puzzle_example() {
        let grid = "\
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
        assert_eq!(count_accessible_rolls(grid), 13);
    }

    #[test]
    fn solve_puzzle() {
        let grid = include_str!("../paper-roll-locations.txt");
        let result = count_accessible_rolls(grid);
        println!("Puzzle answer: {}", result);
        assert!(result > 0); // We just want to see the answer
    }

    #[test]
    fn puzzle_example_part2() {
        let grid = "\
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
        assert_eq!(count_total_removable_rolls(grid), 43);
    }
}
