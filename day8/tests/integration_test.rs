use day8::solve_playground_problem;
use std::fs;

#[test]
fn test_solve_with_puzzle_input() {
    let input = read_puzzle_input();
    let result = solve_with_validation(&input, 1000);

    // With 1000 coordinates and 1000 connections, result should be positive
    assert!(result > 0, "Result should be positive");
    println!("Puzzle result: {}", result);
}

#[test]
fn test_solve_with_puzzle_input_small_sample() {
    // Test with first few lines of puzzle input to verify our logic
    let input = "63538,35975,6036
62007,91073,28432
37830,29993,86856
78327,9264,49554";

    let result = solve_with_validation(input, 2);
    println!("Small sample result: {}", result);
}

fn read_puzzle_input() -> String {
    fs::read_to_string("puzzle-input.txt").expect(
        "Failed to read puzzle-input.txt. Make sure you're running from the correct directory.",
    )
}

fn solve_with_validation(input: &str, num_connections: usize) -> u64 {
    let result = solve_playground_problem(input, num_connections);

    // Basic validation that the result makes sense
    assert!(result > 0, "Result should be positive for valid input");

    result
}
