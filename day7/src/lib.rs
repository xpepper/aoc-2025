#[derive(Debug, Clone, PartialEq)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

pub struct Grid {
    pub start: Point,
    pub cells: Vec<Vec<char>>,
    pub width: usize,
    pub height: usize,
}

impl Grid {
    pub fn get(&self, p: &Point) -> Option<char> {
        if p.y < self.height && p.x < self.width {
            Some(self.cells[p.y][p.x])
        } else {
            None
        }
    }
}

impl std::str::FromStr for Grid {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cells: Vec<Vec<char>> = s.lines().map(|line| line.chars().collect()).collect();
        if cells.is_empty() {
            return Err("Empty grid".to_string());
        }
        let height = cells.len();
        let width = cells[0].len();

        let start = cells
            .iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .map(move |(x, c)| (Point { x, y }, c))
            })
            .find(|(_, c)| **c == 'S')
            .map(|(point, _)| point)
            .ok_or_else(|| "Start point 'S' not found".to_string())?;

        Ok(Grid {
            start,
            cells,
            width,
            height,
        })
    }
}

pub fn parse(input: &str) -> Grid {
    use std::str::FromStr;
    Grid::from_str(input).expect("Invalid grid format")
}

pub fn solve(input: &str) -> u64 {
    let grid = parse(input);
    let mut beams = vec![grid.start.clone()];
    let mut splits = 0;

    while !beams.is_empty() {
        let mut next_beams = Vec::new();

        for beam in beams {
            // Move beam down
            let next_pos = Point {
                x: beam.x,
                y: beam.y + 1,
            };

            if let Some(cell) = grid.get(&next_pos) {
                match cell {
                    '^' => {
                        splits += 1;
                        // Split: create two new beams at left and right of splitter
                        // Check bounds for left beam
                        if next_pos.x > 0 {
                            next_beams.push(Point {
                                x: next_pos.x - 1,
                                y: next_pos.y,
                            });
                        }
                        // Check bounds for right beam
                        if next_pos.x + 1 < grid.width {
                            next_beams.push(Point {
                                x: next_pos.x + 1,
                                y: next_pos.y,
                            });
                        }
                    }
                    _ => {
                        // Continue down
                        next_beams.push(next_pos);
                    }
                }
            }
        }
        beams = next_beams;
    }

    splits
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

    #[test]
    fn solve_counts_single_split() {
        let input = ".S.\n.^.\n...";
        assert_eq!(solve(input), 1);
    }
}
