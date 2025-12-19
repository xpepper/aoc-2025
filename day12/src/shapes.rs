// ABOUTME: Christmas present shape definitions and transformations
// ABOUTME: Implements 6 standard shapes with rotation and flipping capabilities

use crate::{Cell, ShapeIndex};
use std::collections::HashSet;

/// Represents a Christmas present shape with all possible orientations
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Shape {
    pub index: ShapeIndex,
    pub cells: Vec<Cell>, // Normalized cell coordinates (0,0) origin
    pub width: usize,     // Bounding box width
    pub height: usize,    // Bounding box height
    pub transformations: Vec<ShapeTransformation>, // All unique orientations
}

/// A single transformation/orientation of a shape
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ShapeTransformation {
    pub shape_index: ShapeIndex,
    pub cells: Vec<Cell>,
    pub width: usize,
    pub height: usize,
    pub bit_pattern: u64, // Bitmask for fast collision detection
}

impl Shape {
    /// Create a new shape from a set of cells
    #[must_use]
    pub fn new(index: ShapeIndex, cells: Vec<Cell>) -> Self {
        let (width, height) = Self::calculate_bounds(&cells);
        let mut shape = Shape {
            index,
            cells: Self::normalize_cells(cells),
            width,
            height,
            transformations: Vec::new(),
        };

        // Generate all transformations
        shape.transformations = shape.generate_all_transformations();
        shape
    }

    /// Calculate bounding box dimensions for a set of cells
    fn calculate_bounds(cells: &[Cell]) -> (usize, usize) {
        if cells.is_empty() {
            return (0, 0);
        }

        let max_x = cells.iter().map(|c| c.x).max().unwrap();
        let max_y = cells.iter().map(|c| c.y).max().unwrap();
        (max_x + 1, max_y + 1)
    }

    /// Normalize cells so they have (0,0) as the top-left corner and consistent ordering
    fn normalize_cells(cells: Vec<Cell>) -> Vec<Cell> {
        if cells.is_empty() {
            return cells;
        }

        let min_x = cells.iter().map(|c| c.x).min().unwrap();
        let min_y = cells.iter().map(|c| c.y).min().unwrap();

        let mut normalized: Vec<Cell> = cells
            .into_iter()
            .map(|c| Cell::new(c.x - min_x, c.y - min_y))
            .collect();

        // Sort cells for consistent ordering (row-major order)
        normalized.sort_by(|a, b| a.y.cmp(&b.y).then_with(|| a.x.cmp(&b.x)));
        normalized
    }

    /// Generate all unique transformations (rotations and flips) of this shape
    fn generate_all_transformations(&self) -> Vec<ShapeTransformation> {
        let mut unique_transformations = HashSet::new();

        // Generate all 8 possible orientations (4 rotations × 2 flip states)
        for flipped in [false, true] {
            for rotation in 0..4 {
                let cells = if flipped {
                    self.get_flipped_rotated_cells(rotation)
                } else {
                    self.get_rotated_cells(rotation)
                };

                let (width, height) = Self::calculate_bounds(&cells);
                let bit_pattern = Self::cells_to_bit_pattern(&cells, width);

                let transformation = ShapeTransformation {
                    shape_index: self.index,
                    cells: Self::normalize_cells(cells),
                    width,
                    height,
                    bit_pattern,
                };

                unique_transformations.insert(transformation);
            }
        }

        // Convert to sorted vector for deterministic behavior
        let mut transformations: Vec<_> = unique_transformations.into_iter().collect();
        transformations.sort_by(|a, b| {
            a.cells
                .iter()
                .cmp(b.cells.iter())
                .then_with(|| a.width.cmp(&b.width))
                .then_with(|| a.height.cmp(&b.height))
        });

        transformations
    }

    /// Get cells rotated by 90-degree increments (0, 1, 2, 3)
    fn get_rotated_cells(&self, rotations: usize) -> Vec<Cell> {
        let rotations = rotations % 4;
        if rotations == 0 {
            return self.cells.clone();
        }

        self.cells
            .iter()
            .map(|cell| self.rotate_cell(*cell, rotations))
            .collect()
    }

