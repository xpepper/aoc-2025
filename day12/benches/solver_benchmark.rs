// ABOUTME: Performance benchmarks for present packing solver

use criterion::{Criterion, black_box, criterion_group, criterion_main};
use std::time::Duration;

// Import solver functionality when implemented
// use day12::{solve_puzzle, BitPackedGrid, Shape, Region};

/// Benchmark for 4x4 region processing (target: < 10ms)
fn benchmark_4x4_region(c: &mut Criterion) {
    c.bench_function("4x4_region", |b| {
        // TODO: Implement actual test when solver is ready
        // For now, placeholder to validate benchmark framework
        b.iter(|| {
            let input = "4x4: 4:2"; // 4x4 region with 2 shapes of type 4
            // let result = solve_region(black_box(input)).unwrap();
            // result
            42 // placeholder
        })
    });
}

/// Benchmark for 12x5 region processing (target: < 100ms per region)
fn benchmark_12x5_region(c: &mut Criterion) {
    c.bench_function("12x5_region", |b| {
        // TODO: Implement actual test when solver is ready
        b.iter(|| {
            let input = "12x5: 0:1, 2:1, 4:2, 5:2"; // 12x5 region with mixed shapes
            // let result = solve_region(black_box(input)).unwrap();
            // result
            42 // placeholder
        })
    });
}

/// Benchmark for complete puzzle input (target: < 40 seconds total)
fn benchmark_complete_puzzle_input(c: &mut Criterion) {
    let input = include_str!("../puzzle-input.txt");

    let mut group = c.benchmark_group("complete_puzzle_input");
    group.measurement_time(Duration::from_secs(60)); // Allow longer measurement
    group.sample_size(10); // Fewer samples for long-running test

    group.bench_function("solve_all_regions", |b| {
        // TODO: Implement actual test when solver is ready
        b.iter(|| {
            // let result = solve_puzzle(black_box(input)).unwrap();
            // result
            42 // placeholder
        })
    });

    group.finish();
}

/// Memory usage benchmark to ensure we stay under 100MB
fn benchmark_memory_usage(c: &mut Criterion) {
    c.bench_function("memory_usage", |b| {
        // TODO: Implement memory usage validation when solver is ready
        b.iter(|| {
            // Simulate grid allocations to test memory usage
            let mut grids = Vec::new();
            for _ in 0..100 {
                // let grid = BitPackedGrid::new(12, 5);
                // grids.push(grid);
            }
            grids.len()
        })
    });
}

/// Performance regression test to ensure we maintain 30-40 second target
fn benchmark_performance_regression(c: &mut Criterion) {
    c.bench_function("performance_regression", |b| {
        b.iter(|| {
            // This will be the final comprehensive test
            // that validates our performance targets
            // TODO: Implement when solver is complete
            1
        })
    });
}

criterion_group!(
    benches,
    benchmark_4x4_region,
    benchmark_12x5_region,
    benchmark_complete_puzzle_input,
    benchmark_memory_usage,
    benchmark_performance_regression
);

criterion_main!(benches);
