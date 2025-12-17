// ABOUTME: Reference tests preserved from original implementation
// ABOUTME: Used to verify new implementation produces correct results

use std::time::Instant;

/// Preserve reference to original implementation for correctness validation
pub mod original {
    // Re-export original implementation from backup
    include!("../backup/src_original/lib.rs");
}

/// Test that validates the new implementation produces the same results as the original
#[test]
#[ignore] // Remove when new implementation is ready
fn test_new_vs_original_correctness() {
    let test_input = include_str!("../puzzle-input.txt");

    // Test original implementation
    let timer_original = Instant::now();
    let original_result = original::solve_puzzle(test_input);
    let original_time = timer_original.elapsed();

    // Test new implementation when ready
    // let timer_new = Instant::now();
    // let new_result = day12::solve_puzzle(test_input);
    // let new_time = timer_new.elapsed();

    // TODO: Uncomment when new implementation is ready
    // assert_eq!(original_result, new_result, "Results should match");

    println!("Original result: {}", original_result);
    println!("Original time: {:?}", original_time);
    // println!("New result: {}", new_result);
    // println!("New time: {:?}", new_time);

    // TODO: Add performance comparison
    // let speedup = original_time.as_nanos() as f64 / new_time.as_nanos() as f64;
    // println!("Speedup: {:.2}x", speedup);
    // assert!(speedup > 100.0, "Should achieve at least 100x speedup");
}

/// Preserve key test cases from original implementation
#[cfg(test)]
mod preserved_test_cases {
    use super::original::*;

    #[test]
    fn test_original_parse_single_shape() {
        let input = "0:\n###\n##.\n##.";
        let shape = parse_shape(input);

        assert_eq!(shape.index, 0);
        assert_eq!(
            shape.cells,
            vec![(0, 0), (1, 0), (2, 0), (0, 1), (1, 1), (0, 2), (1, 2)]
        );
    }

    #[test]
    fn test_original_4x4_region_solution() {
        let region = Region {
            width: 4,
            height: 4,
            shape_counts: vec![0, 0, 0, 0, 2, 0], // Two shape 4's
        };
        let shape4 = Shape {
            index: 4,
            cells: vec![(0, 0), (1, 0), (2, 0), (0, 1), (0, 2), (1, 2), (2, 2)],
        };
        let shapes = vec![
            Shape { index: 0, cells: vec![] },
            Shape { index: 1, cells: vec![] },
            Shape { index: 2, cells: vec![] },
            Shape { index: 3, cells: vec![] },
            shape4,
        ];

        let result = can_fit_all_shapes(&region, &shapes);
        assert!(result, "Original implementation should find solution for 4x4: 4:2");
    }

    #[test]
    fn test_original_performance_baseline() {
        let test_input = include_str!("../puzzle-input.txt");

        let timer = Instant::now();
        let result = solve_puzzle(test_input);
        let elapsed = timer.elapsed();

        println!("Original implementation result: {}", result);
        println!("Original implementation time: {:?}", elapsed);

        // This gives us a baseline for measuring our performance improvements
        // The original takes "a lot of time" - we'll measure this precisely
    }
}

/// Helper function to compare results between implementations
pub fn compare_implementations(input: &str) -> Result<(usize, usize), String> {
    let timer_original = Instant::now();
    let original_result = original::solve_puzzle(input);
    let original_time = timer_original.elapsed();

    // TODO: Compare with new implementation when ready
    // let timer_new = Instant::now();
    // let new_result = new_implementation::solve_puzzle(input);
    // let new_time = timer_new.elapsed();

    // if original_result != new_result {
    //     return Err(format!(
    //         "Results differ: original={}, new={}",
    //         original_result, new_result
    //     ));
    // }

    println!("Original: {} results in {:?}", original_result, original_time);
    // println!("New: {} results in {:?}", new_result, new_time);

    // Ok((original_time.as_millis(), new_time.as_millis()))
    Ok((original_time.as_millis() as usize, 0)) // placeholder
}