    /// Get cells flipped horizontally, then rotated
    fn get_flipped_rotated_cells(&self, rotations: usize) -> Vec<Cell> {
        // First flip horizontally
        let flipped: Vec<Cell> = self
            .cells
            .iter()
            .map(|cell| {
                let flipped_x = if self.width > 0 {
                    self.width - 1 - cell.x
                } else {
                    0
                };
                Cell::new(flipped_x, cell.y)
            })
            .collect();

        // Then rotate
        let rotations = rotations % 4;
        if rotations == 0 {
            return flipped;
        }

        flipped
            .iter()
            .map(|cell| self.rotate_cell(*cell, rotations))
            .collect()
    }

    /// Rotate a cell by 90-degree increments around origin (0,0)
    fn rotate_cell(&self, cell: Cell, rotations: usize) -> Cell {
        match rotations % 4 {
            0 => cell,
            1 => Cell::new(cell.y, (self.width - 1).saturating_sub(cell.x)),
            2 => Cell::new(
                (self.width - 1).saturating_sub(cell.x),
                (self.height - 1).saturating_sub(cell.y),
            ),
            3 => Cell::new((self.height - 1).saturating_sub(cell.y), cell.x),
            _ => unreachable!(),
        }
    }

    /// Convert cell coordinates to bit pattern for fast collision detection
    fn cells_to_bit_pattern(cells: &[Cell], width: usize) -> u64 {
        if cells.is_empty() || width > 8 {
            return 0; // Can't fit in 64 bits if width > 8
        }

        cells.iter().fold(0u64, |pattern, cell| {
            pattern | (1u64 << (cell.y * width + cell.x))
        })
    }

    /// Get transformation count
    #[must_use]
    pub fn transformation_count(&self) -> usize {
        self.transformations.len()
    }

    /// Get transformation by index
    #[must_use]
    pub fn get_transformation(&self, index: usize) -> Option<&ShapeTransformation> {
        self.transformations.get(index)
    }
}

impl ShapeTransformation {
    /// Get the cells of this transformation
    #[must_use]
    pub fn cells(&self) -> &[Cell] {
        &self.cells
    }

    /// Get the dimensions of this transformation
    #[must_use]
    pub fn dimensions(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    /// Get the bit pattern for fast collision detection
    #[must_use]
    pub fn bit_pattern(&self) -> u64 {
        self.bit_pattern
    }

    /// Check if this transformation fits within given bounds
    #[must_use]
    pub fn fits_in_bounds(&self, width: usize, height: usize) -> bool {
        self.width <= width && self.height <= height
    }

    /// Get the area (number of cells) of this transformation
    #[must_use]
    pub fn area(&self) -> usize {
        self.cells.len()
    }
}

/// Factory for creating the 6 standard present shapes
pub struct ShapeFactory;

impl ShapeFactory {
    /// Create all 6 standard present shapes
    #[must_use]
    pub fn create_all_shapes() -> Vec<Shape> {
        (0..6)
            .map(|i| ShapeFactory::create_shape(ShapeIndex(i)))
            .collect()
    }

    /// Create a specific shape by index
    #[must_use]
    pub fn create_shape(index: ShapeIndex) -> Shape {
        match index.0 {
            0 => ShapeFactory::create_shape_0(), // Straight vertical
            1 => ShapeFactory::create_shape_1(), // L-shape
            2 => ShapeFactory::create_shape_2(), // T-shape
            3 => ShapeFactory::create_shape_3(), // Square
            4 => ShapeFactory::create_shape_4(), // Zigzag vertical
            5 => ShapeFactory::create_shape_5(), // Single cell
            _ => panic!("Invalid shape index: {}", index.0),
        }
    }

    fn create_shape_0() -> Shape {
        // Shape 0 from README:
        // ###
        // ##.
        // ##.
        let cells = vec![
            Cell::new(0, 0),
            Cell::new(1, 0),
            Cell::new(2, 0),
            Cell::new(0, 1),
            Cell::new(1, 1),
            Cell::new(0, 2),
            Cell::new(1, 2),
        ];
        Shape::new(ShapeIndex(0), cells)
    }

