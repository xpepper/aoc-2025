use day12::solver::{solve_region};

fn main() {
    let input = "12x5: 0:1, 2:1, 4:3, 5:2";
    println!("Testing: {}", input);

    match solve_region(input) {
        Ok(result) => {
            println!("Result: {}", result);
            println!("Expected: false");
        }
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }
}