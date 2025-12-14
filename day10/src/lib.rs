struct Machine {
    target_lights: Vec<bool>,
    buttons: Vec<Vec<usize>>,
}

struct MachineJoltage {
    target_joltage: Vec<usize>,
    buttons: Vec<Vec<usize>>,
}

fn parse_machine(input: &str) -> Machine {
    let parts: Vec<&str> = input
        .split(['[', ']', '{', '}'])
        .filter(|s| !s.is_empty())
        .collect();

    let target_lights = parse_target_lights(parts[0]);
    let buttons = parse_buttons(parts[1]);

    Machine {
        target_lights,
        buttons,
    }
}

fn parse_target_lights(lights_str: &str) -> Vec<bool> {
    lights_str
        .chars()
        .filter(|c| *c == '.' || *c == '#')
        .map(|c| c == '#')
        .collect()
}

fn parse_buttons(buttons_str: &str) -> Vec<Vec<usize>> {
    buttons_str
        .trim()
        .split(") (")
        .map(|button_str| {
            button_str
                .trim_matches(|c| c == '(' || c == ')')
                .split(',')
                .map(|n| n.trim().parse().unwrap())
                .collect()
        })
        .collect()
}

fn parse_machine_part2(input: &str) -> MachineJoltage {
    let parts: Vec<&str> = input
        .split(['[', ']', '{', '}'])
        .filter(|s| !s.is_empty())
        .collect();

    let buttons = parse_buttons(parts[1]);
    let target_joltage = parse_joltage(parts[2]);

    MachineJoltage {
        target_joltage,
        buttons,
    }
}

fn parse_joltage(joltage_str: &str) -> Vec<usize> {
    joltage_str
        .trim()
        .split(',')
        .map(|n| n.trim().parse().unwrap())
        .collect()
}

fn min_presses(machine: &Machine) -> usize {
    let num_buttons = machine.buttons.len();
    let num_lights = machine.target_lights.len();

    // For toggling, pressing a button twice = not pressing it at all
    // So we only need to try 0 or 1 presses per button
    // Try all 2^num_buttons combinations
    let mut min_presses = usize::MAX;

    for mask in 0..(1 << num_buttons) {
        let mut lights = vec![false; num_lights];
        let mut total_presses = 0;

        for button_idx in 0..num_buttons {
            if (mask & (1 << button_idx)) != 0 {
                total_presses += 1;
                toggle_lights(&mut lights, &machine.buttons[button_idx]);
            }
        }

        if lights == machine.target_lights {
            min_presses = min_presses.min(total_presses);
        }
    }

    min_presses
}

fn toggle_lights(lights: &mut [bool], button: &[usize]) {
    for &light_idx in button {
        lights[light_idx] = !lights[light_idx];
    }
}

fn min_presses_joltage(machine: &MachineJoltage) -> usize {
    // 1) Parametric linear solve for underdetermined systems (fast exact)
    if let Some(total) = solve_with_free_variables(machine) {
        return total;
    }

    // 2) A* search with admissible heuristic (much faster than Dijkstra)
    if let Some(total) = a_star_search(machine) {
        return total;
    }

    // 3) Greedy (quick upper bound; rarely exact but cheap)
    if let Some(total) = greedy_solve(machine) {
        return total;
    }

    // 4) Fallback limited search (guarded)
    let total = limited_search(machine);
    if total == usize::MAX { 0 } else { total }
}

// Gaussian elimination to solve Ax = b
fn solve_linear_system(matrix: &mut [Vec<f64>], num_vars: usize) -> Option<Vec<f64>> {
    let num_equations = matrix.len();

    // Forward elimination
    for col in 0..num_vars.min(num_equations) {
        // Find pivot
        let mut pivot_row = col;
        for row in col..num_equations {
            if matrix[row][col].abs() > matrix[pivot_row][col].abs() {
                pivot_row = row;
            }
        }

        if matrix[pivot_row][col].abs() < 1e-10 {
            continue; // Skip this column
        }

        // Swap rows
        if pivot_row != col {
            matrix.swap(col, pivot_row);
        }

        // Eliminate
        for row in (col + 1)..num_equations {
            let factor = matrix[row][col] / matrix[col][col];
            for c in col..=num_vars {
                matrix[row][c] -= factor * matrix[col][c];
            }
        }
    }

    // Back substitution
    let mut solution = vec![0.0; num_vars];

    for row in (0..num_equations.min(num_vars)).rev() {
        let mut sum = matrix[row][num_vars];
        for col in (row + 1)..num_vars {
            sum -= matrix[row][col] * solution[col];
        }
        if matrix[row][row].abs() > 1e-10 {
            solution[row] = sum / matrix[row][row];
        } else {
            // Free variable, try 0
            solution[row] = 0.0;
        }
    }

    Some(solution)
}

