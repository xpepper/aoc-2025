// ABOUTME: High-performance memoization and Zobrist hashing for solver optimization
// ABOUTME: Implements fast state hashing and subproblem result caching

use crate::grid::BitPackedGrid;
use crate::{GridPosition, ShapeIndex};
use std::collections::HashMap;

/// Memoization cache for storing solved subproblems
#[derive(Debug, Clone)]
pub struct MemoizationCache {
    cache: HashMap<u64, bool>, // Grid hash -> solvable result
    max_size: usize,           // Memory limit
    hits: u64,                 // Performance counters
    misses: u64,
}

impl MemoizationCache {
    /// Create a new cache with specified size limit
    #[must_use]
    pub fn new(max_size: usize) -> Self {
        Self {
            cache: HashMap::with_capacity(max_size),
            max_size,
            hits: 0,
            misses: 0,
        }
    }

    /// Get cached result for a grid state
    #[must_use]
    pub fn get(&self, hash: u64) -> Option<bool> {
        self.cache.get(&hash).copied()
    }

    /// Store result for a grid state
    pub fn insert(&mut self, hash: u64, result: bool) {
        // Simple eviction policy: clear if at capacity
        if self.cache.len() >= self.max_size {
            self.cache.clear();
        }

        self.cache.insert(hash, result);
    }

    /// Check if hash exists in cache
    #[must_use]
    pub fn contains(&self, hash: u64) -> bool {
        self.cache.contains_key(&hash)
    }

    /// Get cache hit rate (0.0 to 1.0)
    #[must_use]
    pub fn hit_rate(&self) -> f64 {
        let total = self.hits + self.misses;
        if total == 0 {
            0.0
        } else {
            self.hits as f64 / total as f64
        }
    }

    /// Get performance statistics
    #[must_use]
    pub fn stats(&self) -> (u64, u64, f64) {
        (self.hits, self.misses, self.hit_rate())
    }

    /// Record a cache hit
    pub fn record_hit(&mut self) {
        self.hits += 1;
    }

    /// Record a cache miss
    pub fn record_miss(&mut self) {
        self.misses += 1;
    }

    /// Clear the cache and reset statistics
    pub fn clear(&mut self) {
        self.cache.clear();
        self.hits = 0;
        self.misses = 0;
    }

    /// Get current cache size
    #[must_use]
    pub fn size(&self) -> usize {
        self.cache.len()
    }
}

/// Zobrist hasher for fast incremental grid state hashing
#[derive(Debug, Clone)]
pub struct ZobristHasher {
    table: Vec<u64>,        // Random hash table for cells
    shape_hashes: Vec<u64>, // Hash values for each shape type
    width: usize,
    height: usize,
}

impl ZobristHasher {
    /// Create new hasher for grid dimensions
    #[must_use]
    pub fn new(width: usize, height: usize) -> Self {
        let mut rng_state = 123456789u64; // Simple PRNG seed
        let mut table = Vec::with_capacity(width * height);

        // Generate random 64-bit values for each cell position
        for _ in 0..(width * height) {
            rng_state = rng_state.wrapping_mul(1103515245).wrapping_add(12345);
            table.push(rng_state);
        }

        // Generate hashes for shape types (0-5)
        let mut shape_hashes = Vec::with_capacity(6);
        for _ in 0..6 {
            rng_state = rng_state.wrapping_mul(1103515245).wrapping_add(12345);
            shape_hashes.push(rng_state);
        }

        Self {
            table,
            shape_hashes,
            width,
            height,
        }
    }

    /// Compute hash for current grid state
    #[must_use]
    pub fn compute_hash(&self, grid: &BitPackedGrid) -> u64 {
        let mut hash = 0u64;

        for y in 0..grid.height {
            for x in 0..grid.width {
                let pos = GridPosition::new(x, y);
                if grid.is_occupied(pos) {
                    let index = y * self.width + x;
                    hash ^= self.table[index];
                }
            }
        }

        hash
    }

    /// Compute hash for grid with additional shape context
    #[must_use]
    pub fn compute_hash_with_shapes(
        &self,
        grid: &BitPackedGrid,
        placed_shapes: &[ShapeIndex],
    ) -> u64 {
        let mut hash = self.compute_hash(grid);

        // Include shape types in hash for better cache discrimination
        for shape_index in placed_shapes {
            hash ^= self.shape_hashes[shape_index.0.min(5)]; // Safety: max 5
        }

        hash
    }

    /// Incrementally update hash when a cell is toggled
    #[must_use]
    pub fn toggle_cell(&self, current_hash: u64, x: usize, y: usize, is_occupied: bool) -> u64 {
        if x >= self.width || y >= self.height {
            return current_hash;
        }

        let index = y * self.width + x;
        if is_occupied {
            current_hash ^ self.table[index]
        } else {
            current_hash ^ self.table[index] // XOR is its own inverse
        }
    }

