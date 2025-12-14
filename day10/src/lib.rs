/// Parses indicator diagram like "[.##.]" into target state
/// '.' = false (off), '#' = true (on)
fn parse_indicator_diagram(input: &str) -> Vec<bool> {
    input
        .trim_start_matches('[')
        .trim_end_matches(']')
        .chars()
        .map(|c| c == '#')
        .collect()
}

/// Parses button wiring like "(1,3)" into indices to toggle
fn parse_button(input: &str) -> Vec<usize> {
    input
        .trim_start_matches('(')
        .trim_end_matches(')')
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect()
}

/// Parses a machine line and returns (target_state, buttons)
fn parse_machine(line: &str) -> (Vec<bool>, Vec<Vec<usize>>) {
    let bracket_end = line.find(']').unwrap();
    let indicator = &line[..=bracket_end];
    let target = parse_indicator_diagram(indicator);

    let rest = &line[bracket_end + 1..];
    let buttons: Vec<Vec<usize>> = rest
        .split_whitespace()
        .filter(|s| s.starts_with('('))
        .map(parse_button)
        .collect();

    (target, buttons)
}

/// Parses joltage requirements like "{3,5,4,7}" into target values
fn parse_joltage(input: &str) -> Vec<i64> {
    input
        .trim_start_matches('{')
        .trim_end_matches('}')
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect()
}

/// Parses a machine line for Part 2 and returns (buttons, joltage_targets)
fn parse_machine_part2(line: &str) -> (Vec<Vec<usize>>, Vec<i64>) {
    // Extract buttons (...)
    let buttons: Vec<Vec<usize>> = line
        .split_whitespace()
        .filter(|s| s.starts_with('('))
        .map(parse_button)
        .collect();

    // Extract joltage {...}
    let joltage_start = line.find('{').unwrap();
    let joltage_end = line.find('}').unwrap();
    let joltage = parse_joltage(&line[joltage_start..=joltage_end]);

    (buttons, joltage)
}

/// Builds the augmented matrix [A | b] for Gaussian elimination
/// Each row represents a light, each column a button
fn build_augmented_matrix(target: &[bool], buttons: &[Vec<usize>]) -> Vec<Vec<u8>> {
    let num_lights = target.len();
    let num_buttons = buttons.len();
    let mut matrix: Vec<Vec<u8>> = vec![vec![0; num_buttons + 1]; num_lights];

    // Set button columns: matrix[light][button] = 1 if button toggles that light
    for (button_idx, indices) in buttons.iter().enumerate() {
        for &light_idx in indices {
            if light_idx < num_lights {
                matrix[light_idx][button_idx] = 1;
            }
        }
    }

    // Set target column (last column)
    for (light_idx, &is_on) in target.iter().enumerate() {
        matrix[light_idx][num_buttons] = u8::from(is_on);
    }

    matrix
}

/// Performs Gaussian elimination over GF(2)
/// Returns mapping from row to pivot column, and transforms matrix in-place
fn gaussian_elimination_gf2(matrix: &mut [Vec<u8>], num_buttons: usize) -> Vec<Option<usize>> {
    let num_lights = matrix.len();
    let mut row_pivot: Vec<Option<usize>> = vec![None; num_lights];
    let mut pivot_col = 0;
    let mut current_row = 0;

    while current_row < num_lights && pivot_col < num_buttons {
        let found = (current_row..num_lights).find(|&r| matrix[r][pivot_col] == 1);

        if let Some(pivot_row) = found {
            matrix.swap(current_row, pivot_row);
            row_pivot[current_row] = Some(pivot_col);

            // Eliminate other rows
            let pivot_values: Vec<u8> = matrix[current_row].to_vec();
            for (r, row) in matrix.iter_mut().enumerate() {
                if r != current_row && row[pivot_col] == 1 {
                    for (cell, &pivot_val) in row.iter_mut().zip(pivot_values.iter()) {
                        *cell ^= pivot_val;
                    }
                }
            }
            current_row += 1;
        }
        pivot_col += 1;
    }

    row_pivot
}

