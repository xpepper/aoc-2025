use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Coordinate {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Coordinate {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Coordinate { x, y, z }
    }

    /// Calculate the distance from this coordinate to another
    pub fn distance_from(&self, other: Coordinate) -> f64 {
        let squared_distance = self.squared_distance_from(other);
        (squared_distance as f64).sqrt()
    }

    /// Calculate squared distance for performance comparisons
    pub fn squared_distance_from(&self, other: Coordinate) -> i64 {
        let dx = (other.x - self.x) as i64;
        let dy = (other.y - self.y) as i64;
        let dz = (other.z - self.z) as i64;
        dx * dx + dy * dy + dz * dz
    }
}

impl FromStr for Coordinate {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<i32> = s
            .split(',')
            .map(|p| p.parse().map_err(|e| format!("Parse error: {}", e)))
            .collect::<Result<Vec<_>, _>>()?;

        if parts.len() != 3 {
            return Err(format!("Expected 3 coordinates, got {}", parts.len()));
        }

        Ok(Coordinate::new(parts[0], parts[1], parts[2]))
    }
}

pub fn parse_coordinates(input: &str) -> Result<Vec<Coordinate>, String> {
    input
        .lines()
        .map(|line| {
            line.parse()
                .map_err(|e| format!("Failed to parse line '{}': {}", line, e))
        })
        .collect()
}

pub fn calculate_all_pair_distances(coordinates: &[Coordinate]) -> Vec<(usize, usize, f64)> {
    let mut pairs = Vec::new();

    for i in 0..coordinates.len() {
        for j in (i + 1)..coordinates.len() {
            let dist = coordinates[i].distance_from(coordinates[j]);
            pairs.push((i, j, dist));
        }
    }

    pairs
}

pub fn get_all_circuit_sizes(
    coordinates: &[Coordinate],
    connections: &[(usize, usize)],
) -> Vec<usize> {
    let mut uf = build_circuits(coordinates.len());
    apply_connections(&mut uf, connections);
    extract_and_sort_circuit_sizes(coordinates.len(), &mut uf)
}

fn build_circuits(num_coordinates: usize) -> UnionFind {
    UnionFind::new(num_coordinates)
}

fn apply_connections(uf: &mut UnionFind, connections: &[(usize, usize)]) {
    for &(i, j) in connections {
        uf.union(i, j);
    }
}

fn extract_and_sort_circuit_sizes(num_coordinates: usize, uf: &mut UnionFind) -> Vec<usize> {
    let mut circuit_sizes = collect_unique_circuit_sizes(num_coordinates, uf);
    sort_circuit_sizes_descending(&mut circuit_sizes);
    circuit_sizes
}

fn collect_unique_circuit_sizes(num_coordinates: usize, uf: &mut UnionFind) -> Vec<usize> {
    let mut unique_sizes = std::collections::HashSet::new();
    for i in 0..num_coordinates {
        let size = uf.circuit_size(i);
        unique_sizes.insert(size);
    }
    unique_sizes.into_iter().collect()
}

fn sort_circuit_sizes_descending(sizes: &mut [usize]) {
    sizes.sort_by(|a, b| b.cmp(a));
}

