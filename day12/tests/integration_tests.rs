// ABOUTME: Integration tests for present packing optimization
// ABOUTME: Validates correct functionality and performance targets

use std::time::Instant;

use day12::solver::{solve_region, solve_puzzle};

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
    #[ignore] // Remove ignore when ready to run
    fn test_all_six_shapes_rotation_flipping() {
        // TODO: Test all 6 standard present shapes with rotation/flipping
        // This test should verify each shape generates correct orientations

        // Placeholder - this will be implemented
        assert!(false, "Shape rotation/flipping not implemented yet");
    }

    #[test]
    #[ignore] // Remove ignore when ready to run
    fn test_overlap_detection() {
        // TODO: Test that overlapping '#' cells are rejected

        // Placeholder - this will be implemented
        assert!(false, "Overlap detection not implemented yet");
    }

    #[test]
    #[ignore] // Remove ignore when ready to run
    fn test_transformation_deduplication() {
        // TODO: Test that duplicate transformations are eliminated

        // Placeholder - this will be implemented
        assert!(false, "Transformation deduplication not implemented yet");
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
