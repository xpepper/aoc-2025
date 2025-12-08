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
}
