struct Machine {
    target_lights: Vec<bool>,
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

fn min_presses(machine: &Machine) -> usize {
    let num_buttons = machine.buttons.len();
    let num_lights = machine.target_lights.len();

    // Try all combinations from 0 to 2^num_buttons - 1
    for total_presses in 0..=num_buttons * 2 {
        if let Some(presses) = try_find_solution(machine, total_presses, num_buttons, num_lights) {
            return presses;
        }
    }

    usize::MAX // No solution found
}

fn try_find_solution(
    machine: &Machine,
    max_presses: usize,
    num_buttons: usize,
    num_lights: usize,
) -> Option<usize> {
    // Try all combinations with up to max_presses total presses
    try_combinations(machine, &vec![0; num_buttons], 0, max_presses, num_lights)
}

fn try_combinations(
    machine: &Machine,
    presses: &[usize],
    button_idx: usize,
    remaining: usize,
    num_lights: usize,
) -> Option<usize> {
    if button_idx == presses.len() {
        // Check if this combination produces the target
        let mut lights = vec![false; num_lights];
        let total: usize = presses.iter().sum();

        for (i, &count) in presses.iter().enumerate() {
            for _ in 0..count {
                for &light_idx in &machine.buttons[i] {
                    lights[light_idx] = !lights[light_idx];
                }
            }
        }

        if lights == machine.target_lights {
            return Some(total);
        }
        return None;
    }

    // Try different press counts for current button
    for count in 0..=remaining {
        let mut new_presses = presses.to_vec();
        new_presses[button_idx] = count;

        if let Some(result) = try_combinations(
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
}
