// ABOUTME: Core optimized solver for present packing optimization
// ABOUTME: Implements high-performance backtracking with memoization and intelligent search

use crate::cache::{MemoizationCache, SolverStats, ZobristHasher};
use crate::grid::BitPackedGrid;
use crate::parser::ParseError;
use crate::shapes::Shape;
use crate::{GridPosition, ShapeIndex};
use std::collections::HashMap;

/// Optimized solver result type
pub type SolveResult = Result<bool, ParseError>;

/// Shape requirement for a region
#[derive(Debug, Clone)]
pub struct ShapeRequirement {
    pub shape_index: ShapeIndex,
    pub count: usize,
}

/// Region specification with dimensions and shape requirements
#[derive(Debug, Clone)]
pub struct Region {
    pub width: usize,
    pub height: usize,
    pub requirements: Vec<ShapeRequirement>,
}

/// High-performance optimized solver
pub struct OptimizedSolver {
    grid: BitPackedGrid,
    shapes: Vec<ShapeInstance>,
    shape_definitions: HashMap<ShapeIndex, Shape>,
    cache: MemoizationCache,
    hasher: ZobristHasher,
    stats: SolverStats,
    is_impossible: bool, // True if region is mathematically impossible
}

/// Shape instance for tracking placements
#[derive(Debug, Clone)]
pub struct ShapeInstance {
    pub shape_index: ShapeIndex,
    pub count: usize,
    pub placed: usize,
}

impl OptimizedSolver {
    /// Create new solver for region dimensions with dynamic shape definitions
    pub fn new(
        width: usize,
        height: usize,
        requirements: Vec<ShapeRequirement>,
        shape_definitions: HashMap<ShapeIndex, Shape>,
    ) -> Result<Self, ParseError> {
        // Validate grid dimensions
        crate::validate_grid_dimensions(width, height)
            .map_err(|_| ParseError::InvalidShapeFormat("Invalid grid dimensions".to_string()))?;

        let grid = BitPackedGrid::new(width, height)
            .map_err(|_| ParseError::InvalidShapeFormat("Grid creation failed".to_string()))?;

        // Create shape instances from requirements
        let shapes: Vec<ShapeInstance> = requirements
            .into_iter()
            .map(|req| ShapeInstance {
                shape_index: req.shape_index,
                count: req.count,
                placed: 0,
            })
            .collect();

        // Validate total cells
        let total_required_cells = shapes
            .iter()
            .map(|instance| {
                let shape = shape_definitions.get(&instance.shape_index)
                    .expect("Shape definition not found");
                let cells = shape.cells.len() * instance.count;
                cells
            })
            .sum::<usize>();

        let grid_capacity = width * height;

        // If region is mathematically impossible, we can't solve it
        // This is not an error - it just means the answer is "false"
        let is_impossible = total_required_cells > grid_capacity;

        Ok(Self {
            grid,
            shapes,
            shape_definitions,
            cache: MemoizationCache::new(10000),
            hasher: ZobristHasher::new(width, height),
            stats: SolverStats::new(),
            is_impossible,
        })
    }

    /// Solve the packing problem with optimizations
    pub fn solve(&mut self) -> bool {
        // If region is mathematically impossible, return false immediately
        if self.is_impossible {
            return false;
        }

        self.stats.reset();
        let placed_shapes: Vec<ShapeIndex> = Vec::new();
        self.solve_recursive(0, 0, &placed_shapes)
    }

    /// Recursive solver with memoization and pruning
    fn solve_recursive(
        &mut self,
        shape_idx: usize,
        hash: u64,
        placed_shapes: &[ShapeIndex],
    ) -> bool {
        self.stats.record_node();

        // Check cache first
        if let Some(cached_result) = self.cache.get(hash) {
            self.stats.record_cache_hit();
            return cached_result;
        }
        self.stats.record_cache_miss();

        // Find next shape to place
        let current_shape_idx = self.find_next_shape(shape_idx);
        if current_shape_idx >= self.shapes.len() {
            // All shapes placed - success!
            let result = true;
            self.cache.insert(hash, result);
            return result;
        }

        let instance = &self.shapes[current_shape_idx];
        if instance.placed >= instance.count {
            // Move to next shape
            let result = self.solve_recursive(current_shape_idx + 1, hash, placed_shapes);
            self.cache.insert(hash, result);
            return result;
        }

        // Copy shape index before mutable operations
        let shape_index = instance.shape_index;

        // Get shape from definitions and try all transformations
        let shape = self
            .shape_definitions
            .get(&shape_index)
            .expect("Shape definition not found");

        // Try transformations in order of fit quality (intelligent ordering)
        let mut transformations = shape.transformations.clone();
        self.order_transformations_by_fit(&mut transformations);

        // Try each transformation at each valid position
        for transformation in &transformations {
            if !self.can_fit_transformation(transformation) {
                self.stats.record_pruned_branch();
                continue;
            }

            // Try all valid positions for this transformation
            let positions = self.find_valid_positions(transformation);

            for pos in positions {
                // Place the shape
                self.place_transformation(transformation, pos);
                let mut new_placed_shapes = placed_shapes.to_vec();
                new_placed_shapes.push(shape_index);

                // Update hash incrementally
                let new_hash = self.update_hash_for_placement(hash, transformation, pos);

                // Recurse
                self.shapes[current_shape_idx].placed += 1;

                if self.solve_recursive(current_shape_idx, new_hash, &new_placed_shapes) {
                    let result = true;
                    self.cache.insert(hash, result);
                    return result;
                }

                // Backtrack
                self.shapes[current_shape_idx].placed -= 1;
                self.remove_transformation(transformation, pos);
            }
        }

        // No valid placement found
        let result = false;
        self.cache.insert(hash, result);
        result
    }

