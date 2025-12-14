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

# Part Two

All of the machines are starting to come online! Now, it's time to worry about the joltage requirements.

Each machine needs to be configured to exactly the specified joltage levels to function properly. Below the buttons on each machine is a big lever that you can use to switch the buttons from configuring the indicator lights to increasing the joltage levels. (Ignore the indicator light diagrams.)

The machines each have a set of numeric counters tracking its joltage levels, one counter per joltage requirement. The counters are all initially set to zero.

So, joltage requirements like {3,5,4,7} mean that the machine has four counters which are initially 0 and that the goal is to simultaneously configure the first counter to be 3, the second counter to be 5, the third to be 4, and the fourth to be 7.

The button wiring schematics are still relevant: in this new joltage configuration mode, each button now indicates which counters it affects, where 0 means the first counter, 1 means the second counter, and so on. When you push a button, each listed counter is increased by 1.

So, a button wiring schematic like (1,3) means that each time you push that button, the second and fourth counters would each increase by 1. If the current joltage levels were {0,1,2,3}, pushing the button would change them to be {0,2,2,4}.

You can push each button as many times as you like. However, your finger is getting sore from all the button pushing, and so you will need to determine the fewest total presses required to correctly configure each machine's joltage level counters to match the specified joltage requirements.

Consider again the example from before:

```txt
[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
```

Configuring the first machine's counters requires a minimum of 10 button presses. One way to do this is by pressing (3) once, (1,3) three times, (2,3) three times, (0,2) once, and (0,1) twice.

Configuring the second machine's counters requires a minimum of 12 button presses. One way to do this is by pressing (0,2,3,4) twice, (2,3) five times, and (0,1,2) five times.

Configuring the third machine's counters requires a minimum of 11 button presses. One way to do this is by pressing (0,1,2,3,4) five times, (0,1,2,4,5) five times, and (1,2) once.

So, the fewest button presses required to correctly configure the joltage level counters on all of the machines is 10 + 12 + 11 = 33.

Analyze each machine's joltage requirements and button wiring schematics. What is the fewest button presses required to correctly configure the joltage level counters on all of the machines?

# Solution Approach

## Part 1: Linear Algebra over GF(2)
The problem asks for the minimum number of button presses to reach a target configuration where lights toggle (XOR operation). This models a system of linear equations over the binary field GF(2).

1.  **Modeling**: Each button press is a variable $x_i \in \{0, 1\}$.
    -   $Ax = b$ where $A$ is the connectivity matrix and $b$ is the target state.
2.  **Algorithm**:
    -   Use **Gaussian Elimination** to reduce the augmented matrix $[A|b]$ to row-echelon form.
    -   Identify pivot variables and free variables.
    -   If there are free variables, iterate through all $2^k$ combinations (where $k$ is the number of free variables) to find the solution with the minimum Hamming weight (minimum total presses).

## Part 2: Integer Linear Programming (ILP)
The problem changes from toggling (XOR) to adding 1. We need to find non-negative integers $x_i \ge 0$ such that $Ax = b$ over the integers, minimizing $\sum x_i$.

1.  **Modeling**:
    -   $A$ is the same connectivity matrix (0s and 1s).
    -   $b$ is the vector of joltage requirements.
    -   We need to solve $Ax = b$ for $x_i \in \mathbb{Z}_{\ge 0}$.
2.  **Algorithm**:
    -   Use **Gaussian Elimination** (over Rationals/Integers) to row-reduce the system.
    -   Express pivot variables as linear functions of free variables: $x_{pivot} = C - \sum k_j x_{free_j}$.
    -   Use a **Branch and Bound** search on the free variables:
        -   Iterate possible values for free variables.
        -   Compute implied values for pivot variables.
        -   Check if pivot variables are non-negative integers.
        -   Prune the search space if the partial sum exceeds the current best minimum.
