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

#[derive(Debug)]
pub enum Interaction {
    Continue(Point),
    Split(Option<Point>, Option<Point>),
    Terminated,
}

impl Grid {
    pub fn get(&self, p: &Point) -> Option<char> {
        if p.y < self.height && p.x < self.width {
            Some(self.cells[p.y][p.x])
        } else {
            None
        }
    }

    pub fn interact(&self, p: &Point) -> Interaction {
        let next_y = p.y + 1;
        if next_y >= self.height {
            return Interaction::Terminated;
        }

        let next_pos = Point { x: p.x, y: next_y };
        match self.get(&next_pos) {
            Some('^') => {
                let left = if next_pos.x > 0 {
                    Some(Point {
                        x: next_pos.x - 1,
                        y: next_pos.y,
                    })
                } else {
                    None
                };
                let right = if next_pos.x + 1 < self.width {
                    Some(Point {
                        x: next_pos.x + 1,
                        y: next_pos.y,
                    })
                } else {
                    None
                };
                Interaction::Split(left, right)
            }
            Some(_) => Interaction::Continue(next_pos),
            None => Interaction::Terminated,
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
            match self.grid.interact(beam) {
                Interaction::Split(left, right) => {
                    self.splits += 1;
                    if let Some(p) = left {
                        next_beams.push(p);
                    }
                    if let Some(p) = right {
                        next_beams.push(p);
                    }
                }
                Interaction::Continue(p) => {
                    next_beams.push(p);
                }
                Interaction::Terminated => {}
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

        let count = match self.grid.interact(&p) {
            Interaction::Split(left, right) => {
                let left_count = left.map(|p| self.count(p)).unwrap_or(1);
                let right_count = right.map(|p| self.count(p)).unwrap_or(1);
                left_count + right_count
            }
            Interaction::Continue(next_p) => self.count(next_p),
            Interaction::Terminated => 1,
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

    #[test]
    fn solve_part2_with_puzzle_input() {
        let input = include_str!("../puzzle-input.txt");
        assert_eq!(solve_part2(input), 8632253783011);
    }
}
