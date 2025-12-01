# Day 1: Secret Entrance

## Problem Description

The Elves need you to open the safe at the secret entrance to the North Pole base. The safe has a dial with numbers 0-99 arranged in a circle.

### Safe Dial Mechanics

- The dial starts at position **50**
- The dial is circular: going left from 0 reaches 99, going right from 99 reaches 0
- Rotations are specified as:
  - `L<n>`: rotate left (toward lower numbers) by n clicks
  - `R<n>`: rotate right (toward higher numbers) by n clicks

### Examples

- From position 11: `R8` → position 19
- From position 19: `L19` → position 0
- From position 5: `L10` → position 95
- From position 95: `R5` → position 0

### The Task

**The actual password is the number of times the dial points at 0 after any rotation in the sequence.**

### Example

Given the rotations:
```
L68 L30 R48 L5 R60 L55 L1 L99 R14 L82
```

The dial moves as follows:
- Start: 50
- L68 → 82
- L30 → 52
- R48 → **0** ✓
- L5 → 95
- R60 → 55
- L55 → **0** ✓
- L1 → 99
- L99 → **0** ✓
- R14 → 14
- L82 → 32

The dial points at 0 three times, so the password is **3**.

## Input Format

The puzzle input contains one rotation per line in the format `L<n>` or `R<n>`.
