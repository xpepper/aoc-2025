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

impl Default for Safe {
    fn default() -> Self {
        Self { position: 50 }
    }
}

impl Safe {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn rotate(&mut self, direction: Direction, distance: u32) -> u32 {
        match direction {
            Direction::Left => {
                let dist_to_first = if self.position == 0 {
                    100
                } else {
                    self.position
                };

                self.position = (self.position + 100 - (distance % 100)) % 100;

                if distance < dist_to_first {
                    0
                } else {
                    1 + (distance - dist_to_first) / 100
                }
            }
            Direction::Right => {
                let count = (self.position + distance) / 100;
                self.position = (self.position + distance) % 100;
                count
            }
        }
    }
}

pub fn solve(input: &str) -> u32 {
    let mut safe = Safe::new();
    let mut zero_count = 0;

    parse_and_iterate(input, |rotation| {
        safe.rotate(rotation.direction, rotation.distance);
        if safe.position == 0 {
            zero_count += 1;
        }
    });

    zero_count
}



fn parse_and_iterate(input: &str, mut processor: impl FnMut(Rotation) -> ()) {
    for line in input.lines() {
        if line.trim().is_empty() {
            continue;
        }

        // We unwrap here because the input is guaranteed to be valid in the puzzle
        let rotation = parse_rotation(line.trim()).unwrap();
        processor(rotation);
    }
}

pub fn solve_part2(input: &str) -> u32 {
    let mut safe = Safe::new();
    let mut total_crossings = 0;

    parse_and_iterate(input, |rotation| {
        total_crossings += safe.rotate(rotation.direction, rotation.distance);
    });

    total_crossings
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

    #[test]
    fn rotate_right_counts_zeros() {
        let mut safe = Safe::new(); // 50
        let crossings = safe.rotate(Direction::Right, 1000);
        assert_eq!(crossings, 10);
        assert_eq!(safe.position, 50);
    }

    #[test]
    fn rotate_left_counts_zeros() {
        let mut safe = Safe::new(); // 50
        // 50 -> 0 (needs 50)
        // Then 9 full rotations (900)
        // Total 950
        let crossings = safe.rotate(Direction::Left, 950);
        assert_eq!(crossings, 10);
        assert_eq!(safe.position, 0);
    }

    #[test]
    fn solve_part2_example() {
        let input = "L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82";
        assert_eq!(solve_part2(input), 6);
    }



    #[test]
    fn solve_counts_zero_positions_not_crossings() {
        let input = "R50\nR50";
        // After first R50: position 0 (zero_count = 1, crossings = 0)
        // After second R50: position 50 (zero_count = 1, crossings = 1)
        assert_eq!(solve(input), 1);
        assert_eq!(solve_part2(input), 1);
    }

    #[test]
    fn solve_with_rotations_txt_file() {
        let input = include_str!("../rotations.txt");
        assert_eq!(solve(input), 1055);
        assert_eq!(solve_part2(input), 6386);
    }
}
