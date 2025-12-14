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

    // Try increasing number of total presses until we find a solution
    for total_presses in 0..=num_buttons * 2 {
        if let Some(presses) = find_solution_with_max_presses(machine, total_presses) {
            return presses;
        }
    }

    usize::MAX // No solution found
}

fn find_solution_with_max_presses(machine: &Machine, max_presses: usize) -> Option<usize> {
    let num_buttons = machine.buttons.len();
    let num_lights = machine.target_lights.len();
    search_combinations(machine, &vec![0; num_buttons], 0, max_presses, num_lights)
}

fn search_combinations(
    machine: &Machine,
    presses: &[usize],
    button_idx: usize,
    remaining: usize,
    num_lights: usize,
) -> Option<usize> {
    if button_idx == presses.len() {
        return check_solution(machine, presses, num_lights);
    }

    // Try different press counts for current button
    for count in 0..=remaining {
        let mut new_presses = presses.to_vec();
        new_presses[button_idx] = count;

        if let Some(result) = search_combinations(
            machine,
            &new_presses,
            button_idx + 1,
            remaining - count,
            num_lights,
        ) {
            return Some(result);
        }
    }

    None
}

fn check_solution(machine: &Machine, presses: &[usize], num_lights: usize) -> Option<usize> {
    let lights = simulate_presses(machine, presses, num_lights);

    if lights == machine.target_lights {
        Some(presses.iter().sum())
    } else {
        None
    }
}

fn simulate_presses(machine: &Machine, presses: &[usize], num_lights: usize) -> Vec<bool> {
    let mut lights = vec![false; num_lights];

    for (button_idx, &count) in presses.iter().enumerate() {
        for _ in 0..count {
            toggle_lights(&mut lights, &machine.buttons[button_idx]);
        }
    }

    lights
}

fn toggle_lights(lights: &mut [bool], button: &[usize]) {
    for &light_idx in button {
        lights[light_idx] = !lights[light_idx];
    }
}

fn min_presses_joltage(machine: &MachineJoltage) -> usize {
    // Try increasing number of total presses until we find a solution
    for total_presses in 0..=1000 {
        if let Some(presses) = find_joltage_solution(machine, total_presses) {
            return presses;
        }
    }

    usize::MAX
}

fn find_joltage_solution(machine: &MachineJoltage, max_presses: usize) -> Option<usize> {
    let num_buttons = machine.buttons.len();
    search_joltage_combinations(machine, &vec![0; num_buttons], 0, max_presses)
}

fn search_joltage_combinations(
    machine: &MachineJoltage,
    presses: &[usize],
    button_idx: usize,
    remaining: usize,
) -> Option<usize> {
    if button_idx == presses.len() {
        return check_joltage_solution(machine, presses);
    }

    for count in 0..=remaining {
        let mut new_presses = presses.to_vec();
        new_presses[button_idx] = count;

        if let Some(result) =
            search_joltage_combinations(machine, &new_presses, button_idx + 1, remaining - count)
        {
            return Some(result);
        }
    }

    None
}

fn check_joltage_solution(machine: &MachineJoltage, presses: &[usize]) -> Option<usize> {
    let counters = simulate_joltage_presses(machine, presses);

    if counters == machine.target_joltage {
        Some(presses.iter().sum())
    } else {
        None
    }
}

fn simulate_joltage_presses(machine: &MachineJoltage, presses: &[usize]) -> Vec<usize> {
    let num_counters = machine.target_joltage.len();
    let mut counters = vec![0; num_counters];

    for (button_idx, &count) in presses.iter().enumerate() {
        for &counter_idx in &machine.buttons[button_idx] {
            counters[counter_idx] += count;
        }
    }

    counters
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
