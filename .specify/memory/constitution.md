<!--
Sync Impact Report:
Version change: 0.0.0 → 1.0.0 (initial ratification)
List of modified principles: N/A (initial creation)
Added sections: All sections
Removed sections: N/A
Templates requiring updates: ✅ updated (.specify/templates/plan-template.md, .specify/templates/tasks-template.md)
Follow-up TODOs: N/A
-->

# Advent of Code 2025 Constitution

## Core Principles

### I. Test-Driven Development (NON-NEGOTIABLE)
TDD is mandatory for all challenge implementations: Tests written → User approved → Tests fail → Then implement. Red-Green-Refactor cycle strictly enforced with no skipping of refactor step. Small incremental steps with minimal implementation to make tests pass, followed by thorough refactoring for code quality and readability.

### II. Domain-Driven Design
Code must read like the problem domain through explicit domain modeling. Extract domain types from primitive types as patterns emerge during refactoring. Leverage Rust's type system to prevent domain errors and make code self-documenting. Use newtypes, enums, and structs to represent domain concepts rather than raw primitives.

### III. Idiomatic Rust Patterns
Always refactor towards standard library traits and Rust idioms: FromStr for parsing, Display for formatting, From/Into for conversions, and derive traits for common functionality. Follow Rust naming conventions (snake_case for functions, CamelCase for types) and leverage the type system for correctness. Always use cargo clippy and fmt for code quality.

### IV. Challenge-Based Structure
Each daily challenge is implemented as an independent Rust package in its own directory with clear separation. Challenges follow a two-part implementation approach with stopping points for user input. Each challenge includes comprehensive README documentation with problem description and solution summary.

### V. Incremental Simplicity
Prefer simple, readable solutions over clever or complex ones. Follow YAGNI principles and extract functions only when patterns emerge, not prematurely. Maintain small, focused functions with single responsibilities. Code should be easy to understand and maintain, with clear separation between algorithm steps.

## Development Standards

### Code Quality Requirements
- **Readability First**: Code should be easy to understand and read like the problem domain
- **DRY Principle**: Eliminate duplication after patterns emerge, not prematurely
- **Small Steps**: Each test should verify one small behavior with minimal implementation
- **No Skipping**: Never write production code without a failing test first
- **Refactor Mercilessly**: Always complete the refactor step with thorough code improvement

### Testing Standards
- Keep tests in the same file as implementation using `#[cfg(test)]` modules
- Use integration tests for end-to-end behavior verification when needed
- Name tests descriptively to explain what behavior they verify
- Group related tests in nested modules when appropriate
- All tests must run quickly to maintain rapid feedback

## Development Workflow

### TDD Cycle Process
1. **Red**: Write a failing test for the next small piece of functionality
2. **Green**: Write minimal code to make the test pass
3. **Refactor**: Clean up code while keeping tests green (MANDATORY - never skip)
4. **Safeguards**: Run cargo test, clippy, and fmt after each step
5. **Commit**: Separate implementation commits from refactoring commits
6. **Stop**: Ask user before starting next TDD cycle

### Commit Strategy
Use conventional commit messages with clear separation:
- `feat` when adding features or changing visible behavior
- `refactor` when changing structure without changing behavior
- `test` when editing or adding tests
- `docs` when editing or adding documents
Always separate implementation commits from refactoring commits.

## Governance

This constitution supersedes all other development practices and represents the non-negotiable ground rules for this Advent of Code 2025 project. Amendments require documentation, approval process, and migration plan to ensure consistency with TDD principles and Rust best practices.

All pull requests and reviews must verify compliance with these constitutional principles. Any complexity deviations must be explicitly justified with simpler alternatives considered and rejected. Use the `.agent/rules/aoc-2025-ground-rules.md` file for runtime development guidance and detailed implementation workflows.

**Version**: 1.0.0 | **Ratified**: 2025-12-17 | **Last Amended**: 2025-12-17