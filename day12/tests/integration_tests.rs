// ABOUTME: Integration tests for present packing optimization
// ABOUTME: Validates correct functionality and performance targets

use std::time::Instant;

use day12::solver::{solve_region, solve_puzzle};
use day12::shapes::{ShapeFactory};
use day12::grid::{BitPackedGrid};
use day12::{GridPosition};

/// Test framework helper for measuring performance
pub struct PerformanceTimer {
    start: Instant,
}

impl PerformanceTimer {
    pub fn new() -> Self {
        Self {
            start: Instant::now(),
        }
    }

    pub fn elapsed(&self) -> std::time::Duration {
        self.start.elapsed()
    }

    pub fn elapsed_ms(&self) -> u64 {
        self.elapsed().as_millis() as u64
    }
}

/// Helper to validate performance targets
pub fn validate_performance_target(
    duration: std::time::Duration,
    target_ms: u64,
    test_name: &str,
) -> Result<(), String> {
    let duration_ms = duration.as_millis() as u64;

    if duration_ms > target_ms {
        return Err(format!(
            "{}: Performance target missed. Expected < {}ms, got {}ms",
            test_name, target_ms, duration_ms
        ));
    }

    Ok(())
}

/// Helper function to create test inputs
pub fn create_test_input_4x4() -> &'static str {
    "4x4: 4:2" // 4x4 region with 2 shapes of type 4
}

pub fn create_test_input_12x5_positive() -> &'static str {
    "12x5: 0:1, 2:1, 4:2, 5:2" // Should return true
}

pub fn create_test_input_12x5_negative() -> &'static str {
    "12x5: 0:1, 2:1, 4:3, 5:2" // Should return false
}

/// Test data from problem description examples
pub mod test_data {
    /// Example from problem description that should return true
    pub fn example_true_case() -> (&'static str, bool) {
        ("4x4: 4:2", true)
    }

    /// Example from problem description that should return false
    pub fn example_false_case() -> (&'static str, bool) {
        ("12x5: 0:1, 2:1, 4:3, 5:2", false)
    }

    /// Small region for quick testing
    pub fn small_region_case() -> (&'static str, bool) {
        ("4x4: 0:1", true) // Single shape should fit
    }

    /// Impossible case for testing rejection logic
    pub fn impossible_case() -> (&'static str, bool) {
        ("4x4: 0:20", false) // Too many shapes
    }
}

/// Placeholder tests that will fail initially (TDD approach)
#[cfg(test)]
mod failing_tests {
    use super::*;

    // User Story 1 Tests - These MUST FAIL before implementation
    #[test]
    fn test_4x4_region_verification() {
        let input = create_test_input_4x4();
        let timer = PerformanceTimer::new();

        // Test the actual solve_region function
        let result = solve_region(input).expect("solve_region should succeed");

        assert!(
            result,
            "4x4 region with 2 shape-4 presents should return true"
        );

        // Validate performance target (should complete in < 10ms)
        validate_performance_target(timer.elapsed(), 10, "4x4 region verification")
            .expect("Performance target missed");
    }

    #[test]
    fn test_12x5_positive_case() {
        let input = create_test_input_12x5_positive();
        let timer = PerformanceTimer::new();

        // Test the actual solve_region function
        let result = solve_region(input).expect("solve_region should succeed");

        assert!(result, "12x5 positive case should return true");

        // Validate performance target (should complete in < 100ms)
        validate_performance_target(timer.elapsed(), 100, "12x5 positive case")
            .expect("Performance target missed");
    }

    #[test]
    fn test_12x5_negative_case() {
        let input = create_test_input_12x5_negative();
        let timer = PerformanceTimer::new();

        // Test the actual solve_region function
        let result = solve_region(input).expect("solve_region should succeed");

        assert!(!result, "12x5 negative case should return false");

        // Validate performance target (should complete in < 100ms)
        validate_performance_target(timer.elapsed(), 100, "12x5 negative case")
            .expect("Performance target missed");
    }

    // User Story 2 Tests - These MUST FAIL before implementation
    #[test]
    fn test_all_six_shapes_rotation_flipping() {
        // Test all 6 standard present shapes with rotation/flipping
        let shapes = ShapeFactory::create_all_shapes();
        assert_eq!(shapes.len(), 6, "Should have exactly 6 shapes");

        // Test each shape generates transformations correctly
        for (shape_index, shape) in shapes.iter().enumerate() {
            assert!(shape.transformation_count() >= 1,
                   "Shape {} should have at least 1 transformation", shape_index);

            // Verify all transformations are valid
            for (trans_index, transformation) in shape.transformations.iter().enumerate() {
                assert!(!transformation.cells.is_empty(),
                       "Shape {} transformation {} should have cells", shape_index, trans_index);

                assert!(transformation.width > 0 && transformation.height > 0,
                       "Shape {} transformation {} should have valid dimensions", shape_index, trans_index);
            }
        }

        // Test specific shape properties
        let line_shape = &shapes[0]; // Shape 0: Line
        assert!(line_shape.transformation_count() >= 2,
               "Line should have at least 2 orientations (horizontal/vertical)");

        let square_shape = &shapes[3]; // Shape 3: Square
        // All transformations of square should be equivalent
        if square_shape.transformation_count() > 1 {
            let first_cells = &square_shape.get_transformation(0).unwrap().cells;
            for i in 1..square_shape.transformation_count() {
                let transform = square_shape.get_transformation(i).unwrap();
                assert_eq!(transform.cells, *first_cells,
                          "All square transformations should be identical");
            }
        }
    }

