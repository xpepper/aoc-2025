use day7::solve;
use std::fs;

fn main() {
    let input = fs::read_to_string("puzzle-input.txt").expect("Failed to read input file");
    let result = solve(&input);
    println!("Part 1 Answer: {}", result);
}
