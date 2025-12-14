# Day 10: Factory

## Part 1

You find a large factory across the hall. The factory machines are all offline, and the Elves can't figure out the initialization procedure.

All that remains of the manual are some indicator light diagrams, button wiring schematics, and joltage requirements for each machine.

### Problem Description

Each machine is described by a line containing:
- **Indicator light diagram** in `[square brackets]`: Shows the target state where `.` means off and `#` means on
- **Button wiring schematics** in `(parentheses)`: Each button lists which indicator lights it toggles (0-indexed)
- **Joltage requirements** in `{curly braces}`: Can be ignored for now

### Rules

1. All indicator lights start **off**
2. You can push each button an integer number of times
3. Each button press toggles the specified lights (on→off, off→on)
4. Goal: Find the **fewest total button presses** needed to configure all machines

### Examples

```
[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
```
- Machine has 4 lights, needs to be configured to: off, on, on, off
- Fewest presses: **2** (press buttons `(0,2)` and `(0,1)` once each)

```
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
```
- Machine has 5 lights, needs light 3 to be on
- Fewest presses: **3** (press `(0,4)`, `(0,1,2)`, and `(1,2,3,4)` once each)

```
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
```
- Machine has 6 lights
- Fewest presses: **2** (press `(0,3,4)` and `(0,1,2,4,5)` once each)

**Total for all three machines**: 2 + 3 + 2 = **7**

### Task

Analyze each machine's indicator light diagram and button wiring schematics. What is the **fewest button presses** required to correctly configure the indicator lights on all machines?
