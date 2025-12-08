use day8::solve_playground_problem;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = fs::read_to_string("puzzle-input.txt")?;
    let result = solve_playground_problem(&input, 1000);

    println!("Day 8 Playground Problem Result: {}", result);

    Ok(())
}
