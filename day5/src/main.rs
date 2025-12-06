use day5::solve;
use std::fs;

fn main() {
    let input = fs::read_to_string("puzzle-input.txt").expect("Failed to read input file");
    let result = solve(&input).expect("Failed to solve puzzle");
    println!("Part 1 Answer: {}", result);
}
