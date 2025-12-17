fn main() {
    let input = include_str!("../puzzle-input.txt");
    let result = day12::solve_puzzle(&input);
    println!("{}", result);
}