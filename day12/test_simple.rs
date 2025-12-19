use day12::solver::solve_region;

fn main() {
    // Test the example from README
    println!("Testing README examples:");

    // First region: 4x4 with two shape 4's - should be TRUE
    let region1 = "4x4: 4:2";
    match solve_region(region1) {
        Ok(true) => println!("✓ Region 1 (4x4: 4:2): SOLVABLE (correct)"),
        Ok(false) => println!("✗ Region 1 (4x4: 4:2): NOT SOLVABLE (WRONG!)"),
        Err(e) => println!("✗ Region 1 error: {:?}", e),
    }

    // Second region: 12x5 with shapes - should be TRUE
    let region2 = "12x5: 0:1, 2:1, 4:2, 5:2";
    match solve_region(region2) {
        Ok(true) => println!("✓ Region 2 (12x5: 0:1, 2:1, 4:2, 5:2): SOLVABLE (correct)"),
        Ok(false) => println!("✗ Region 2 (12x5: 0:1, 2:1, 4:2, 5:2): NOT SOLVABLE (WRONG!)"),
        Err(e) => println!("✗ Region 2 error: {:?}", e),
    }

    // Third region: 12x5 with shapes - should be FALSE
    let region3 = "12x5: 0:1, 2:1, 4:3, 5:2";
    match solve_region(region3) {
        Ok(true) => println!("✗ Region 3 (12x5: 0:1, 2:1, 4:3, 5:2): SOLVABLE (WRONG!)"),
        Ok(false) => println!("✓ Region 3 (12x5: 0:1, 2:1, 4:3, 5:2): NOT SOLVABLE (correct)"),
        Err(e) => println!("✗ Region 3 error: {:?}", e),
    }
}