/// Builds mapping from column index to its pivot row (if any)
fn build_column_to_pivot_map(
    row_pivot: &[Option<usize>],
    num_buttons: usize,
) -> Vec<Option<usize>> {
    let mut col_to_pivot_row: Vec<Option<usize>> = vec![None; num_buttons];
    for (row, &pivot) in row_pivot.iter().enumerate() {
        if let Some(col) = pivot {
            col_to_pivot_row[col] = Some(row);
        }
    }
    col_to_pivot_row
}

/// Finds the minimum number of button presses by trying all free variable combinations
fn find_minimum_solution(
    matrix: &[Vec<u8>],
    col_to_pivot_row: &[Option<usize>],
    num_buttons: usize,
) -> usize {
    let free_vars: Vec<usize> = (0..num_buttons)
        .filter(|&c| col_to_pivot_row[c].is_none())
        .collect();

    let mut min_presses = usize::MAX;

    for mask in 0..(1u64 << free_vars.len()) {
        let mut solution = vec![0u8; num_buttons];

        // Set free variables based on mask bits
        for (i, &col) in free_vars.iter().enumerate() {
            solution[col] = ((mask >> i) & 1) as u8;
        }

        // Back-substitute for pivot variables
        for col in (0..num_buttons).rev() {
            if let Some(row) = col_to_pivot_row[col] {
                let mut val = matrix[row][num_buttons];
                for c in (col + 1)..num_buttons {
                    val ^= matrix[row][c] * solution[c];
                }
                solution[col] = val;
            }
        }

        let presses: usize = solution.iter().map(|&x| x as usize).sum();
        min_presses = min_presses.min(presses);
    }

    min_presses
}

/// Solves for minimum button presses to achieve target state
/// Uses Gaussian elimination over GF(2) (binary field)
fn solve_machine(line: &str) -> usize {
    let (target, buttons) = parse_machine(line);
    let num_buttons = buttons.len();

    let mut matrix = build_augmented_matrix(&target, &buttons);
    let row_pivot = gaussian_elimination_gf2(&mut matrix, num_buttons);
    let col_to_pivot_row = build_column_to_pivot_map(&row_pivot, num_buttons);

    find_minimum_solution(&matrix, &col_to_pivot_row, num_buttons)
}

/// Solves for the total minimum button presses for all machines in input
pub fn solve(input: &str) -> usize {
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(solve_machine)
        .sum()
}

// ============ Part 2 ============

/// Builds augmented matrix for integer linear programming
/// Each row represents a counter, each column a button
fn build_augmented_matrix_i64(joltage: &[i64], buttons: &[Vec<usize>]) -> Vec<Vec<i64>> {
    let num_counters = joltage.len();
    let num_buttons = buttons.len();
    let mut matrix: Vec<Vec<i64>> = vec![vec![0; num_buttons + 1]; num_counters];

    for (button_idx, indices) in buttons.iter().enumerate() {
        for &counter_idx in indices {
            if counter_idx < num_counters {
                matrix[counter_idx][button_idx] = 1;
            }
        }
    }

    // Set target column
    for (counter_idx, &target) in joltage.iter().enumerate() {
        matrix[counter_idx][num_buttons] = target;
    }

    matrix
}

