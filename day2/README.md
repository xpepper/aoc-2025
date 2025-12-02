# Day 2: Gift Shop

## Problem Description

The gift shop database has invalid product IDs that need to be identified. Invalid IDs are numbers made of a sequence of digits repeated exactly twice.

### Invalid ID Pattern

An ID is invalid if it consists of some sequence of digits repeated twice:
- `55` → invalid (5 repeated twice)
- `6464` → invalid (64 repeated twice)
- `123123` → invalid (123 repeated twice)
- `101` → valid (not a repeated pattern)
- `0101` → not a valid ID (has leading zero)

### Input Format

The input contains comma-separated ranges on a single line:
```
11-22,95-115,998-1012,1188511880-1188511890,...
```

Each range is formatted as `start-end` (inclusive).

### Example

Given ranges:
```
11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124
```

Invalid IDs found:
- `11-22`: 11, 22
- `95-115`: 99
- `998-1012`: 1010
- `1188511880-1188511890`: 1188511885
- `222220-222224`: 222222
- `1698522-1698528`: none
- `446443-446449`: 446446
- `38593856-38593862`: 38593859
- Rest: none

**Sum of all invalid IDs**: 1227775554

### Task

Find all invalid IDs in the given ranges and sum them up.

# Part Two

--- Part Two ---
The clerk quickly discovers that there are still invalid IDs in the ranges in your list. Maybe the young Elf was doing other silly patterns as well?

Now, an ID is invalid if it is made only of some sequence of digits repeated at least twice. So, 12341234 (1234 two times), 123123123 (123 three times), 1212121212 (12 five times), and 1111111 (1 seven times) are all invalid IDs.

From the same example as before:

- 11-22 still has two invalid IDs, 11 and 22.
- 95-115 now has two invalid IDs, 99 and 111.
- 998-1012 now has two invalid IDs, 999 and 1010.
- 1188511880-1188511890 still has one invalid ID, 1188511885.
- 222220-222224 still has one invalid ID, 222222.
- 1698522-1698528 still contains no invalid IDs.
- 446443-446449 still has one invalid ID, 446446.
- 38593856-38593862 still has one invalid ID, 38593859.
- 565653-565659 now has one invalid ID, 565656.
- 824824821-824824827 now has one invalid ID, 824824824.
- 2121212118-2121212124 now has one invalid ID, 2121212121.
- Adding up all the invalid IDs in this example produces 4174379265.

What do you get if you add up all of the invalid IDs using these new rules?
