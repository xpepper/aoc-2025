#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
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

struct Simulation {
    grid: Grid,
    beams: Vec<Point>,
    splits: u64,
}

impl Simulation {
    fn new(grid: Grid) -> Self {
        let beams = vec![grid.start.clone()];
        Self {
            grid,
            beams,
            splits: 0,
        }
    }

    fn run(&mut self) -> u64 {
        while !self.beams.is_empty() {
            self.step();
        }
        self.splits
    }

    fn step(&mut self) {
        let mut next_beams = Vec::new();

        for beam in &self.beams {
            let next_pos = Point {
                x: beam.x,
                y: beam.y + 1,
            };

            if let Some(cell) = self.grid.get(&next_pos) {
                match cell {
                    '^' => {
                        self.splits += 1;
                        if next_pos.x > 0 {
                            next_beams.push(Point {
                                x: next_pos.x - 1,
                                y: next_pos.y,
                            });
                        }
                        if next_pos.x + 1 < self.grid.width {
                            next_beams.push(Point {
                                x: next_pos.x + 1,
                                y: next_pos.y,
                            });
                        }
                    }
                    _ => {
                        next_beams.push(next_pos);
                    }
                }
            }
        }
        next_beams.sort();
        next_beams.dedup();
        self.beams = next_beams;
    }
}

pub fn solve(input: &str) -> u64 {
    let grid = parse(input);
    let mut simulation = Simulation::new(grid);
    simulation.run()
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

    #[test]
    fn solve_example_returns_21() {
        let input = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";
        assert_eq!(solve(input), 21);
    }
}
