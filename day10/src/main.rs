use day10::{solve, solve_part2};
use std::fs;

fn main() {
    let input = fs::read_to_string("puzzle-input.txt").expect("Failed to read puzzle input");
    let result1 = solve(&input);
    let result2 = solve_part2(&input);

    println!("Part 1: {}", result1);
    println!("Part 2: {}", result2);
}
