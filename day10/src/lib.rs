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
    // Try a simpler greedy approach with local search
    let target = &machine.target_joltage;
    let num_counters = target.len();
    let num_buttons = machine.buttons.len();

    // Start with a simple greedy solution
    let mut button_presses = vec![0; num_buttons];
    let mut current = vec![0; num_counters];

    // Greedy: repeatedly press the button that reduces the maximum gap
    for _ in 0..10000 { // Limit iterations
        if &current == target {
            return button_presses.iter().sum();
        }

        // Find the best button to press
        let mut best_button = None;
        let mut best_score = i32::MIN;

        for (button_idx, button) in machine.buttons.iter().enumerate() {
            // Check if pressing this button would help
            let mut score = 0;
            let mut would_overshoot = false;

            for &counter_idx in button {
                if current[counter_idx] < target[counter_idx] {
                    score += 1; // Helps this counter
                } else if current[counter_idx] >= target[counter_idx] {
                    would_overshoot = true;
                    break;
                }
            }

            if !would_overshoot && score > best_score {
                best_score = score;
                best_button = Some(button_idx);
            }
        }

        if let Some(button_idx) = best_button {
            button_presses[button_idx] += 1;
            for &counter_idx in &machine.buttons[button_idx] {
                current[counter_idx] += 1;
            }
        } else {
            // No button helps without overshooting - problem unsolvable with greedy
            break;
        }
    }

    // If greedy didn't work, try limited BFS from the greedy solution
    if &current != target {
        // Fall back to exhaustive search with small limit
        return limited_search(machine);
    }

    button_presses.iter().sum()
}

fn limited_search(machine: &MachineJoltage) -> usize {
    use std::cmp::Reverse;
    use std::collections::{BinaryHeap, HashSet};

    let target = &machine.target_joltage;
    let mut heap = BinaryHeap::new();
    let mut visited = HashSet::new();

    let initial = vec![0; target.len()];
    heap.push(Reverse((0, initial.clone())));
    visited.insert(initial);

    let max_states = 1_000_000; // Limit state exploration
    let mut states_explored = 0;

    while let Some(Reverse((presses, state))) = heap.pop() {
        if &state == target {
            return presses;
        }

        states_explored += 1;
        if states_explored > max_states {
            break;
        }

        for button in &machine.buttons {
            let mut new_state = state.clone();
            let mut useful = false;

            for &idx in button {
                if new_state[idx] < target[idx] {
                    useful = true;
                }
                new_state[idx] += 1;
            }

            if !useful {
                continue;
            }

            let valid = new_state
                .iter()
                .zip(target)
                .all(|(current, tgt)| current <= tgt);

            if valid && visited.insert(new_state.clone()) {
                heap.push(Reverse((presses + 1, new_state)));
            }
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