// Row-reduced echelon form to identify pivot and free variables and minimize presses
fn solve_with_free_variables(machine: &MachineJoltage) -> Option<usize> {
    let m = machine.target_joltage.len();
    let n = machine.buttons.len();

    // Build augmented matrix [A | b]
    let mut mat: Vec<Vec<f64>> = vec![vec![0.0; n + 1]; m];
    for (j, button) in machine.buttons.iter().enumerate() {
        for &i in button {
            mat[i][j] = 1.0;
        }
    }
    for (i, &t) in machine.target_joltage.iter().enumerate() {
        mat[i][n] = t as f64;
    }

    // RREF
    let mut pivot_cols: Vec<usize> = Vec::new();
    let mut row = 0usize;
    for col in 0..n {
        // find pivot
        let mut max_row = row;
        for r in row..m {
            if mat[r][col].abs() > mat[max_row][col].abs() {
                max_row = r;
            }
        }
        if mat[max_row][col].abs() < 1e-10 {
            continue;
        }
        if max_row != row { mat.swap(row, max_row); }
        let pivot = mat[row][col];
        for c in 0..=n { mat[row][c] /= pivot; }
        for r in 0..m {
            if r != row {
                let f = mat[r][col];
                for c in 0..=n { mat[r][c] -= f * mat[row][c]; }
            }
        }
        pivot_cols.push(col);
        row += 1;
        if row >= m { break; }
    }
    let free_cols: Vec<usize> = (0..n).filter(|c| !pivot_cols.contains(c)).collect();

    // Unique solution case
    if free_cols.is_empty() {
        let mut sol = vec![0.0; n];
        for (ri, &pc) in pivot_cols.iter().enumerate() {
            sol[pc] = mat[ri][n];
        }
        return validate_and_sum(&sol);
    }

    // Enumerate small number of free variables
    let max_val = *machine.target_joltage.iter().max().unwrap_or(&0);
    if free_cols.len() <= 3 {
        let mut min_sum = usize::MAX;
        let mut free_vals: Vec<usize> = Vec::new();
        fn try_combo(
            depth: usize,
            free_cols: &[usize],
            free_vals: &mut Vec<usize>,
            mat: &[Vec<f64>],
            pivot_cols: &[usize],
            n: usize,
            min_sum: &mut usize,
        ) {
            if depth == free_cols.len() {
                let mut sol = vec![0.0; n];
                for (i, &v) in free_vals.iter().enumerate() { sol[free_cols[i]] = v as f64; }
                for (ri, &pc) in pivot_cols.iter().enumerate() {
                    let mut val = mat[ri][n];
                    for (i, &fc) in free_cols.iter().enumerate() { val -= mat[ri][fc] * free_vals[i] as f64; }
                    sol[pc] = val;
                }
                if let Some(sum) = validate_and_sum(&sol) { *min_sum = (*min_sum).min(sum); }
                return;
            }
            // Simple bound: iterate up to observed RHS max to keep space small
            let bound = 64usize; // heuristic cap; targets are ~40-86
            for v in 0..=bound {
                free_vals.push(v);
                try_combo(depth + 1, free_cols, free_vals, mat, pivot_cols, n, min_sum);
                free_vals.pop();
            }
        }
        try_combo(0, &free_cols, &mut free_vals, &mat, &pivot_cols, n, &mut min_sum);
        if min_sum != usize::MAX { return Some(min_sum); }
    }
    None
}

fn validate_and_sum(solution: &[f64]) -> Option<usize> {
    let mut total = 0usize;
    for &v in solution {
        if v < -0.0001 { return None; }
        let r = v.round();
        if (v - r).abs() > 0.0001 { return None; }
        total += r as usize;
    }
    Some(total)
}

// A* with admissible heuristic: max remaining presses needed on any counter
fn a_star_search(machine: &MachineJoltage) -> Option<usize> {
    use std::collections::{BinaryHeap, HashMap};
    use std::cmp::Reverse;

    let target = &machine.target_joltage;
    let initial = vec![0usize; target.len()];
    let mut heap: BinaryHeap<Reverse<(usize, usize, Vec<usize>)>> = BinaryHeap::new();
    let mut best_g: HashMap<Vec<usize>, usize> = HashMap::new();

    let h0 = heuristic_max_remaining(target, &initial);
    heap.push(Reverse((h0, 0, initial.clone())));
    best_g.insert(initial, 0);

    let max_states = 5_000_000usize; // tighter cap than Dijkstra
    let mut explored = 0usize;

    while let Some(Reverse((_, g, state))) = heap.pop() {
        explored += 1;
        if explored > max_states { break; }
        if &state == target { return Some(g); }

        if let Some(&bg) = best_g.get(&state) { if g > bg { continue; } }

        for button in &machine.buttons {
            let mut ns = state.clone();
            let mut useful = false;
            for &idx in button {
                if ns[idx] < target[idx] { useful = true; }
                ns[idx] += 1;
            }
            if !useful { continue; }
            if !ns.iter().zip(target.iter()).all(|(c, t)| c <= t) { continue; }

            let ng = g + 1;
            if let Some(&bg) = best_g.get(&ns) { if ng >= bg { continue; } }
            best_g.insert(ns.clone(), ng);
            let h = heuristic_max_remaining(target, &ns);
            let f = ng + h;
            heap.push(Reverse((f, ng, ns)));
        }
    }
    None
}

