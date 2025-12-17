// ABOUTME: Core library for optimized present packing solution
// ABOUTME: Implements bit-packed grid and optimized algorithms for Day 12 challenge

#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]

pub mod cache;
pub mod grid;
pub mod parser;
pub mod shapes;
pub mod solver;

// Re-export error types for convenience
pub use parser::{GridError, ParseError, PlacementError, RegionError};
pub use parser::{GridResult, ParseResult, PlacementResult, RegionResult};

// Core domain types
/// Index of a present shape (0-5)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ShapeIndex(pub usize);

impl std::fmt::Display for ShapeIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Individual cell coordinate
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct Cell {
    pub x: usize,
    pub y: usize,
}

impl Cell {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

/// Position on a grid
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GridPosition {
    pub x: usize,
    pub y: usize,
}

impl GridPosition {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

impl std::fmt::Display for GridPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

/// Region dimensions
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RegionDimensions {
    pub width: usize,
    pub height: usize,
}

impl RegionDimensions {
    pub fn new(width: usize, height: usize) -> Self {
        Self { width, height }
    }

    pub fn area(&self) -> usize {
        self.width * self.height
    }
}

impl std::fmt::Display for RegionDimensions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}x{}", self.width, self.height)
    }
}

/// Validate that grid dimensions fit within 64-bit representation
pub fn validate_grid_dimensions(width: usize, height: usize) -> GridResult<()> {
    if width == 0 || height == 0 {
        return Err(GridError::InvalidDimensions(width, height));
    }

    if width * height > 64 {
        return Err(GridError::TooLarge(width, height));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shape_index_display() {
        let idx = ShapeIndex(3);
        assert_eq!(idx.to_string(), "3");
    }

    #[test]
    fn test_cell_creation() {
        let cell = Cell::new(5, 10);
        assert_eq!(cell.x, 5);
        assert_eq!(cell.y, 10);
    }

    #[test]
    fn test_grid_position_display() {
        let pos = GridPosition::new(2, 4);
        assert_eq!(pos.to_string(), "(2, 4)");
    }

    #[test]
    fn test_region_dimensions() {
        let dims = RegionDimensions::new(10, 15);
        assert_eq!(dims.width, 10);
        assert_eq!(dims.height, 15);
        assert_eq!(dims.area(), 150);
        assert_eq!(dims.to_string(), "10x15");
    }

    #[test]
    fn test_validate_grid_dimensions_valid() {
        assert!(validate_grid_dimensions(8, 8).is_ok());
        assert!(validate_grid_dimensions(1, 64).is_ok());
        assert!(validate_grid_dimensions(64, 1).is_ok());
    }

    #[test]
    fn test_validate_grid_dimensions_invalid() {
        assert!(validate_grid_dimensions(0, 5).is_err());
        assert!(validate_grid_dimensions(5, 0).is_err());
        assert!(validate_grid_dimensions(8, 9).is_err()); // 72 cells > 64
    }
}
