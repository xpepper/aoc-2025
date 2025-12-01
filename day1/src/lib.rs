#[derive(Debug, PartialEq)]
pub enum Direction {
    Left,
    Right,
}

pub struct Rotation {
    pub direction: Direction,
    pub distance: u32,
}

pub fn parse_rotation(input: &str) -> Result<Rotation, String> {
    if input.is_empty() {
        return Err("Input cannot be empty".to_string());
    }

    let direction = match input.chars().next().unwrap() {
        'L' => Direction::Left,
        'R' => Direction::Right,
        c => return Err(format!("Invalid direction: {}", c)),
    };

    let distance = input[1..]
        .parse::<u32>()
        .map_err(|e| format!("Invalid distance: {}", e))?;

    Ok(Rotation {
        direction,
        distance,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_left_rotation() {
        let rotation = parse_rotation("L68").unwrap();
        assert_eq!(rotation.direction, Direction::Left);
        assert_eq!(rotation.distance, 68);
    }
}
