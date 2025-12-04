use day3::{solve, solve_part2};

fn main() {
    let input = include_str!("batteries.txt");
    let result = solve(input);
    println!("Part 1: {}", result);
    let result_part2 = solve_part2(input);
    println!("Part 2: {}", result_part2);
}
