struct Machine {
    target_lights: Vec<bool>,
    buttons: Vec<Vec<usize>>,
}

fn parse_machine(input: &str) -> Machine {
    let parts: Vec<&str> = input.split(['[', ']', '{', '}'])
        .filter(|s| !s.is_empty())
        .collect();
    
    // Parse target lights from the first part
    let lights_str = parts[0];
    let target_lights: Vec<bool> = lights_str.chars()
        .filter(|c| *c == '.' || *c == '#')
        .map(|c| c == '#')
        .collect();
    
    // Parse buttons from the middle part
    let buttons_str = parts[1].trim();
    let buttons: Vec<Vec<usize>> = buttons_str
        .split(") (")
        .map(|button_str| {
            button_str
                .trim_matches(|c| c == '(' || c == ')')
                .split(',')
                .map(|n| n.trim().parse().unwrap())
                .collect()
        })
        .collect();
    
    Machine { target_lights, buttons }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_machine() {
        let input = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}";
        let machine = parse_machine(input);
        
        assert_eq!(machine.target_lights, vec![false, true, true, false]);
        assert_eq!(machine.buttons, vec![
            vec![3],
            vec![1, 3],
            vec![2],
            vec![2, 3],
            vec![0, 2],
            vec![0, 1],
        ]);
    }
}