    /// Get hash value for a specific shape type
    #[must_use]
    pub fn shape_hash(&self, shape_index: ShapeIndex) -> u64 {
        self.shape_hashes[shape_index.0.min(5)] // Safety: max 5
    }
}

impl Default for MemoizationCache {
    fn default() -> Self {
        Self::new(10000) // Default cache size
    }
}

/// Performance statistics for solver optimization
#[derive(Debug, Clone, Default)]
pub struct SolverStats {
    pub nodes_explored: u64,
    pub cache_hits: u64,
    pub cache_misses: u64,
    pub pruned_branches: u64,
    pub solutions_found: u64,
}

impl SolverStats {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn record_node(&mut self) {
        self.nodes_explored += 1;
    }

    pub fn record_cache_hit(&mut self) {
        self.cache_hits += 1;
    }

    pub fn record_cache_miss(&mut self) {
        self.cache_misses += 1;
    }

    pub fn record_pruned_branch(&mut self) {
        self.pruned_branches += 1;
    }

    pub fn record_solution(&mut self) {
        self.solutions_found += 1;
    }

    #[must_use]
    pub fn cache_hit_rate(&self) -> f64 {
        let total = self.cache_hits + self.cache_misses;
        if total == 0 {
            0.0
        } else {
            self.cache_hits as f64 / total as f64
        }
    }

    #[must_use]
    pub fn prune_rate(&self) -> f64 {
        if self.nodes_explored == 0 {
            0.0
        } else {
            self.pruned_branches as f64 / self.nodes_explored as f64
        }
    }

    pub fn reset(&mut self) {
        *self = Self::default();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memoization_cache_basic() {
        let mut cache = MemoizationCache::new(10);

        assert!(!cache.contains(123));
        assert_eq!(cache.get(456), None);

        cache.insert(123, true);
        assert!(cache.contains(123));
        assert_eq!(cache.get(123), Some(true));
        assert_eq!(cache.get(456), None);
    }

    #[test]
    fn test_memoization_cache_eviction() {
        let mut cache = MemoizationCache::new(2);

        cache.insert(1, true);
        cache.insert(2, false);
        assert_eq!(cache.size(), 2);

        // Insert third item should trigger eviction
        cache.insert(3, true);
        assert_eq!(cache.size(), 1); // Should be cleared due to eviction policy
        assert!(cache.contains(3));
    }

    #[test]
    fn test_cache_statistics() {
        let mut cache = MemoizationCache::new(10);

        cache.record_miss();
        cache.record_hit();
        cache.record_hit();

        let (hits, misses, hit_rate) = cache.stats();
        assert_eq!(hits, 2);
        assert_eq!(misses, 1);
        assert!((hit_rate - 0.666).abs() < 0.001);
    }

    #[test]
    fn test_zobrist_hasher() {
        let hasher = ZobristHasher::new(4, 4);

        let mut grid = BitPackedGrid::new(4, 4).unwrap();
        let hash1 = hasher.compute_hash(&grid);

        // Place a cell
        grid.set_occupied(GridPosition::new(1, 1), true);
        let hash2 = hasher.compute_hash(&grid);

        assert_ne!(hash1, hash2);

        // Remove the cell
        grid.set_occupied(GridPosition::new(1, 1), false);
        let hash3 = hasher.compute_hash(&grid);

        assert_eq!(hash1, hash3); // Should be back to original
    }

    #[test]
    fn test_zobrist_incremental_update() {
        let hasher = ZobristHasher::new(3, 3);
        let mut grid = BitPackedGrid::new(3, 3).unwrap();

        let initial_hash = hasher.compute_hash(&grid);

        // Update incrementally
        let updated_hash = hasher.toggle_cell(initial_hash, 1, 1, true);

        // Set cell directly and recompute
        grid.set_occupied(GridPosition::new(1, 1), true);
        let recomputed_hash = hasher.compute_hash(&grid);

        assert_eq!(updated_hash, recomputed_hash);
    }

    #[test]
    fn test_solver_stats() {
        let mut stats = SolverStats::new();

        stats.record_node();
        stats.record_node();
        stats.record_cache_hit();
        stats.record_cache_miss();
        stats.record_cache_miss();
        stats.record_pruned_branch();
        stats.record_solution();

        assert_eq!(stats.nodes_explored, 2);
        assert_eq!(stats.cache_hits, 1);
        assert_eq!(stats.cache_misses, 2);
        assert_eq!(stats.solutions_found, 1);
        assert_eq!(stats.pruned_branches, 1);

        let hit_rate = stats.cache_hit_rate();
        assert!((hit_rate - 0.333).abs() < 0.001);
    }
}
