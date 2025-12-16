use day11::solve_part1;
use std::fs;

fn main() {
    let input = fs::read_to_string("puzzle-input.txt").unwrap_or_else(|_| {
        eprintln!("Warning: puzzle-input.txt not found, using empty string");
        String::new()
    });

    println!("Part 1: {}", solve_part1(&input));
}
