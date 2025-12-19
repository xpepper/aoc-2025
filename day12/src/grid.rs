// ABOUTME: High-performance bit-packed grid for present packing optimization
// ABOUTME: Provides fast cell operations using 64-bit word manipulation

use crate::{Cell, GridPosition};

/// High-performance grid representation using 64-bit words
#[derive(Debug, Clone)]
pub struct BitPackedGrid {
    pub cells: Vec<u64>, // Bit-packed grid cells
    pub width: usize,
    pub height: usize,
    pub words_per_row: usize, // Pre-computed for efficiency
}

impl BitPackedGrid {
    /// Create a new bit-packed grid with the given dimensions
    pub fn new(width: usize, height: usize) -> Result<Self, crate::parser::GridError> {
        // Validate dimensions using shared validation function
        crate::validate_grid_dimensions(width, height)
            .map_err(|_| crate::parser::GridError::TooLarge(width, height))?;

        // Calculate number of 64-bit words needed per row
        let words_per_row = (width + 63) / 64;
        let total_cells = words_per_row * height;

        Ok(BitPackedGrid {
            cells: vec![0; total_cells],
            width,
            height,
            words_per_row,
        })
    }

    /// Check if a position is occupied
    #[inline]
    pub fn is_occupied(&self, pos: GridPosition) -> bool {
        if pos.x >= self.width || pos.y >= self.height {
            return false; // Out of bounds treated as empty
        }

        let word_index = pos.y * self.words_per_row + pos.x / 64;
        let bit_index = pos.x % 64;

        (self.cells[word_index] >> bit_index) & 1 == 1
    }

    /// Set a position as occupied or empty
    #[inline]
    pub fn set_occupied(&mut self, pos: GridPosition, occupied: bool) {
        if pos.x >= self.width || pos.y >= self.height {
            return; // Ignore out of bounds
        }

        let word_index = pos.y * self.words_per_row + pos.x / 64;
        let bit_index = pos.x % 64;

        if occupied {
            self.cells[word_index] |= 1 << bit_index;
        } else {
            self.cells[word_index] &= !(1 << bit_index);
        }
    }

    /// Check if a transformation can be placed at the given position
    #[inline]
    pub fn can_place_transformation(&self, cells: &[Cell], pos: GridPosition) -> bool {
        for cell in cells {
            let absolute_x = pos.x + cell.x;
            let absolute_y = pos.y + cell.y;

            if absolute_x >= self.width || absolute_y >= self.height {
                return false; // Out of bounds
            }

            let check_pos = GridPosition {
                x: absolute_x,
                y: absolute_y,
            };
            if self.is_occupied(check_pos) {
                return false; // Cell already occupied
            }
        }
        true
    }

    /// Place a transformation at the given position
    #[inline]
    pub fn place_transformation(&mut self, cells: &[Cell], pos: GridPosition) {
        for cell in cells {
            let absolute_x = pos.x + cell.x;
            let absolute_y = pos.y + cell.y;

            if absolute_x < self.width && absolute_y < self.height {
                let set_pos = GridPosition {
                    x: absolute_x,
                    y: absolute_y,
                };
                self.set_occupied(set_pos, true);
            }
        }
    }

    /// Remove a transformation from the given position
    #[inline]
    pub fn remove_transformation(&mut self, cells: &[Cell], pos: GridPosition) {
        for cell in cells {
            let absolute_x = pos.x + cell.x;
            let absolute_y = pos.y + cell.y;

            if absolute_x < self.width && absolute_y < self.height {
                let remove_pos = GridPosition {
                    x: absolute_x,
                    y: absolute_y,
                };
                self.set_occupied(remove_pos, false);
            }
        }
    }

    /// Get grid dimensions
    pub fn dimensions(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    /// Count total occupied cells
    pub fn occupied_count(&self) -> usize {
        self.cells
            .iter()
            .map(|word| word.count_ones() as usize)
            .sum()
    }

    /// Check if grid is completely empty
    pub fn is_empty(&self) -> bool {
        self.cells.iter().all(|&word| word == 0)
    }

    /// Clear all cells
    pub fn clear(&mut self) {
        self.cells.fill(0);
    }
}

impl Default for BitPackedGrid {
    fn default() -> Self {
        BitPackedGrid::new(1, 1).unwrap()
    }
}
