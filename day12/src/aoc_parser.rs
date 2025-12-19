// ABOUTME: Parser for Advent of Code Day 12 format
// ABOUTME: Handles shape definitions and region specifications from puzzle-input.txt

use crate::parser::ParseError;
use crate::shapes::Shape;
use crate::solver::ShapeRequirement;
use crate::{Cell, ShapeIndex};
use std::collections::HashMap;

/// Represents a loaded shape from AoC format
#[derive(Debug, Clone)]
pub struct AocShape {
    pub index: ShapeIndex,
    pub cells: Vec<Cell>,
    pub width: usize,
    pub height: usize,
}

/// Represents a region specification from AoC format
#[derive(Debug, Clone)]
pub struct AocRegion {
    pub width: usize,
    pub height: usize,
    pub shape_requirements: Vec<ShapeRequirement>,
}

/// Parser for AoC Day 12 format
pub struct AocParser {
    shapes: Vec<AocShape>,
}

impl AocParser {
    /// Create new parser
    pub fn new() -> Self {
        Self { shapes: Vec::new() }
    }

    /// Parse the complete AoC format input
    pub fn parse(&mut self, input: &str) -> Result<Vec<AocRegion>, ParseError> {
        let lines: Vec<&str> = input.lines().collect();

        // Parse shapes until we have all 6 shapes (0-5)
        let mut i = 0;
        while i < lines.len() && self.shapes.len() < 6 {
            let line = lines[i].trim();
            if line.is_empty() {
                i += 1;
                continue;
            }

            // Check if this is a shape definition (ends with ':')
            if line.ends_with(':') {
                let index_str = &line[..line.len() - 1];
                if let Ok(index) = index_str.parse::<usize>() {
                    if index <= 5 {
                        // Parse this shape
                        i += 1;
                        let mut shape_lines = Vec::new();
                        while i < lines.len() && !lines[i].trim().is_empty() {
                            shape_lines.push(lines[i].trim());
                            i += 1;
                        }

                        if !shape_lines.is_empty() {
                            let shape = self.parse_shape_grid(index, &shape_lines)?;
                            self.shapes.push(shape);
                        }
                        // Skip the empty line after shape
                        i += 1;
                        continue;
                    }
                }
            }

            // If we get here and don't have 6 shapes yet, it's an error
            if self.shapes.len() < 6 {
                return Err(ParseError::InvalidShapeFormat(format!(
                    "Expected shape definition, found: '{}'",
                    line
                )));
            }
        }

        // Skip any remaining empty lines after shapes
        while i < lines.len() && lines[i].trim().is_empty() {
            i += 1;
        }

        // Parse regions from remaining lines
        let region_lines = &lines[i..];
        self.parse_regions(region_lines)
    }