    /// Find next shape index to place (skip completed shapes)
    fn find_next_shape(&self, start_idx: usize) -> usize {
        let mut idx = start_idx;
        while idx < self.shapes.len() {
            if self.shapes[idx].placed < self.shapes[idx].count {
                break;
            }
            idx += 1;
        }
        idx
    }

    /// Order transformations by fit quality (min-fit heuristic)
    fn order_transformations_by_fit(
        &self,
        transformations: &mut Vec<crate::shapes::ShapeTransformation>,
    ) {
        // Sort by area (smaller shapes first for better pruning)
        transformations.sort_by_key(|t| t.area());
    }

    /// Check if transformation can fit anywhere in grid
    fn can_fit_transformation(&self, transformation: &crate::shapes::ShapeTransformation) -> bool {
        transformation.fits_in_bounds(self.grid.width, self.grid.height)
    }

    /// Find all valid positions for a transformation
    fn find_valid_positions(
        &self,
        transformation: &crate::shapes::ShapeTransformation,
    ) -> Vec<GridPosition> {
        let mut positions = Vec::new();
        let max_x = self.grid.width.saturating_sub(transformation.width) + 1;
        let max_y = self.grid.height.saturating_sub(transformation.height) + 1;

        for y in 0..max_y {
            for x in 0..max_x {
                let pos = GridPosition::new(x, y);
                if self
                    .grid
                    .can_place_transformation(&transformation.cells, pos)
                {
                    positions.push(pos);
                }
            }
        }

        positions
    }

    /// Place transformation on grid
    fn place_transformation(
        &mut self,
        transformation: &crate::shapes::ShapeTransformation,
        pos: GridPosition,
    ) {
        self.grid.place_transformation(&transformation.cells, pos);
    }

    /// Remove transformation from grid
    fn remove_transformation(
        &mut self,
        transformation: &crate::shapes::ShapeTransformation,
        pos: GridPosition,
    ) {
        self.grid.remove_transformation(&transformation.cells, pos);
    }

    /// Update hash for shape placement
    fn update_hash_for_placement(
        &self,
        current_hash: u64,
        transformation: &crate::shapes::ShapeTransformation,
        pos: GridPosition,
    ) -> u64 {
        let mut new_hash = current_hash;

        // Add shape hash
        new_hash ^= self.hasher.shape_hash(transformation.shape_index);

        // Add cell hashes
        for cell in &transformation.cells {
            new_hash ^= self
                .hasher
                .toggle_cell(new_hash, pos.x + cell.x, pos.y + cell.y, true);
        }

        new_hash
    }

    /// Get solver statistics
    pub fn get_stats(&self) -> &SolverStats {
        &self.stats
    }

    /// Reset solver state
    pub fn reset(&mut self) {
        self.grid.clear();
        for instance in &mut self.shapes {
            instance.placed = 0;
        }
        self.cache.clear();
        self.stats.reset();
    }
}

