# API Contracts: Day12 Performance Optimization

**Purpose**: Define internal APIs and interfaces for the present packing solution
**Date**: 2025-12-17

## Core API Interfaces

### Input Parsing API

```rust
/// Parse present shapes from input format
pub fn parse_shapes(input: &str) -> Result<Vec<Shape>, ParseError>

/// Parse regions from input format
pub fn parse_regions(input: &str, shapes: &[Shape]) -> Result<Vec<Region>, ParseError>

/// Parse complete input file
impl InputParser {
    pub fn parse(input: &str) -> Result<InputParser, ParseError>
}
```

### Shape Operations API

```rust
impl Shape {
    /// Generate all valid orientations (rotations + flips)
    pub fn generate_orientations(&self) -> Vec<Orientation>

    /// Get orientation by index
    pub fn get_orientation(&self, index: usize) -> Option<&Orientation>
}

impl Orientation {
    /// Get dimensions of this orientation
    pub fn dimensions(&self) -> (usize, usize)

    /// Get bitmask pattern for this orientation
    pub fn pattern(&self) -> &Grid
}
```

### Grid Operations API

```rust
impl Grid {
    /// Create empty grid with given dimensions
    pub fn new(width: usize, height: usize) -> Result<Grid, GridError>

    /// Check if shape orientation can be placed at position
    pub fn can_place(&self, orientation: &Orientation, position: Position) -> bool

    /// Place shape and return new grid
    pub fn place(&self, orientation: &Orientation, position: Position) -> Result<Grid, PlacementError>

    /// Check if position is empty
    pub fn is_empty(&self, position: Position) -> bool

    /// Count free cells
    pub fn count_free_cells(&self) -> usize
}
```

### Region Operations API

```rust
impl Region {
    /// Create new region from specifications
    pub fn new(width: usize, height: usize, required_shapes: Vec<(usize, usize)>) -> Result<Region, RegionError>

    /// Add placement to region
    pub fn add_placement(&mut self, placement: Placement) -> Result<(), PlacementError>

    /// Remove last placement (for backtracking)
    pub fn remove_placement(&mut self) -> Option<Placement>

    /// Check if all required shapes are placed
    pub fn is_complete(&self) -> bool

    /// Get remaining shapes to place
    pub fn get_remaining_shapes(&self) -> Vec<(usize, usize)>
}
```

### Search Algorithm API

```rust
/// Main solving function
pub fn solve_regions(parser: &InputParser) -> Vec<bool>

/// Backtracking search implementation
impl Region {
    /// Check if any valid arrangement exists
    pub fn can_fit_all_shapes(&mut self, shapes: &[Shape]) -> bool

    /// Get all valid placements for a shape
    pub fn get_valid_placements(&self, shape_id: usize, shapes: &[Shape]) -> Vec<Placement>

    /// Check if remaining shapes can possibly fit
    pub fn can_fit_remaining(&self, shapes: &[Shape]) -> bool
}
```

## Error Types

```rust
#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    InvalidShapeFormat(String),
    InvalidRegionFormat(String),
    InvalidShapeIndex(usize),
    InconsistentLineLength(String),
}

#[derive(Debug, PartialEq, Eq)]
pub enum GridError {
    InvalidDimensions(usize, usize),  // width, height
    TooLarge(usize, usize),           // width, height
}

#[derive(Debug, PartialEq, Eq)]
pub enum PlacementError {
    OutOfBounds(Position),
    Overlap(Position),
    InvalidShape(usize),
}

#[derive(Debug, PartialEq, Eq)]
pub enum RegionError {
    InvalidDimensions(usize, usize),
    InvalidShapeQuantity(usize, usize),  // shape_id, quantity
}
```

## Performance Constraints

### Time Complexity Requirements
- **Shape parsing**: O(n) where n is input size
- **Orientation generation**: O(1) per shape (max 8 orientations)
- **Placement validation**: O(1) using bitmask operations
- **Backtracking search**: Exponential worst case, but 30-40 seconds for complete puzzle input

### Space Complexity Requirements
- **Grid storage**: Fixed 64 bits per grid (max 12x5 = 60 cells)
- **Shape storage**: O(1) for all 6 shapes with orientations
- **Search stack**: O(depth) where depth is number of shapes placed
- **Total memory**: <1MB for typical inputs

### Optimization Guarantees
- **Bitmask operations**: Constant time overlap detection
- **Early pruning**: Eliminate impossible branches quickly
- **Shape ordering**: Process larger shapes first
- **Position ordering**: Try most constrained positions first