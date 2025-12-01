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

pub struct Safe {
    pub position: u32,
}

impl Safe {
    pub fn new() -> Self {
        Safe { position: 50 }
    }

    pub fn rotate(&mut self, direction: Direction, distance: u32) {
        match direction {
            Direction::Left => {
                self.position = (self.position + 100 - (distance % 100)) % 100;
            }
            Direction::Right => {
                self.position = (self.position + distance) % 100;
            }
        }
    }
}

pub fn solve(input: &str) -> u32 {
    let mut safe = Safe::new();
    let mut zero_count = 0;

    for line in input.lines() {
        if line.trim().is_empty() {
            continue;
        }

        // We unwrap here because the input is guaranteed to be valid in the puzzle
        let rotation = parse_rotation(line.trim()).unwrap();
        safe.rotate(rotation.direction, rotation.distance);

        if safe.position == 0 {
            zero_count += 1;
        }
    }

    zero_count
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

    #[test]
    fn parse_right_rotation() {
        let rotation = parse_rotation("R48").unwrap();
        assert_eq!(rotation.direction, Direction::Right);
        assert_eq!(rotation.distance, 48);
    }

    #[test]
    fn dial_starts_at_50() {
        let safe = Safe::new();
        assert_eq!(safe.position, 50);
    }

    #[test]
    fn rotate_left_no_wrap() {
        let mut safe = Safe::new();
        safe.rotate(Direction::Left, 10);
        assert_eq!(safe.position, 40);
    }

    #[test]
    fn rotate_left_with_wrap() {
        let mut safe = Safe::new();
        safe.position = 5;
        safe.rotate(Direction::Left, 10);
        assert_eq!(safe.position, 95);
    }

    #[test]
    fn rotate_right_no_wrap() {
        let mut safe = Safe::new();
        safe.rotate(Direction::Right, 10);
        assert_eq!(safe.position, 60);
    }

    #[test]
    fn rotate_right_with_wrap() {
        let mut safe = Safe::new();
        safe.position = 95;
        safe.rotate(Direction::Right, 10);
        assert_eq!(safe.position, 5);
    }

    #[test]
    fn solve_example() {
        let input = "L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82";
        assert_eq!(solve(input), 3);
    }
}
