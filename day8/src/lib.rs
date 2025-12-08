pub fn parse_coordinate(line: &str) -> (i32, i32, i32) {
    let parts: Vec<i32> = line.split(',').map(|s| s.parse().unwrap()).collect();
    let [x, y, z]: [i32; 3] = parts.try_into().unwrap();
    (x, y, z)
}

pub fn distance(coord1: (i32, i32, i32), coord2: (i32, i32, i32)) -> f64 {
    let dx = (coord2.0 - coord1.0) as f64;
    let dy = (coord2.1 - coord1.1) as f64;
    let dz = (coord2.2 - coord1.2) as f64;
    (dx * dx + dy * dy + dz * dz).sqrt()
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
}
