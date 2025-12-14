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
    // Use Dijkstra's algorithm to find minimum button presses
    use std::cmp::Reverse;
    use std::collections::{BinaryHeap, HashMap};

    let target = &machine.target_joltage;
    let mut heap = BinaryHeap::new();
    let mut best = HashMap::new();

    let initial = vec![0; target.len()];
    heap.push(Reverse((0, initial.clone())));
    best.insert(initial, 0);

    while let Some(Reverse((presses, state))) = heap.pop() {
        // If we reached the target, return the number of presses
        if &state == target {
            return presses;
        }

        // Skip if we've already found a better path to this state
        if let Some(&best_presses) = best.get(&state) {
            if presses > best_presses {
                continue;
            }
        }

        // Try pressing each button
        for button in &machine.buttons {
            let mut new_state = state.clone();
            for &idx in button {
                new_state[idx] += 1;
            }

            // Only explore states that don't exceed the target
            let valid = new_state
                .iter()
                .zip(target)
                .all(|(current, tgt)| current <= tgt);

            if valid {
                let new_presses = presses + 1;
                let is_better = best
                    .get(&new_state)
                    .map_or(true, |&prev| new_presses < prev);

                if is_better {
                    best.insert(new_state.clone(), new_presses);
                    heap.push(Reverse((new_presses, new_state)));
                }
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
