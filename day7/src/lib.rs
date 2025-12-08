#[derive(Debug, PartialEq)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

pub struct Grid {
    pub start: Point,
}

impl std::str::FromStr for Grid {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let start = s
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(move |(x, c)| (Point { x, y }, c))
            })
            .find(|(_, c)| *c == 'S')
            .map(|(point, _)| point)
            .ok_or_else(|| "Start point 'S' not found".to_string())?;

        Ok(Grid { start })
    }
}

pub fn parse(input: &str) -> Grid {
    use std::str::FromStr;
    Grid::from_str(input).expect("Invalid grid format")
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
