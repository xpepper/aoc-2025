---
trigger: always_on
---

# Advent of Code 2025 - Ground Rules

## TDD Approach

We follow strict Test-Driven Development with small, incremental steps:

### Red-Green-Refactor Cycle

1. **Red**: Write a failing test for the next small piece of functionality
2. **Green**: Write the minimal code to make the test pass
3. **Refactor**: **NEVER SKIP THIS STEP**. Clean up the code while keeping tests green. This means:
   - **Extract small, focused functions**: Break down complex functions into smaller, well-named functions that do one thing
   - **Reduce cognitive load**: Make code easier to read and understand by eliminating duplication and improving naming
   - **Improve modularity**: Make the code easier to change, easier to understand (more readable), flexible (less coupled) and well composed
   - **Look for patterns**: When you see similar code repeated, extract it into a focused helper function
   - **Single Responsibility**: Each function should have a clear, single purpose

### Key Principles

- **Small Steps**: Each test should verify one small behavior
- **Minimal Implementation**: Write only enough code to pass the current test
- **No Skipping**: Never write production code without a failing test first
- **Refactor Fearlessly**: Tests give us confidence to improve code structure. Always refactor after green - it's not optional!
- **Extract Functions**: When you see similar logic or complex expressions, extract them into small, focused helper functions with descriptive names
- **Keep Tests Fast**: All tests should run quickly to maintain rapid feedback

## Rust Best Practices

- Follow Rust naming conventions (snake_case for functions, CamelCase for types)
- Leverage the type system for correctness
- Use `cargo test` for running tests
- Always run `cargo clippy -- -D warnings` for linting and getting hints for idiomatic Rust code
- Use `cargo fmt` before committing any code, for consistent formatting

## Problem-Solving Strategy

1. **Understand**: Read the problem carefully, identify inputs and outputs
2. **Examples**: Start with the provided examples as test cases
3. **Decompose**: Break down the problem into smaller sub-problems
4. **Test First**: Write tests for edge cases and examples
5. **Iterate**: Build up the solution incrementally through TDD cycles

## Test Organization

- Keep tests in the same file as implementation (using `#[cfg(test)]` module)
- If we need to test and end-to-end behaviour, we can use integration tests, which sits in a different directory, according to the Rust conventions.
- Name tests descriptively to explain what behavior they verify
- Use `assert_eq!` for equality checks with helpful error messages
- Group related tests in nested modules when appropriate

## Commit Strategy

- Use conventional commit messages
  - `feat` when adding features or changing visible behaviour
  - `refactor` when changing the structure without changing the behaviour
  - `doc` when editing or adding documents
  - `test` when editing or adding tests
- Commit after each green phase (passing tests)
- Commit every time you:
  - Add a new feature
  - Complete a single TDD cycle
  - Get a green bar that confirms progress
  - Do a refactor while tests are passing
- Write clear commit messages describing what was added/changed
- Keep commits small and focused on one behavior

## Code Quality

- **Readability First**: Code should be easy to understand
- **DRY Principle**: Don't Repeat Yourself, but only after patterns emerge
- **YAGNI**: You Aren't Gonna Need It - don't add features prematurely
- **Simplicity**: Prefer simple solutions over clever ones

## Pairing Guidelines

- Discuss approach before writing tests
- Explain reasoning for test cases
- Review code together during refactoring
- Celebrate when tests pass! 🎉

### TDD Cycle Workflow (Agent-Assisted Pairing)

When pairing with an AI agent, follow this workflow for each TDD cycle:

1. **Execute one complete TDD cycle**:
   - Write a failing test (RED)
   - Implement the simplest and minimal code that would make the test pass (GREEN)
   - **Refactor mercilessly (REFACTOR)**: This is mandatory, not optional. Extract small, focused functions, eliminate duplication, improve readability, and reduce cognitive load. Look for similar patterns and extract them into well-named helper functions.

2. **Run safeguards**:
   - `cargo test` - ensure all tests pass
   - `cargo clippy -- -D warnings` - check for linting issues
   - `cargo fmt` - format the code

3. **Commit the changes** with a descriptive conventional commit message

4. **STOP and ask the user** before starting the next TDD cycle
   - This gives the user the opportunity to review, discuss, or redirect
   - The agent should NOT proceed to the next cycle without explicit user confirmation