/// Performs Gaussian elimination over integers (not GF(2))
/// Returns mapping from row to pivot column
fn gaussian_elimination_integers(
    matrix: &mut [Vec<i64>],
    num_buttons: usize,
) -> Vec<Option<usize>> {
    let num_rows = matrix.len();
    let mut row_pivot: Vec<Option<usize>> = vec![None; num_rows];
    let mut pivot_col = 0;
    let mut current_row = 0;

    while current_row < num_rows && pivot_col < num_buttons {
        // Find pivot (non-zero entry)
        let found = (current_row..num_rows).find(|&r| matrix[r][pivot_col] != 0);

        if let Some(pivot_row) = found {
            matrix.swap(current_row, pivot_row);
            row_pivot[current_row] = Some(pivot_col);

            // Get the pivot value
            let pivot_val = matrix[current_row][pivot_col];

            // Eliminate other rows using integer arithmetic
            for r in 0..num_rows {
                if r != current_row && matrix[r][pivot_col] != 0 {
                    let factor = matrix[r][pivot_col];
                    for c in 0..=num_buttons {
                        matrix[r][c] = matrix[r][c] * pivot_val - matrix[current_row][c] * factor;
                    }
                }
            }
            current_row += 1;
        }
        pivot_col += 1;
    }

    row_pivot
}

/// Finds minimum non-negative integer solution with smart pruning
fn find_minimum_solution_integers(
    matrix: &[Vec<i64>],
    row_pivot: &[Option<usize>],
    num_buttons: usize,
) -> i64 {
    // Build column to pivot row mapping
    let mut col_to_pivot_row: Vec<Option<usize>> = vec![None; num_buttons];
    for (row, &pivot) in row_pivot.iter().enumerate() {
        if let Some(col) = pivot {
            col_to_pivot_row[col] = Some(row);
        }
    }

    let free_vars: Vec<usize> = (0..num_buttons)
        .filter(|&c| col_to_pivot_row[c].is_none())
        .collect();

    // If no free variables, compute unique solution
    if free_vars.is_empty() {
        let solution = compute_solution(&[], &free_vars, matrix, &col_to_pivot_row, num_buttons);
        return if let Some(sol) = solution {
            sol.iter().sum()
        } else {
            i64::MAX
        };
    }

    // Search with branch and bound
    let mut min_presses = i64::MAX;
    let mut values = Vec::new();

    fn search(
        free_vars: &[usize],
        idx: usize,
        values: &mut Vec<i64>,
        matrix: &[Vec<i64>],
        col_to_pivot_row: &[Option<usize>],
        num_buttons: usize,
        min_presses: &mut i64,
    ) {
        // Prune: current sum of free vars already >= best
        let current_sum: i64 = values.iter().sum();
        if current_sum >= *min_presses {
            return;
        }

        if idx == free_vars.len() {
            // Compute full solution
            if let Some(solution) =
                compute_solution(values, free_vars, matrix, col_to_pivot_row, num_buttons)
            {
                if solution.iter().all(|&x| x >= 0) {
                    let total: i64 = solution.iter().sum();
                    *min_presses = (*min_presses).min(total);
                }
            }
            return;
        }

        // Compute upper bound for this free variable
        // based on non-negativity constraints of pivot variables
        let max_val = compute_max_free_value(
            idx,
            values,
            free_vars,
            matrix,
            col_to_pivot_row,
            num_buttons,
        );

        for v in 0..=max_val {
            values.push(v);
            search(
                free_vars,
                idx + 1,
                values,
                matrix,
                col_to_pivot_row,
                num_buttons,
                min_presses,
            );
            values.pop();
        }
    }

    search(
        &free_vars,
        0,
        &mut values,
        matrix,
        &col_to_pivot_row,
        num_buttons,
        &mut min_presses,
    );

    min_presses
}

/// Compute upper bound for free variable at index idx
fn compute_max_free_value(
    _idx: usize,
    _values: &[i64],
    _free_vars: &[usize],
    matrix: &[Vec<i64>],
    _col_to_pivot_row: &[Option<usize>],
    num_buttons: usize,
) -> i64 {
    // Simple bound: max of target values
    // A more sophisticated bound would analyze the linear dependencies
    matrix
        .iter()
        .map(|row| row[num_buttons].abs())
        .max()
        .unwrap_or(0)
}

