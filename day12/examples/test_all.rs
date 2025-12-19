use day12::solver::solve_region;

fn main() {
    println!("Testing all examples from test-input.txt:");

    let test_cases = vec![
        ("4x4: 4:2", true, "First README example"),
        ("12x5: 0:1, 2:1, 4:2, 5:2", true, "Second README example"),
        (
            "12x5: 0:1, 2:1, 4:3, 5:2",
            false,
            "Third README example (should be false)",
        ),
        (
            "8x3: 1:1, 3:1",
            true,
            "Additional test case - unknown expected",
        ),
    ];

    for (i, (input, expected, description)) in test_cases.iter().enumerate() {
        match solve_region(input) {
            Ok(result) => {
                if result == *expected {
                    println!(
                        "✓ Test {}: {} - CORRECT (got {})",
                        i + 1,
                        description,
                        result
                    );
                } else {
                    println!(
                        "✗ Test {}: {} - WRONG (got {}, expected {})",
                        i + 1,
                        description,
                        result,
                        expected
                    );
                }
            }
            Err(e) => {
                println!("✗ Test {}: {} - ERROR: {:?}", i + 1, description, e);
            }
        }
    }
}
