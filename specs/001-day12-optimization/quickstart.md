# Quickstart Guide: Day12 Performance Optimization

**Purpose**: Rapid onboarding for implementing the optimized present packing solution
**Date**: 2025-12-17
**Branch**: 001-day12-optimization

## Implementation Strategy

This implementation follows a **from-scratch approach** to achieve optimal performance while maintaining correctness. The existing implementation serves as a reference for test cases only.

## Phase 1: Core Infrastructure (2-3 hours)

### Step 1: Bit-Packed Grid Implementation

**Priority**: Critical - This alone provides 10-20x speedup

```rust
// src/grid.rs
pub struct BitPackedGrid {
    cells: Vec<u64>,
    width: usize,
    height: usize,
    words_per_row: usize,
}

impl BitPackedGrid {
    pub fn new(width: usize, height: usize) -> Self { /* implementation */ }

    #[inline]
    pub fn is_occupied(&self, x: usize, y: usize) -> bool { /* bit test */ }

    #[inline]
    pub fn can_place_transformation(&self, trans: &ShapeTransformation, x: usize, y: usize) -> bool {
        // Fast bit overlap checking
    }
}
```

**Tests to Write**:
- Grid creation and sizing
- Individual cell operations
- Bulk shape placement and removal
- Performance benchmarks vs Vec<bool>

### Step 2: Shape Processing System

```rust
// src/shapes.rs
pub struct Shape {
    index: ShapeIndex,
    cells: Vec<Cell>,
    width: usize,
    height: usize,
    transformations: Vec<ShapeTransformation>,
}

pub struct ShapeTransformation {
    shape_index: ShapeIndex,
    cells: Vec<Cell>,
    width: usize,
    height: usize,
    bit_pattern: u64,  // Pre-computed for collision detection
}
```

**Key Functions to Implement**:
- `parse_shape_definition()` - Parse visual representation
- `generate_transformations()` - All rotations/flips with deduplication
- `compute_bit_pattern()` - Fast collision detection
- `normalize_coordinates()` - Consistent positioning

### Step 3: Input Parsing

```rust
// src/parser.rs
pub fn parse_input(input: &str) -> ParseResult<(Vec<Shape>, Vec<Region>)> {
    // Parse shapes first, then regions
    // Validate format consistency
    // Return structured data
}
```

## Phase 2: Search Optimization (2-3 hours)

### Step 4: Basic Solver

```rust
// src/solver.rs
pub struct Solver {
    shapes: Vec<Shape>,
    grid: BitPackedGrid,
    instances: Vec<ShapeInstance>,
}

impl Solver {
    pub fn can_fit_all_shapes(region: &Region, shapes: &[Shape]) -> bool {
        // Initialize grid and instances
        // Order shapes by constraint level
        // Run optimized backtracking
    }

    fn backtrack(&mut self, instances: &mut [ShapeInstance]) -> bool {
        // Recursive search with pruning
        // Try most constrained shapes first
        // Early termination on impossible states
    }
}
```

### Step 5: Memoization System

```rust
// src/cache.rs
pub struct MemoizationCache {
    cache: HashMap<u64, bool>,
    hasher: ZobristHasher,
}

pub struct ZobristHasher {
    table: Vec<u64>,  // Pre-computed random values
}
```

**Implementation Priority**:
1. Basic cache lookup/store
2. Incremental hash computation
3. Cache size management
4. Performance tuning

## Phase 3: Advanced Pruning (1-2 hours)

### Step 6: Intelligent Ordering

```rust
// impl Solver
fn reorder_by_constraints(&mut self) {
    // Compute placement options for each shape
    // Sort by fewest options first (min-fit heuristic)
    // Dynamic reordering during search
}
```

### Step 7: Bounds Checking

```rust
// Advanced pruning heuristics
fn can_fit_remaining_area(&self, instances: &[ShapeInstance]) -> bool {
    // Area-based pruning
}

fn has_isolated_empty_regions(&self) -> bool {
    // Connected component analysis
    // Detect unpackable holes
}
```

## Testing Strategy

### Reference Tests (Preserve Existing)

