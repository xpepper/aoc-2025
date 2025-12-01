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


## Part 2: Method 0x434C49434B

The security protocol has changed. Now we must count **every time** the dial points at 0, regardless of whether it happens during a rotation or at the end of one.

### New Rules

- Count every click that lands on 0
- This includes passing through 0 during a rotation
- Example: From 50, `R1000` would pass 0 ten times

### Example

Given the same rotations:
```
L68 L30 R48 L5 R60 L55 L1 L99 R14 L82
```

The zero crossings are:
- L68 (50 -> 82): passes 0 **once**
- L30 (82 -> 52): no zero crossing
- R48 (52 -> 0): lands on 0 **once**
- L5 (0 -> 95): no zero crossing
- R60 (95 -> 55): passes 0 **once**
- L55 (55 -> 0): lands on 0 **once**
- L1 (0 -> 99): no zero crossing
- L99 (99 -> 0): lands on 0 **once**
- R14 (0 -> 14): no zero crossing
- L82 (14 -> 32): passes 0 **once**

Total count: **6**