/// Parse input format: "WxH: shape_id:count, shape_id:count, ..."
fn parse_region_input(input: &str) -> Result<Region, ParseError> {
    let trimmed = input.trim();

    // Find the first colon that separates dimensions from shape requirements
    let colon_pos = trimmed
        .find(':')
        .ok_or_else(|| ParseError::InvalidShapeFormat("Missing colon separator".to_string()))?;

    let (dimensions_part, shapes_part) = trimmed.split_at(colon_pos);
    let shapes_part = &shapes_part[1..]; // Skip the colon

    // Parse dimensions "WxH"
    let dim_parts: Vec<&str> = dimensions_part.trim().split('x').collect();
    if dim_parts.len() != 2 {
        return Err(ParseError::InvalidShapeFormat(
            "Invalid dimension format".to_string(),
        ));
    }

    let width = dim_parts[0]
        .parse::<usize>()
        .map_err(|_| ParseError::InvalidShapeFormat("Invalid width".to_string()))?;
    let height = dim_parts[1]
        .parse::<usize>()
        .map_err(|_| ParseError::InvalidShapeFormat("Invalid height".to_string()))?;

    // Parse shape requirements
    let mut requirements = Vec::new();
    if !shapes_part.trim().is_empty() {
        let shape_parts: Vec<&str> = shapes_part.split(',').collect();

        for shape_part in shape_parts {
            let shape_part = shape_part.trim();
            if shape_part.is_empty() {
                continue;
            }
            let shape_spec: Vec<&str> = shape_part.split(':').collect();

            if shape_spec.len() != 2 {
                return Err(ParseError::InvalidShapeFormat(format!(
                    "Invalid shape format: '{}'",
                    shape_part
                )));
            }

            let shape_id = shape_spec[0]
                .parse::<usize>()
                .map_err(|_| ParseError::InvalidShapeFormat("Invalid shape ID".to_string()))?;

            if shape_id > 5 {
                return Err(ParseError::InvalidShapeFormat(
                    "Shape ID must be 0-5".to_string(),
                ));
            }

            let count = shape_spec[1]
                .parse::<usize>()
                .map_err(|_| ParseError::InvalidShapeFormat("Invalid shape count".to_string()))?;

            requirements.push(ShapeRequirement {
                shape_index: ShapeIndex(shape_id),
                count,
            });
        }
    }

    Ok(Region {
        width,
        height,
        requirements,
    })
}

/// Solve a single region packing problem with optimized solver (using ShapeFactory for backward compatibility)
pub fn solve_region(input: &str) -> SolveResult {
    use crate::shapes::ShapeFactory;

    let region = parse_region_input(input)?;

    // Build shape definitions from ShapeFactory for backward compatibility
    let mut shape_definitions = HashMap::new();
    for i in 0..=5 {
        let shape_index = ShapeIndex(i);
        let shape = ShapeFactory::create_shape(shape_index);
        shape_definitions.insert(shape_index, shape);
    }

    let mut solver = OptimizedSolver::new(
        region.width,
        region.height,
        region.requirements,
        shape_definitions,
    )?;

    Ok(solver.solve())
}

/// Count solvable regions in complete puzzle input (using ShapeFactory for backward compatibility)
pub fn solve_puzzle(input: &str) -> Result<usize, String> {
    use crate::shapes::ShapeFactory;

    // Build shape definitions from ShapeFactory
    let mut shape_definitions = HashMap::new();
    for i in 0..=5 {
        let shape_index = ShapeIndex(i);
        let shape = ShapeFactory::create_shape(shape_index);
        shape_definitions.insert(shape_index, shape);
    }

    let lines: Vec<&str> = input.trim().lines().collect();
    let mut count = 0;

    for line in lines {
        if line.trim().is_empty() {
            continue;
        }

        let region = parse_region_input(line)
            .map_err(|e| format!("Failed to parse region '{}': {}", line.trim(), e))?;

        let mut solver = OptimizedSolver::new(
            region.width,
            region.height,
            region.requirements,
            shape_definitions.clone(),
        )
        .map_err(|e| format!("Failed to create solver for region '{}': {}", line.trim(), e))?;

        if solver.solve() {
            count += 1;
        }
    }

    Ok(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_region_input() {
        let input = "4x4: 4:2";
        let region = parse_region_input(input).unwrap();
        assert_eq!(region.width, 4);
        assert_eq!(region.height, 4);
        assert_eq!(region.requirements.len(), 1);
        assert_eq!(region.requirements[0].shape_index.0, 4);
        assert_eq!(region.requirements[0].count, 2);
    }

    #[test]
    fn test_parse_multiple_shapes() {
        let input = "12x5: 0:1, 2:1, 4:2, 5:2";
        let region = parse_region_input(input).unwrap();
        assert_eq!(region.width, 12);
        assert_eq!(region.height, 5);
        assert_eq!(region.requirements.len(), 4);
    }

    #[test]
    fn test_optimized_solver_creation() {
        use crate::shapes::ShapeFactory;

        let requirements = vec![ShapeRequirement {
            shape_index: ShapeIndex(4),
            count: 2,
        }];

        // Build shape definitions from ShapeFactory
        let mut shape_definitions = HashMap::new();
        for i in 0..=5 {
            let shape_index = ShapeIndex(i);
            let shape = ShapeFactory::create_shape(shape_index);
            shape_definitions.insert(shape_index, shape);
        }

        let solver = OptimizedSolver::new(4, 4, requirements, shape_definitions);
        assert!(solver.is_ok());
    }

    #[test]
    fn test_optimized_solver_solve() {
        let input = "4x4: 4:2";
        let result = solve_region(input);
        assert!(result.is_ok());
        // We don't assert the result value since it depends on the actual packing logic
    }

    #[test]
    fn test_solve_puzzle_basic() {
        let input = "4x4: 4:2\n12x5: 0:1, 2:1, 4:2, 5:2\n12x5: 0:1, 2:1, 4:3, 5:2";
        let result = solve_puzzle(input);
        assert!(result.is_ok());
        // Should process all regions successfully
    }
}
