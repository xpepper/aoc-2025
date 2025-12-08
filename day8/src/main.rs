use day8::{solve_playground_problem, solve_playground_problem_part_two};
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = fs::read_to_string("puzzle-input.txt")?;

    // Part One
    let part_one_result = solve_playground_problem(&input, 1000);
    println!("Day 8 Part One Result: {}", part_one_result);

    // Part Two
    let part_two_result = solve_playground_problem_part_two(&input);
    println!("Day 8 Part Two Result: {}", part_two_result);

    Ok(())
}
