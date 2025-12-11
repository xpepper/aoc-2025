use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Tile {
    pub x: i64,
    pub y: i64,
}

impl Tile {
    fn area_with(self, other: Tile) -> u64 {
        let width = self.x.saturating_sub(other.x).unsigned_abs() + 1;
        let height = self.y.saturating_sub(other.y).unsigned_abs() + 1;
        width * height
    }
}

impl FromStr for Tile {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s
            .split_once(',')
            .ok_or_else(|| "Line must contain a comma separating x and y".to_string())?;

        let x = x
            .trim()
            .parse()
            .map_err(|e| format!("Invalid x coordinate: {}", e))?;
        let y = y
            .trim()
            .parse()
            .map_err(|e| format!("Invalid y coordinate: {}", e))?;

        Ok(Tile { x, y })
    }
}

pub fn largest_rectangle_area(input: &str) -> u64 {
    let tiles = parse_tiles(input);
    max_rectangle_area(&tiles)
}

fn parse_tiles(input: &str) -> Vec<Tile> {
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.parse::<Tile>().expect("Invalid coordinate line"))
        .collect()
}

fn max_rectangle_area(tiles: &[Tile]) -> u64 {
    let mut best = 0;
    for (i, &a) in tiles.iter().enumerate() {
        for &b in tiles.iter().skip(i + 1) {
            best = best.max(a.area_with(b));
        }
    }
    best
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "\
7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
";

    #[test]
    fn computes_max_rectangle_area_for_sample() {
        let area = largest_rectangle_area(SAMPLE);
        assert_eq!(area, 50);
    }
}
