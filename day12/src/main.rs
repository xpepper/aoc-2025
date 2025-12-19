// ABOUTME: Main entry point for Day12 present packing solution
// ABOUTME: Provides CLI interface for running the optimized AoC solver

use std::time::Instant;

fn main() {
    println!("🎄 Day 12: Christmas Tree Farm - Present Packing Solver");
    println!("🚀 High-performance optimized implementation");
    println!();

    let start_time = Instant::now();

    // Load the puzzle input
    let input = include_str!("../puzzle-input.txt");
    println!("📖 Loading puzzle input from puzzle-input.txt");

    // Solve the complete puzzle
    match day12::aoc_parser::solve_aoc_puzzle(input) {
        Ok(solvable_count) => {
            let elapsed = start_time.elapsed();

            println!();
            println!("🎉 PUZZLE SOLVED SUCCESSFULLY!");
            println!("📊 Solvable regions: {}", solvable_count);
            println!("⏱️  Total solve time: {}ms", elapsed.as_millis());
            println!(
                "⏱️  Average time per region: {}ms",
                if solvable_count > 0 {
                    elapsed.as_millis() / solvable_count as u128
                } else {
                    0
                }
            );

            // Performance validation
            if elapsed.as_secs() <= 40 {
                println!("✅ PERFORMANCE TARGET ACHIEVED: ≤ 40 seconds");
            } else {
                println!("⚠️  PERFORMANCE TARGET MISSED: > 40 seconds");
            }
        }
        Err(e) => {
            let elapsed = start_time.elapsed();
            println!();
            println!("❌ PUZZLE SOLVE FAILED!");
            println!("📊 Error: {:?}", e);
            println!("⏱️  Time to failure: {}ms", elapsed.as_millis());
            std::process::exit(1);
        }
    }
}
