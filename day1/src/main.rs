use day1::solve;
use std::fs;

fn main() {
    let input = fs::read_to_string("rotations.txt").expect("Failed to read input file");
    let result = solve(&input);
    println!("Part 1 Answer: {}", result);
}
