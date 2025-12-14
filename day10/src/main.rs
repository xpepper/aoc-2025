use day10::solve;
use std::fs;

fn main() {
    let input = fs::read_to_string("puzzle-input.txt").expect("Failed to read puzzle input");
    let result = solve(&input);
    println!("Part 1: {}", result);
}
