use day1::{solve, solve_part2};
use std::fs;

fn main() {
    let input = fs::read_to_string("rotations.txt").expect("Failed to read input file");
    let result = solve(&input);
    println!("Part 1 Answer: {}", result);
    let result_part2 = solve_part2(&input);
    println!("Part 2 Answer: {}", result_part2);
}