```bash
# Run existing tests to ensure correctness
cargo test --lib

# Reference implementation tests
cargo test -- --ignored ref_tests
```

### Performance Benchmarks

```rust
// benches/solver_benchmark.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn benchmark_solver(c: &mut Criterion) {
    let input = include_str!("../puzzle-input.txt");
    let (shapes, regions) = parse_input(input).unwrap();

    c.bench_function("solve_all_regions", |b| {
        b.iter(|| {
            regions.iter().filter(|r| can_fit_all_shapes(black_box(r), &shapes)).count()
        })
    });
}

criterion_group!(benches, benchmark_solver);
criterion_main!(benches);
```

### Acceptance Criteria

1. **Functional**: All reference tests pass
2. **Performance**: Complete puzzle input in ≤ 40 seconds
3. **Memory**: Peak usage < 100MB
4. **Code**: Follows constitution guidelines

## Development Workflow

### 1. Setup Environment

```bash
# Start fresh implementation
rm -rf day12/src/*
mkdir -p day12/src day12/tests day12/benches

# Create module structure
touch day12/src/lib.rs day12/src/main.rs
touch day12/src/{grid,shapes,parser,solver,cache}.rs
```

### 2. Incremental Development

**Red-Green-Refactor Cycle**:
1. **Red**: Write failing test for next component
2. **Green**: Implement minimal code to pass test
3. **Refactor**: Improve design while maintaining tests

**Example Flow**:
```rust
// First: Write test
#[test]
fn test_bit_packed_grid_creation() {
    let grid = BitPackedGrid::new(4, 4);
    assert_eq!(grid.width, 4);
    assert_eq!(grid.height, 4);
    assert_eq!(grid.cells.len(), 1); // 4*4/64 = 1 word
}

// Second: Implement minimal code
impl BitPackedGrid {
    pub fn new(width: usize, height: usize) -> Self {
        let words_needed = (width * height + 63) / 64;
        Self {
            cells: vec![0u64; words_needed],
            width,
            height,
            words_per_row: (width + 63) / 64,
        }
    }
}

// Third: Refactor for performance and clarity
```

### 3. Performance Validation

```bash
# Run performance benchmarks
cargo bench

# Profile with perf (Linux) or instruments (macOS)
cargo build --release
perf record --call-graph=dwarf ./target/release/day12

# Memory usage check
valgrind --tool=massif ./target/release/day12
```

### 4. Integration Testing

```bash
# Test against full puzzle input
time cargo run --release

# Should complete within 30-40 seconds
# Expected output: "Solution: X" where X is region count
```

## Critical Success Factors

### Performance Optimization Priority

1. **Bit-packed grid** (10-20x improvement) - **IMPLEMENT FIRST**
2. **Memoization** (5-50x improvement) - **IMPLEMENT SECOND**
3. **Intelligent ordering** (3-10x improvement) - **IMPLEMENT THIRD**
4. **Advanced pruning** (5-20x improvement) - **IMPLEMENT LAST**

### Common Pitfalls to Avoid

1. **Premature optimization** - Get correctness first, then optimize
2. **Complex abstractions** - Prefer simple, direct implementations
3. **Memory leaks** - Monitor cache size and cleanup
4. **Incorrect bit operations** - Test thoroughly, use property tests
5. **Skipping refactoring** - Complete the Red-Green-Refactor cycle

### Performance Validation Checklist

- [ ] Bit-packed grid operations are O(1)
- [ ] Shape transformations are pre-computed
- [ ] Memoization cache has size limits
- [ ] Search ordering reduces branching factor
- [ ] Bounds checking eliminates impossible branches
- [ ] Complete input processes in ≤ 40 seconds
- [ ] Memory usage stays within reasonable bounds
- [ ] All reference tests pass

## Next Steps

1. **Start Phase 1** with bit-packed grid implementation
2. **Write comprehensive tests** for each component
3. **Benchmark against reference** implementation
4. **Iterate based on performance results**
5. **Validate against full puzzle input**

This quickstart provides a clear path to achieving the performance targets while maintaining code quality and correctness through TDD principles.