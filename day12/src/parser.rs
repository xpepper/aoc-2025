// ABOUTME: Error handling types for present packing parser

/// Error types for parsing operations
#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    /// Invalid shape format in input
    InvalidShapeFormat(String),
    /// Invalid region dimensions (width x height too large)
    InvalidDimensions(String),
    /// Invalid shape count (negative or missing)
    InvalidCounts(String),
}

/// Error types for grid operations
#[derive(Debug, PartialEq, Eq)]
pub enum GridError {
    /// Grid dimensions too large for 64-bit representation
    TooLarge(usize, usize), // width, height
    /// Invalid grid dimensions
    InvalidDimensions(usize, usize), // width, height
}

/// Error types for shape placement operations
#[derive(Debug, PartialEq, Eq)]
pub enum PlacementError {
    /// Position is outside grid bounds
    OutOfBounds,
    /// Shape would overlap existing cells
    Overlap,
    /// Invalid shape index
    InvalidShape(usize),
}

/// Error types for region operations
#[derive(Debug, PartialEq, Eq)]
pub enum RegionError {
    /// Invalid region dimensions
    InvalidDimensions(usize, usize),
    /// Invalid quantity for required shape
    InvalidShapeQuantity(usize, usize), // shape_id, quantity
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::InvalidShapeFormat(msg) => write!(f, "Invalid shape format: {msg}"),
            ParseError::InvalidDimensions(msg) => write!(f, "Invalid dimensions: {msg}"),
            ParseError::InvalidCounts(msg) => write!(f, "Invalid counts: {msg}"),
        }
    }
}

impl std::fmt::Display for GridError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GridError::TooLarge(width, height) => write!(f, "Grid too large: {width}x{height}"),
            GridError::InvalidDimensions(width, height) => {
                write!(f, "Invalid dimensions: {width}x{height}")
            }
        }
    }
}

impl std::fmt::Display for PlacementError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PlacementError::OutOfBounds => write!(f, "Position out of bounds"),
            PlacementError::Overlap => write!(f, "Shape would overlap existing cells"),
            PlacementError::InvalidShape(id) => write!(f, "Invalid shape index: {id}"),
        }
    }
}

impl std::fmt::Display for RegionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RegionError::InvalidDimensions(width, height) => {
                write!(f, "Invalid region dimensions: {width}x{height}")
            }
            RegionError::InvalidShapeQuantity(id, qty) => {
                write!(f, "Invalid quantity for shape {id}: {qty}")
            }
        }
    }
}

impl std::error::Error for ParseError {}
impl std::error::Error for GridError {}
impl std::error::Error for PlacementError {}
impl std::error::Error for RegionError {}

/// Result type for parsing operations
pub type ParseResult<T> = Result<T, ParseError>;
/// Result type for grid operations
pub type GridResult<T> = Result<T, GridError>;
/// Result type for placement operations
pub type PlacementResult<T> = Result<T, PlacementError>;
/// Result type for region operations
pub type RegionResult<T> = Result<T, RegionError>;