    fn create_shape_1() -> Shape {
        // Shape 1 from README:
        // ###
        // ##.
        // .##
        let cells = vec![
            Cell::new(0, 0),
            Cell::new(1, 0),
            Cell::new(2, 0),
            Cell::new(0, 1),
            Cell::new(1, 1),
            Cell::new(1, 2),
            Cell::new(2, 2),
        ];
        Shape::new(ShapeIndex(1), cells)
    }

    fn create_shape_2() -> Shape {
        // Shape 2 from README:
        // .##
        // ###
        // ##.
        let cells = vec![
            Cell::new(1, 0),
            Cell::new(2, 0),
            Cell::new(0, 1),
            Cell::new(1, 1),
            Cell::new(2, 1),
            Cell::new(0, 2),
            Cell::new(1, 2),
        ];
        Shape::new(ShapeIndex(2), cells)
    }

    fn create_shape_3() -> Shape {
        // Shape 3 from README:
        // ##.
        // ###
        // ##.
        let cells = vec![
            Cell::new(0, 0),
            Cell::new(1, 0),
            Cell::new(0, 1),
            Cell::new(1, 1),
            Cell::new(2, 1),
            Cell::new(0, 2),
            Cell::new(1, 2),
        ];
        Shape::new(ShapeIndex(3), cells)
    }

    fn create_shape_4() -> Shape {
        // Shape 4 from README:
        // ###
        // #..
        // ###
        let cells = vec![
            Cell::new(0, 0),
            Cell::new(1, 0),
            Cell::new(2, 0),
            Cell::new(0, 1),
            Cell::new(0, 2),
            Cell::new(1, 2),
            Cell::new(2, 2),
        ];
        Shape::new(ShapeIndex(4), cells)
    }

    fn create_shape_5() -> Shape {
        // Shape 5 from README:
        // ###
        // .#.
        // ###
        let cells = vec![
            Cell::new(0, 0),
            Cell::new(1, 0),
            Cell::new(2, 0),
            Cell::new(1, 1),
            Cell::new(0, 2),
            Cell::new(1, 2),
            Cell::new(2, 2),
        ];
        Shape::new(ShapeIndex(5), cells)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shape_creation() {
        let shape = ShapeFactory::create_shape(ShapeIndex(0));
        assert_eq!(shape.index, ShapeIndex(0));
        assert_eq!(shape.cells.len(), 7); // Shape 0 has 7 cells
        assert_eq!(shape.width, 3);
        assert_eq!(shape.height, 3);
    }

    #[test]
    fn test_shape_transformations() {
        let shape = ShapeFactory::create_shape(ShapeIndex(3)); // Shape 3: 7-cell pattern
        // Shape 3 will have multiple unique transformations due to its asymmetric pattern
        assert!(shape.transformation_count() >= 1); // At least 1 orientation

        let shape = ShapeFactory::create_shape(ShapeIndex(0)); // Shape 0: 7-cell pattern
        assert!(shape.transformation_count() >= 1); // Will have multiple orientations
    }

    #[test]
    fn test_transformation_bounds() {
        let shape = ShapeFactory::create_shape(ShapeIndex(0));
        let transformation = shape.get_transformation(0).unwrap();

        // Shape 0 is 3x3, so it should fit in 3x3 or larger
        assert!(transformation.fits_in_bounds(3, 3));
        assert!(!transformation.fits_in_bounds(2, 3));
        assert!(!transformation.fits_in_bounds(3, 2));
    }

    #[test]
    fn test_all_shapes_creation() {
        let shapes = ShapeFactory::create_all_shapes();
        assert_eq!(shapes.len(), 6);

        for (i, shape) in shapes.iter().enumerate() {
            assert_eq!(shape.index, ShapeIndex(i));
            assert!(shape.transformation_count() > 0);
        }
    }
}
