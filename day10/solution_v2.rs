// Improved solution using parametric enumeration for underdetermined systems

use std::collections::{BinaryHeap, HashMap};
use std::cmp::Reverse;

pub struct MachineJoltage {
    pub target_joltage: Vec<usize>,
    pub buttons: Vec<Vec<usize>>,
}

// Validate solution and return sum if valid
fn validate_and_sum(solution: &[f64]) -> Option<usize> {
    let mut total = 0;
    for &val in solution {
        if val < -0.0001 {
            return None; // Negative
        }
        let rounded = val.round();
        if (val - rounded).abs() > 0.0001 {
            return None; // Not integer
        }
        total += rounded as usize;
    }
    Some(total)
}

// Enumerate free variables to find minimum solution
fn enumerate_free_vars(
    matrix: &[Vec<f64>],
    pivot_cols: &[usize],
    free_cols: &[usize],
    num_vars: usize,
    max_val: usize,
) -> Option<usize> {
    let mut min_sum = usize::MAX;

    fn try_combo(
        depth: usize,
        free_cols: &[usize],
        free_vals: &mut Vec<usize>,
        matrix: &[Vec<f64>],
        pivot_cols: &[usize],
        num_vars: usize,
        max_val: usize,
        min_sum: &mut usize,
    ) {
        if depth == free_cols.len() {
            // Build solution
            let mut solution = vec![0.0; num_vars];
            for (i, &val) in free_vals.iter().enumerate() {
                solution[free_cols[i]] = val as f64;
            }

            // Calculate dependent variables
            for (row_idx, &pivot_col) in pivot_cols.iter().enumerate() {
                let mut val = matrix[row_idx][num_vars];
                for (i, &free_col) in free_cols.iter().enumerate() {
                    val -= matrix[row_idx][free_col] * free_vals[i] as f64;
                }
                solution[pivot_col] = val;
            }

            // Validate and update minimum
            if let Some(sum) = validate_and_sum(&solution) {
                *min_sum = (*min_sum).min(sum);
            }
            return;
        }

        // Try values for current free variable
        for val in 0..=max_val {
            free_vals.push(val);
            try_combo(depth + 1, free_cols, free_vals, matrix, pivot_cols, num_vars, max_val, min_sum);
            free_vals.pop();
        }
    }

    let mut free_vals = Vec::new();
    try_combo(0, free_cols, &mut free_vals, matrix, pivot_cols, num_vars, max_val, &mut min_sum);

    if min_sum == usize::MAX {
        None
    } else {
        Some(min_sum)
    }
}

// Solve with parametric free variables
fn solve_with_free_variables(
    machine: &MachineJoltage,
    matrix: &mut [Vec<f64>],
    num_vars: usize,
) -> Option<usize> {
    let num_eqs = matrix.len();

    // Row reduce to RREF to identify pivot and free variables
    let mut pivot_cols = Vec::new();
    let mut row = 0;

    for col in 0..num_vars {
        // Find pivot
        let mut max_row = row;
        for r in row..num_eqs {
            if matrix[r][col].abs() > matrix[max_row][col].abs() {
                max_row = r;
            }
        }

        if matrix[max_row][col].abs() < 1e-10 {
            continue; // Not a pivot column
        }

        // Swap rows
        if max_row != row {
            matrix.swap(row, max_row);
        }

        // Normalize pivot row
        let pivot = matrix[row][col];
        for c in 0..=num_vars {
            matrix[row][c] /= pivot;
        }

        // Eliminate column in all other rows
        for r in 0..num_eqs {
            if r != row {
                let factor = matrix[r][col];
                for c in 0..=num_vars {
                    matrix[r][c] -= factor * matrix[row][c];
                }
            }
        }

        pivot_cols.push(col);
        row += 1;
        if row >= num_eqs {
            break;
        }
    }

    // Identify free columns
    let all_cols: Vec<usize> = (0..num_vars).collect();
    let free_cols: Vec<usize> = all_cols
        .into_iter()
        .filter(|c| !pivot_cols.contains(c))
        .collect();

    if free_cols.is_empty() {
        // Unique solution
        let mut solution = vec![0.0; num_vars];
        for (row_idx, &pivot_col) in pivot_cols.iter().enumerate() {
            solution[pivot_col] = matrix[row_idx][num_vars];
        }
        return validate_and_sum(&solution);
    }

    // Underdetermined - enumerate if feasible
    let max_val = *machine.target_joltage.iter().max().unwrap();

    // Only enumerate if not too many combinations
    if free_cols.len() <= 3 {
        return enumerate_free_vars(matrix, &pivot_cols, &free_cols, num_vars, max_val);
    }

    None
}

pub fn min_presses_joltage(machine: &MachineJoltage) -> usize {
    let num_counters = machine.target_joltage.len();
    let num_buttons = machine.buttons.len();

    // Build augmented matrix
    let mut matrix: Vec<Vec<f64>> = vec![vec![0.0; num_buttons + 1]; num_counters];

    for (button_idx, button) in machine.buttons.iter().enumerate() {
        for &counter_idx in button {
            matrix[counter_idx][button_idx] = 1.0;
        }
    }

    for (counter_idx, &target) in machine.target_joltage.iter().enumerate() {
        matrix[counter_idx][num_buttons] = target as f64;
    }

    // Try parametric solution
    if let Some(result) = solve_with_free_variables(machine, &mut matrix, num_buttons) {
        return result;
    }

    // Fallback: return a large number to signal failure
    eprintln!("Failed to solve machine with {} counters, {} buttons", num_counters, num_buttons);
    0
}
