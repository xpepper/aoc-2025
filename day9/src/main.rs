use std::env;
use std::fs;
use std::process;

use day9::{solve_part_one, solve_part_two};

fn main() {
    let path = env::args()
        .nth(1)
        .unwrap_or_else(|| "puzzle-input.txt".to_string());
    let input = fs::read_to_string(&path).unwrap_or_else(|err| {
        eprintln!("Failed to read {}: {}", path, err);
        process::exit(1);
    });

    let part1 = solve_part_one(&input);
    let part2 = solve_part_two(&input);

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
