pub fn parse_coordinate(line: &str) -> (i32, i32, i32) {
    let parts: Vec<i32> = line.split(',').map(|s| s.parse().unwrap()).collect();
    let [x, y, z]: [i32; 3] = parts.try_into().unwrap();
    (x, y, z)
}

pub fn distance(coord1: (i32, i32, i32), coord2: (i32, i32, i32)) -> f64 {
    let (x1, y1, z1) = coord1;
    let (x2, y2, z2) = coord2;
    let dx = (x2 - x1) as f64;
    let dy = (y2 - y1) as f64;
    let dz = (z2 - z1) as f64;
    (dx * dx + dy * dy + dz * dz).sqrt()
}

pub struct UnionFind {
    #[allow(dead_code)]
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
            // Union by size: attach smaller tree to larger tree
            if self.size[root_x] < self.size[root_y] {
                self.parent[root_x] = root_y;
                self.size[root_y] += self.size[root_x];
            } else {
                self.parent[root_y] = root_x;
                self.size[root_x] += self.size[root_y];
            }
        }
    }

    pub fn circuit_size(&self, x: usize) -> usize {
        self.size[x]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_coordinate() {
        let coord = parse_coordinate("162,817,812");
        assert_eq!(coord, (162, 817, 812));
    }

    #[test]
    fn test_distance() {
        let coord1 = (162, 817, 812);
        let coord2 = (425, 690, 689);
        let dist = distance(coord1, coord2);
        // √((425-162)² + (690-817)² + (689-812)²)
        // √(263² + (-127)² + (-123)²)
        // √(69169 + 16129 + 15129)
        // √100427 ≈ 316.898
        assert!((dist - 316.898).abs() < 0.01);
    }

    #[test]
    fn test_union_find_initialization() {
        let uf = UnionFind::new(5);
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
}