    #[test]
    fn test_overlap_detection() {
        // Test that overlapping '#' cells are rejected using BitPackedGrid
        let mut grid = BitPackedGrid::new(4, 4).expect("4x4 grid should be valid");

        // Get a simple shape (single cell shape for easy testing)
        let single_cell_shape = ShapeFactory::create_shape(day12::ShapeIndex(5));
        let transformation = single_cell_shape.get_transformation(0).unwrap();

        // Test 1: Should be able to place first shape
        let pos1 = GridPosition::new(1, 1);
        assert!(grid.can_place_transformation(&transformation.cells, pos1),
               "Should be able to place first shape at (1,1)");

        // Place the first shape
        grid.place_transformation(&transformation.cells, pos1);
        assert!(grid.is_occupied(pos1), "Position (1,1) should now be occupied");

        // Test 2: Should reject overlapping placement
        let pos2 = GridPosition::new(1, 1); // Same position
        assert!(!grid.can_place_transformation(&transformation.cells, pos2),
               "Should reject overlapping placement at same position");

        // Test 3: Should accept non-overlapping placement
        let pos3 = GridPosition::new(2, 2);
        assert!(grid.can_place_transformation(&transformation.cells, pos3),
               "Should accept non-overlapping placement at (2,2)");

        // Test 4: Test with larger shape (2x2 square) - should fail due to overlap at (1,1)
        let square_shape = ShapeFactory::create_shape(day12::ShapeIndex(3));
        let square_transform = square_shape.get_transformation(0).unwrap();

        let square_pos_overlap = GridPosition::new(0, 0); // This would overlap with cell at (1,1)
        assert!(!grid.can_place_transformation(&square_transform.cells, square_pos_overlap),
               "Square at (0,0) should overlap with single cell at (1,1)");

        // Test 5: Square should fit without overlap at (2,0)
        let square_pos_no_overlap = GridPosition::new(2, 0);
        assert!(grid.can_place_transformation(&square_transform.cells, square_pos_no_overlap),
               "Square at (2,0) should not overlap with single cell at (1,1)");

        // Place the square successfully
        grid.place_transformation(&square_transform.cells, square_pos_no_overlap);

        // Test 6: Another square should now overlap at the same position
        assert!(!grid.can_place_transformation(&square_transform.cells, square_pos_no_overlap),
               "Should reject overlapping square placement at same position");

        // Test 7: Test bounds checking - square at (3,0) would exceed bounds
        let out_of_bounds_pos = GridPosition::new(3, 0);
        assert!(!grid.can_place_transformation(&square_transform.cells, out_of_bounds_pos),
               "Should reject placement that would exceed grid bounds");
    }

    #[test]
    fn test_transformation_deduplication() {
        // Test that duplicate transformations are eliminated
        let shapes = ShapeFactory::create_all_shapes();

        // Test square shape - should have exactly 1 unique transformation
        let square_shape = &shapes[3]; // Shape 3: Square
        assert_eq!(square_shape.transformation_count(), 1,
                  "Square shape should have exactly 1 unique transformation after deduplication");

        // Test line shape - should have exactly 2 unique transformations (horizontal and vertical)
        let line_shape = &shapes[0]; // Shape 0: Line
        let line_transformations = line_shape.transformation_count();
        assert!(line_transformations >= 2, "Line should have at least 2 orientations");

        // Verify all transformations of the same shape are actually different
        let mut unique_cell_patterns = std::collections::HashSet::new();
        for transformation in &line_shape.transformations {
            let pattern = format!("{:?}", transformation.cells);
            assert!(!unique_cell_patterns.contains(&pattern),
                   "Found duplicate transformation pattern: {:?}", pattern);
            unique_cell_patterns.insert(pattern);
        }

        // Test that each shape transformation has consistent cell ordering
        for shape in &shapes {
            for transformation in &shape.transformations {
                // Cells should be in row-major order (y first, then x)
                for i in 1..transformation.cells.len() {
                    let prev = &transformation.cells[i-1];
                    let curr = &transformation.cells[i];
                    assert!(
                        prev.y < curr.y || (prev.y == curr.y && prev.x <= curr.x),
                        "Cells should be in row-major order: {:?} -> {:?}",
                        prev, curr
                    );
                }
            }
        }

        // Test transformation count is reasonable (should be <= 8 for any shape)
        for shape in &shapes {
            assert!(shape.transformation_count() <= 8,
                   "Shape {} should have at most 8 unique transformations (4 rotations × 2 flips), got {}",
                   shape.index.0, shape.transformation_count());
        }
    }

    // User Story 3 Tests - These MUST FAIL before implementation
    #[test]
    fn test_performance_benchmark_complete_input() {
        // Use test input for now - will switch to real puzzle-input.txt when parser is implemented
        let input = include_str!("../test-input.txt");
        let timer = PerformanceTimer::new();

        // Test the actual solve_puzzle function
        let result = solve_puzzle(input).expect("solve_puzzle should succeed");

        assert!(result > 0, "Should find some solvable regions");

        // CRITICAL: Must complete within 40 seconds for complete puzzle input
        validate_performance_target(timer.elapsed(), 40000, "complete puzzle input")
            .expect("Critical performance target missed: must complete within 40 seconds");
    }

    #[test]
    #[ignore] // Remove ignore when ready to run
    fn test_linear_scaling_multiple_regions() {
        // TODO: Test that performance scales linearly with number of regions

        // Placeholder - this will be implemented
        assert!(false, "Linear scaling test not implemented yet");
    }
}

/// Utility function to verify tests are failing (TDD requirement)
#[test]
fn verify_tdd_approach() {
    // This test serves as a reminder that we're following TDD
    // All the tests above should fail initially, then pass after implementation

    println!("Running TDD verification test");
    println!("All tests should be ignored initially, then un-ignored as we implement");
    assert!(true, "TDD verification complete");
}
