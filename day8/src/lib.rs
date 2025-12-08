pub fn parse_coordinate(line: &str) -> (i32, i32, i32) {
    let parts: Vec<i32> = line.split(',').map(|s| s.parse().unwrap()).collect();
    (parts[0], parts[1], parts[2])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_coordinate() {
        let coord = parse_coordinate("162,817,812");
        assert_eq!(coord, (162, 817, 812));
    }
}
