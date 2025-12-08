#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

use std::collections::HashMap;

struct PathCounter {
    grid: Grid,
    memo: HashMap<Point, u64>,
}

impl PathCounter {
    fn new(grid: Grid) -> Self {
        Self {
            grid,
            memo: HashMap::new(),
        }
    }

    fn count(&mut self, p: Point) -> u64 {
        // Check if we are already out of bounds (should be handled by caller, but for safety)
        if p.y >= self.grid.height || p.x >= self.grid.width {
            return 1;
        }

        if let Some(&count) = self.memo.get(&p) {
            return count;
        }

        // Move down
        let next_pos = Point { x: p.x, y: p.y + 1 };

        // Check if we exited the manifold (bottom)
        if next_pos.y >= self.grid.height {
            return 1;
        }

        let count = if let Some(cell) = self.grid.get(&next_pos) {
            match cell {
                '^' => {
                    // Split: create two new beams at left and right of splitter
                    let left_count = if next_pos.x > 0 {
                        self.count(Point {
                            x: next_pos.x - 1,
                            y: next_pos.y,
                        })
                    } else {
                        1 // Hit left wall
                    };

                    let right_count = if next_pos.x + 1 < self.grid.width {
                        self.count(Point {
                            x: next_pos.x + 1,
                            y: next_pos.y,
                        })
                    } else {
                        1 // Hit right wall
                    };

                    left_count + right_count
                }
                _ => {
                    // Continue down
                    self.count(next_pos)
                }
            }
        } else {
            1 // Out of bounds (width)
        };

        self.memo.insert(p, count);
        count
    }
}

pub fn solve_part2(input: &str) -> u64 {
    let grid = parse(input);
    let start = grid.start.clone();
    let mut counter = PathCounter::new(grid);
    counter.count(start)
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

    #[test]
    fn solve_with_puzzle_input() {
        let input = include_str!("../puzzle-input.txt");
        assert_eq!(solve(input), 1600);
    }

    #[test]
    fn solve_part2_example_returns_40() {
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
        assert_eq!(solve_part2(input), 40);
    }
}
