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
}
