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

### Idiomatic Rust Patterns

During refactoring, actively look for opportunities to use standard library traits:

- **`FromStr`**: For parsing types from strings (enables `.parse()`)
  - Replace custom `parse_*` functions with `FromStr` implementations
- **`Display`**: For user-friendly string representations
- **`From`/`Into`**: For conversions between types
- **`Default`**: For default values
- **Derive traits**: Use `#[derive(...)]` for `Debug`, `Clone`, `Copy`, `PartialEq`, etc.

**Refactoring Checklist**:
- [ ] Custom parsing functions → `FromStr` trait
- [ ] Primitive types representing domain concepts → domain structs/enums
- [ ] Repeated tuple patterns → named structs
- [ ] Magic numbers/strings → const or enum variants

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
  - `refactor` when changing the structure without changing the behaviour
  - `docs` when editing or adding documents (e.g. markdown files)
  - `test` when editing or adding tests
  - `feat` when adding features or changing visible behaviour
- **Separate Commits**: Always separate implementation commits from refactoring commits.
  - Commit 1: After making the test pass (Green state). Message: `feat: ...`
  - Commit 2: After refactoring (Refactor state). Message: `refactor: ...`
- Write clear commit messages describing what was added/changed
- Keep commits small and focused on one behavior

## Code Quality

- **Readability First**: Code should be easy to understand
- **DRY Principle**: Don't Repeat Yourself, but only after patterns emerge
- **YAGNI**: You Aren't Gonna Need It - don't add features prematurely
- **Simplicity**: Prefer simple solutions over clever ones

## Domain Modeling

As we refactor, we should make domain concepts explicit in the code:

- **Extract Domain Types**: When patterns emerge, replace primitive types with domain-specific structs, enums, or type aliases
  - Example: Instead of [(i32, i32, i32)](cci:1://file:///Users/pietrodibello/Documents/workspace/kata/rust/aoc-2025/day8/src/lib.rs:0:0-4:1) tuples, create a `Coordinate` struct
  - Example: Instead of raw `Vec<f64>`, create a `Circuit` type
- **Leverage Rust's Type System**: Use the compiler to prevent domain errors
  - Create newtypes to prevent mixing up similar primitives
  - Use enums to model distinct states or variants
  - Use traits to define domain behaviors
- **Make the Code Read Like the Problem**: The codebase should reflect the domain language
  - If the problem talks about "circuits", we should have a `Circuit` type
  - If the problem mentions "junction boxes", consider a `JunctionBox` struct
  - If there are "connections", model them explicitly
- **Refactor to Domain Models During the GREEN→REFACTOR Step**:
  - Start with primitives to get tests passing quickly (GREEN)
  - Then extract domain types during refactoring to improve clarity
  - Don't prematurely create domain types before patterns emerge

This approach makes code more:
- **Readable**: Domain concepts are visible and discoverable
- **Maintainable**: Changes are localized to domain types
- **Safe**: The type system prevents misuse
- **Self-documenting**: Types communicate intent

## Pairing Guidelines

- Discuss approach before writing tests
- Explain reasoning for test cases
- Review code together during refactoring
- Celebrate when tests pass! 🎉

### TDD Cycle Workflow (Agent-Assisted Pairing)

When pairing with an AI agent, follow this workflow for each TDD cycle:

1. **Red & Green**:
   - Write a failing test (RED)
   - Implement the simplest and minimal code that would make the test pass (GREEN)
   - Run safeguards (`cargo test`, `clippy`, `fmt`)
   - **Commit**: `feat: ...`

2. **Refactor**:
   - **Refactor mercilessly**: Extract small, focused functions, eliminate duplication, improve readability.
   - Always strive for writing idiomatic Rust code, like a seasoned Rust developer.
   - Run safeguards (`cargo test`, `clippy`, `fmt`)
   - **Commit**: `refactor: ...` (if any changes were made)

3. **STOP and ask the user** before starting the next TDD cycle
   - This gives the user the opportunity to review, discuss, or redirect
   - The agent should NOT proceed to the next cycle without explicit user confirmation
