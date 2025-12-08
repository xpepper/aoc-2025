#[derive(Debug, PartialEq)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

pub struct Grid {
    pub start: Point,
}

pub fn parse(input: &str) -> Grid {
    let mut start = Point { x: 0, y: 0 };
    for (y, line) in input.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            if char == 'S' {
                start = Point { x, y };
            }
        }
    }
    Grid { start }
}

pub fn solve(_input: &str) -> u64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_finds_start_position() {
        let input = "..\n.S";
        let grid = parse(input);
        assert_eq!(grid.start, Point { x: 1, y: 1 });
    }
}
