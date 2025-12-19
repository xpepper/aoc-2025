# aoc-2025 Development Guidelines

Auto-generated from all feature plans. Last updated: 2025-12-17

## Active Technologies
- Rust 2024 Edition (current implementation) + Standard library only (no external dependencies) (001-day12-optimization)
- Input file parsing (puzzle-input.txt ~1030 lines) (001-day12-optimization)

- Rust 2024 Edition + Standard library only (no external dependencies) (001-day12-optimization)

## Project Structure

```text
src/
tests/
```

## Commands

cargo test [ONLY COMMANDS FOR ACTIVE TECHNOLOGIES][ONLY COMMANDS FOR ACTIVE TECHNOLOGIES] cargo clippy

## Code Style

Rust 2024 Edition: Follow standard conventions

## Recent Changes
- 001-day12-optimization: Added Rust 2024 Edition (current implementation) + Standard library only (no external dependencies)

- 001-day12-optimization: Added Rust 2024 Edition + Standard library only (no external dependencies)

## TDD Ground Rules

**MANDATORY: Commit at every successful TDD cycle**
- **RED→GREEN**: Commit immediately after making failing tests pass
- **REFACTOR**: Commit immediately after successful refactoring
- **CYCLE**: Commit after each complete RED-GREEN-REFACTOR cycle
- **MESSAGE**: Use conventional commit format describing the TDD cycle

**MANDATORY: Refactor mercilessly**
- After GREEN phase, always refactor for clarity and maintainability
- Eliminate code duplication and improve naming
- Optimize data structures and algorithms while maintaining tests
- Never skip refactoring - it's a required phase of TDD
- If code can be cleaner, refactor immediately and commit

<!-- MANUAL ADDITIONS START -->
<!-- MANUAL ADDITIONS END -->