/// Compute solution given free variable values
fn compute_solution(
    free_values: &[i64],
    free_vars: &[usize],
    matrix: &[Vec<i64>],
    col_to_pivot_row: &[Option<usize>],
    num_buttons: usize,
) -> Option<Vec<i64>> {
    let mut solution = vec![0i64; num_buttons];

    // Set free variables
    for (i, &col) in free_vars.iter().enumerate() {
        solution[col] = free_values[i];
    }

    // Back-substitute for pivot variables
    for col in (0..num_buttons).rev() {
        if let Some(row) = col_to_pivot_row[col] {
            let pivot_val = matrix[row][col];
            let mut rhs = matrix[row][num_buttons];
            for c in (col + 1)..num_buttons {
                rhs -= matrix[row][c] * solution[c];
            }
            // Check if divisible
            if rhs % pivot_val != 0 {
                return None;
            }
            solution[col] = rhs / pivot_val;
        }
    }

    Some(solution)
}

/// Solves Part 2: minimum button presses for joltage counters
fn solve_machine_part2(line: &str) -> i64 {
    let (buttons, joltage) = parse_machine_part2(line);
    let num_buttons = buttons.len();

    let mut matrix = build_augmented_matrix_i64(&joltage, &buttons);
    let row_pivot = gaussian_elimination_integers(&mut matrix, num_buttons);

    find_minimum_solution_integers(&matrix, &row_pivot, num_buttons)
}

/// Solves for the total minimum button presses for Part 2
pub fn solve_part2(input: &str) -> i64 {
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(solve_machine_part2)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_indicator_diagram() {
        assert_eq!(
            parse_indicator_diagram("[.##.]"),
            vec![false, true, true, false]
        );
    }

    #[test]
    fn test_parse_button() {
        assert_eq!(parse_button("(1,3)"), vec![1, 3]);
    }

    #[test]
    fn test_parse_button_single() {
        assert_eq!(parse_button("(3)"), vec![3]);
    }

    #[test]
    fn test_parse_machine() {
        let (target, buttons) = parse_machine("[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}");
        assert_eq!(target, vec![false, true, true, false]);
        assert_eq!(
            buttons,
            vec![
                vec![3],
                vec![1, 3],
                vec![2],
                vec![2, 3],
                vec![0, 2],
                vec![0, 1]
            ]
        );
    }

    #[test]
    fn test_solve_machine_first_example() {
        assert_eq!(
            solve_machine("[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}"),
            2
        );
    }

    #[test]
    fn test_solve_machine_second_example() {
        assert_eq!(
            solve_machine("[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}"),
            3
        );
    }

    #[test]
    fn test_solve_machine_third_example() {
        assert_eq!(
            solve_machine("[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}"),
            2
        );
    }

    #[test]
    fn test_solve_all_examples() {
        let input = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";
        assert_eq!(solve(input), 7);
    }

    // Part 2 tests
    #[test]
    fn test_parse_joltage() {
        assert_eq!(parse_joltage("{3,5,4,7}"), vec![3, 5, 4, 7]);
    }

    #[test]
    fn test_parse_machine_part2() {
        let (buttons, joltage) =
            parse_machine_part2("[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}");
        assert_eq!(
            buttons,
            vec![
                vec![3],
                vec![1, 3],
                vec![2],
                vec![2, 3],
                vec![0, 2],
                vec![0, 1]
            ]
        );
        assert_eq!(joltage, vec![3, 5, 4, 7]);
    }

    #[test]
    fn test_solve_machine_part2_first_example() {
        assert_eq!(
            solve_machine_part2("[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}"),
            10
        );
    }

    #[test]
    fn test_solve_machine_part2_second_example() {
        assert_eq!(
            solve_machine_part2("[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}"),
            12
        );
    }

    #[test]
    fn test_solve_machine_part2_third_example() {
        assert_eq!(
            solve_machine_part2("[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}"),
            11
        );
    }

    #[test]
    fn test_solve_part2_all_examples() {
        let input = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";
        assert_eq!(solve_part2(input), 33);
    }
}