#[derive(Debug, Clone)]
pub struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        UnionFind {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }

    pub fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]); // Path compression
        }
        self.parent[x]
    }

    pub fn union(&mut self, x: usize, y: usize) {
        let root_x = self.find(x);
        let root_y = self.find(y);

        if root_x != root_y {
            self.merge_circuits(root_x, root_y);
        }
    }

    fn merge_circuits(&mut self, root_x: usize, root_y: usize) {
        // Union by size: attach smaller tree to larger tree
        if self.size[root_x] < self.size[root_y] {
            self.parent[root_x] = root_y;
            self.size[root_y] += self.size[root_x];
        } else {
            self.parent[root_y] = root_x;
            self.size[root_x] += self.size[root_y];
        }
    }

    pub fn circuit_size(&mut self, x: usize) -> usize {
        let root = self.find(x);
        self.size[root]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_coordinate() {
        let coord: Coordinate = "162,817,812".parse().unwrap();
        assert_eq!(coord, Coordinate::new(162, 817, 812));
    }

    #[test]
    fn test_distance() {
        let coord1 = Coordinate::new(162, 817, 812);
        let coord2 = Coordinate::new(425, 690, 689);
        let dist = coord1.distance_from(coord2);
        // √((425-162)² + (690-817)² + (689-812)²)
        // √(263² + (-127)² + (-123)²)
        // √(69169 + 16129 + 15129)
        // √100427 ≈ 316.902
        const TOLERANCE: f64 = 0.01; // Appropriate tolerance for floating point comparisons
        assert!((dist - 316.902).abs() < TOLERANCE);
    }

    #[test]
    fn test_coordinate_squared_distance_from() {
        let coord1 = Coordinate::new(0, 0, 0);
        let coord2 = Coordinate::new(3, 4, 12);
        let squared_dist = coord1.squared_distance_from(coord2);
        // 3² + 4² + 12² = 9 + 16 + 144 = 169
        assert_eq!(squared_dist, 169);

        // Distance should be sqrt(169) = 13.0
        let dist = coord1.distance_from(coord2);
        assert_eq!(dist, 13.0);
    }

    #[test]
    fn test_get_all_circuit_sizes() {
        let coords = vec![
            Coordinate::new(0, 0, 0),
            Coordinate::new(1, 0, 0),
            Coordinate::new(0, 1, 0),
            Coordinate::new(10, 10, 10),
            Coordinate::new(10, 11, 10),
        ];

        // Connect first three into one circuit (0-1, 1-2)
        // Connect last two into another circuit (3-4)
        let connections = vec![(0, 1), (1, 2), (3, 4)];
        let circuit_sizes = get_all_circuit_sizes(&coords, &connections);

        // Should have two circuits: one of size 3, one of size 2
        assert_eq!(circuit_sizes.len(), 2);
        assert!(circuit_sizes.contains(&3));
        assert!(circuit_sizes.contains(&2));
    }

    #[test]
    fn test_union_find_initialization() {
        let mut uf = UnionFind::new(5);
        // Each element should be in its own circuit initially
        assert_eq!(uf.circuit_size(0), 1);
        assert_eq!(uf.circuit_size(1), 1);
        assert_eq!(uf.circuit_size(4), 1);
    }

    #[test]
    fn test_union_find_find() {
        let mut uf = UnionFind::new(5);
        // Initially, each element is its own parent (root)
        assert_eq!(uf.find(0), 0);
        assert_eq!(uf.find(1), 1);
        assert_eq!(uf.find(4), 4);
    }

    #[test]
    fn test_union_find_union() {
        let mut uf = UnionFind::new(5);
        // Union 0 and 1
        uf.union(0, 1);
        // They should now have the same root
        assert_eq!(uf.find(0), uf.find(1));
        // Union 2 and 3
        uf.union(2, 3);
        assert_eq!(uf.find(2), uf.find(3));
        // 0 and 2 should still be in different circuits
        assert_ne!(uf.find(0), uf.find(2));
    }

    #[test]
    fn test_union_find_circuit_size() {
        let mut uf = UnionFind::new(5);
        // Union 0, 1, 2 into one circuit
        uf.union(0, 1);
        uf.union(1, 2);
        // All three should report circuit size of 3
        assert_eq!(uf.circuit_size(0), 3);
        assert_eq!(uf.circuit_size(1), 3);
        assert_eq!(uf.circuit_size(2), 3);
        // Element 3 should still be alone
        assert_eq!(uf.circuit_size(3), 1);
    }

    #[test]
    fn test_parse_coordinates() {
        let input = "162,817,812\n57,618,57\n906,360,560";
        let coordinates = parse_coordinates(input).unwrap();
        assert_eq!(coordinates.len(), 3);
        assert_eq!(coordinates[0], Coordinate::new(162, 817, 812));
        assert_eq!(coordinates[1], Coordinate::new(57, 618, 57));
        assert_eq!(coordinates[2], Coordinate::new(906, 360, 560));
    }

    #[test]
    fn test_parse_coordinates_error_handling() {
        let input = "162,817,812\ninvalid,coordinate\n906,360,560";
        let result = parse_coordinates(input);
        assert!(result.is_err());
        let error_msg = result.unwrap_err();
        assert!(error_msg.contains("invalid,coordinate"));
    }

    #[test]
    fn test_calculate_all_pair_distances() {
        let coords = vec![
            Coordinate::new(0, 0, 0),
            Coordinate::new(3, 4, 0),  // distance = 5.0
            Coordinate::new(0, 0, 12), // distance from first = 12.0, from second ≈ 13.0
        ];

        let pairs = calculate_all_pair_distances(&coords);

        // Should have 3 choose 2 = 3 pairs
        assert_eq!(pairs.len(), 3);

        // Check that all pairs are represented
        let pair_indices: Vec<_> = pairs.iter().map(|(i, j, _)| (*i, *j)).collect();
        assert!(pair_indices.contains(&(0, 1)));
        assert!(pair_indices.contains(&(0, 2)));
        assert!(pair_indices.contains(&(1, 2)));

        // Check specific distances
        let pair_01 = pairs.iter().find(|(i, j, _)| (*i, *j) == (0, 1)).unwrap();
        assert_eq!(pair_01.2, 5.0);

        let pair_02 = pairs.iter().find(|(i, j, _)| (*i, *j) == (0, 2)).unwrap();
        assert_eq!(pair_02.2, 12.0);
    }
}