    /// Parse a single shape's grid definition
    fn parse_shape_grid(&self, index: usize, lines: &[&str]) -> Result<AocShape, ParseError> {
        let height = lines.len();
        if height == 0 {
            return Err(ParseError::InvalidShapeFormat(
                "Empty shape definition".to_string(),
            ));
        }

        let width = lines[0].len();
        if width == 0 {
            return Err(ParseError::InvalidShapeFormat(
                "Empty shape line".to_string(),
            ));
        }

        // Validate all lines have same width
        for line in lines {
            if line.len() != width {
                return Err(ParseError::InvalidShapeFormat(format!(
                    "Inconsistent shape width: expected {}, got {}",
                    width,
                    line.len()
                )));
            }
        }

        // Parse cells
        let mut cells = Vec::new();
        for (y, line) in lines.iter().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                if ch == '#' {
                    cells.push(Cell::new(x, y));
                }
            }
        }

        // Normalize cells to start at (0,0)
        let normalized_cells = self.normalize_cells(cells);

        Ok(AocShape {
            index: ShapeIndex(index),
            cells: normalized_cells.clone(),
            width,
            height,
        })
    }

    /// Normalize cells to have (0,0) as top-left corner
    fn normalize_cells(&self, cells: Vec<Cell>) -> Vec<Cell> {
        if cells.is_empty() {
            return cells;
        }

        let min_x = cells.iter().map(|c| c.x).min().unwrap();
        let min_y = cells.iter().map(|c| c.y).min().unwrap();

        cells
            .iter()
            .map(|c| Cell::new(c.x - min_x, c.y - min_y))
            .collect()
    }

    /// Parse region definitions
    fn parse_regions(&self, lines: &[&str]) -> Result<Vec<AocRegion>, ParseError> {
        let mut regions = Vec::new();

        for line in lines {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            println!("DEBUG: Processing region line: '{}'", line);

            let parts: Vec<&str> = line.split(':').collect();
            if parts.len() != 2 {
                return Err(ParseError::InvalidShapeFormat(format!(
                    "Invalid region format: '{}'",
                    line
                )));
            }

            println!("DEBUG: Parts: {:?}, dim part: '{}'", parts, parts[0]);

            // Parse dimensions
            let dim_parts: Vec<&str> = parts[0].trim().split('x').collect();
            if dim_parts.len() != 2 {
                return Err(ParseError::InvalidShapeFormat(format!(
                    "Invalid dimension format: '{}'",
                    parts[0]
                )));
            }

            println!("DEBUG: Dim parts: {:?}", dim_parts);

            let width = dim_parts[0].parse::<usize>().map_err(|e| {
                ParseError::InvalidShapeFormat(format!("Invalid width '{}': {}", dim_parts[0], e))
            })?;
            let height = dim_parts[1].parse::<usize>().map_err(|e| {
                ParseError::InvalidShapeFormat(format!("Invalid height '{}': {}", dim_parts[1], e))
            })?;

            println!("DEBUG: Parsed dimensions: {}x{}", width, height);

            // Parse shape counts
            let count_parts: Vec<&str> = parts[1].trim().split_whitespace().collect();
            if count_parts.len() != 6 {
                return Err(ParseError::InvalidShapeFormat(format!(
                    "Expected 6 shape counts, got {}: '{}'",
                    count_parts.len(),
                    parts[1]
                )));
            }

            let mut requirements = Vec::new();
            for (shape_idx, count_str) in count_parts.iter().enumerate() {
                let count = count_str.parse::<usize>().map_err(|_| {
                    ParseError::InvalidShapeFormat("Invalid shape count".to_string())
                })?;

                if count > 0 {
                    requirements.push(ShapeRequirement {
                        shape_index: ShapeIndex(shape_idx),
                        count,
                    });
                }
            }

            regions.push(AocRegion {
                width,
                height,
                shape_requirements: requirements,
            });
        }

        Ok(regions)
    }

    /// Get shape by index
    pub fn get_shape(&self, index: ShapeIndex) -> Option<&AocShape> {
        self.shapes.iter().find(|s| s.index == index)
    }

    /// Get all shapes
    pub fn get_shapes(&self) -> &[AocShape] {
        &self.shapes
    }

    /// Convert parsed AocShapes to Shape format for solver
    pub fn get_shape_definitions(&self) -> HashMap<ShapeIndex, Shape> {
        self.shapes
            .iter()
            .map(|aoc_shape| {
                let shape = Shape::new(aoc_shape.index, aoc_shape.cells.clone());
                (aoc_shape.index, shape)
            })
            .collect()
    }
}

/// Solve the complete AoC puzzle
pub fn solve_aoc_puzzle(input: &str) -> Result<usize, ParseError> {
    let mut parser = AocParser::new();
    let regions = parser.parse(input)?;

    // Get shape definitions from parsed shapes
    let shape_definitions = parser.get_shape_definitions();

    let mut solvable_count = 0;

    for (i, region) in regions.iter().enumerate() {
        // Use our optimized solver with dynamic shapes
        match solve_region_with_shapes(region, &shape_definitions) {
            Ok(true) => {
                solvable_count += 1;
                println!(
                    "Region {}: {}x{} - SOLVABLE",
                    i + 1,
                    region.width,
                    region.height
                );
            }
            Ok(false) => {
                println!(
                    "Region {}: {}x{} - NOT SOLVABLE",
                    i + 1,
                    region.width,
                    region.height
                );
            }
            Err(e) => {
                println!(
                    "Region {}: {}x{} - ERROR: {:?}",
                    i + 1,
                    region.width,
                    region.height,
                    e
                );
                return Err(e);
            }
        }
    }

    Ok(solvable_count)
}

/// Convert AocRegion to our solver's input format
pub fn format_region_for_solver(region: &AocRegion) -> String {
    let mut result = format!("{}x{}:", region.width, region.height);

    let mut parts = Vec::new();
    for requirement in &region.shape_requirements {
        parts.push(format!(
            "{}:{}",
            requirement.shape_index.0, requirement.count
        ));
    }

    if !parts.is_empty() {
        result.push(' ');
        result.push_str(&parts.join(", "));
    }

    result
}

/// Solve a single region using our optimized solver with dynamic shapes
pub fn solve_region_with_shapes(
    region: &AocRegion,
    shape_definitions: &HashMap<ShapeIndex, Shape>,
) -> Result<bool, crate::parser::ParseError> {
    use crate::solver::OptimizedSolver;

    let mut solver = OptimizedSolver::new(
        region.width,
        region.height,
        region.shape_requirements.clone(),
        shape_definitions.clone(),
    )?;

    Ok(solver.solve())
}

/// Solve a single region using our optimized solver
pub fn solve_region_optimized(input: &str) -> Result<bool, crate::parser::ParseError> {
    crate::solver::solve_region(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_region_for_solver() {
        let region = AocRegion {
            width: 4,
            height: 4,
            shape_requirements: vec![ShapeRequirement {
                shape_index: ShapeIndex(4),
                count: 2,
            }],
        };

        let result = format_region_for_solver(&region);
        assert_eq!(result, "4x4: 4:2");
    }
}