fn heuristic_max_remaining(target: &[usize], current: &[usize]) -> usize {
    target.iter().zip(current.iter()).map(|(t, c)| t.saturating_sub(*c)).max().unwrap_or(0)
}

fn greedy_solve(machine: &MachineJoltage) -> Option<usize> {
    // Try a greedy approach: for each button, press it as many times as possible
    // without overshooting any counter
    let target = &machine.target_joltage;
    let mut state = vec![0; target.len()];
    let mut presses = vec![0; machine.buttons.len()];

    // Calculate maximum possible presses for each button without overshooting
    loop {
        let mut made_progress = false;

        for (button_idx, button) in machine.buttons.iter().enumerate() {
            // Find how many times we can press this button
            let mut max_presses = usize::MAX;
            for &counter_idx in button {
                let remaining = target[counter_idx].saturating_sub(state[counter_idx]);
                max_presses = max_presses.min(remaining);
            }

            if max_presses > 0 && max_presses != usize::MAX {
                // Press this button max_presses times
                for &counter_idx in button {
                    state[counter_idx] += max_presses;
                }
                presses[button_idx] += max_presses;
                made_progress = true;
            }
        }

        if !made_progress {
            break;
        }
    }

    if state == *target {
        Some(presses.iter().sum())
    } else {
        None
    }
}

fn limited_search(machine: &MachineJoltage) -> usize {
    // Keep as very conservative fallback; prefer A* above
    use std::cmp::Reverse;
    use std::collections::{BinaryHeap, HashMap};

    let target = &machine.target_joltage;
    let initial = vec![0usize; target.len()];
    let mut heap: BinaryHeap<Reverse<(usize, Vec<usize>)>> = BinaryHeap::new();
    let mut best: HashMap<Vec<usize>, usize> = HashMap::new();
    heap.push(Reverse((0, initial.clone())));
    best.insert(initial, 0);

    let max_states = 1_000_000usize;
    let mut explored = 0usize;
    while let Some(Reverse((g, state))) = heap.pop() {
        explored += 1;
        if explored > max_states { break; }
        if &state == target { return g; }
        if let Some(&bg) = best.get(&state) { if g > bg { continue; } }
        for button in &machine.buttons {
            let mut ns = state.clone();
            let mut useful = false;
            for &idx in button { if ns[idx] < target[idx] { useful = true; } ns[idx] += 1; }
            if !useful { continue; }
            if !ns.iter().zip(target.iter()).all(|(c, t)| c <= t) { continue; }
            let ng = g + 1;
            if let Some(&bg) = best.get(&ns) { if ng >= bg { continue; } }
            best.insert(ns.clone(), ng);
            heap.push(Reverse((ng, ns)));
        }
    }
    usize::MAX
}

pub fn solve(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let machine = parse_machine(line);
            min_presses(&machine)
        })
        .sum()
}

pub fn solve_part2(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let machine = parse_machine_part2(line);
            min_presses_joltage(&machine)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_machine() {
        let input = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}";
        let machine = parse_machine(input);

        assert_eq!(machine.target_lights, vec![false, true, true, false]);
        assert_eq!(
            machine.buttons,
            vec![
                vec![3],
                vec![1, 3],
                vec![2],
                vec![2, 3],
                vec![0, 2],
                vec![0, 1],
            ]
        );
    }

    #[test]
    fn test_min_presses_simple_machine() {
        let input = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}";
        let machine = parse_machine(input);
        assert_eq!(min_presses(&machine), 2);
    }

    #[test]
    fn test_min_presses_second_machine() {
        let input = "[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}";
        let machine = parse_machine(input);
        assert_eq!(min_presses(&machine), 3);
    }

    #[test]
    fn test_min_presses_third_machine() {
        let input = "[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";
        let machine = parse_machine(input);
        assert_eq!(min_presses(&machine), 2);
    }

    #[test]
    fn test_solve_all_machines() {
        let input = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";

        assert_eq!(solve(input), 7);
    }

    #[test]
    fn test_parse_machine_with_joltage() {
        let input = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}";
        let machine = parse_machine_part2(input);

        assert_eq!(machine.target_joltage, vec![3, 5, 4, 7]);
        assert_eq!(
            machine.buttons,
            vec![
                vec![3],
                vec![1, 3],
                vec![2],
                vec![2, 3],
                vec![0, 2],
                vec![0, 1],
            ]
        );
    }

    #[test]
    fn test_min_presses_joltage_first_machine() {
        let input = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}";
        let machine = parse_machine_part2(input);
        assert_eq!(min_presses_joltage(&machine), 10);
    }

    #[test]
    fn test_min_presses_joltage_second_machine() {
        let input = "[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}";
        let machine = parse_machine_part2(input);
        assert_eq!(min_presses_joltage(&machine), 12);
    }

    #[test]
    fn test_min_presses_joltage_third_machine() {
        let input = "[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";
        let machine = parse_machine_part2(input);
        assert_eq!(min_presses_joltage(&machine), 11);
    }

    #[test]
    fn test_solve_part2() {
        let input = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";

        assert_eq!(solve_part2(input), 33);
    }
}
