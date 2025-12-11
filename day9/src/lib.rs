use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

pub fn solve_part_one(input: &str) -> u64 {
    largest_rectangle_area(input)
}

pub fn solve_part_two(input: &str) -> u64 {
    let tiles = parse_tiles(input);
    let xs = compress_coords(tiles.iter().map(|t| t.x));
    let ys = compress_coords(tiles.iter().map(|t| t.y));
    let x_index = index_map(&xs);
    let y_index = index_map(&ys);

    let vertical_edges = collect_vertical_edges(&tiles);
    let boundary = collect_boundary_tiles(&tiles);

    let mut inside_grid = build_inside_grid(&xs, &ys, &vertical_edges);
    mark_boundary_tiles(&mut inside_grid, &boundary, &x_index, &y_index);
    let area_prefix = build_area_prefix(&inside_grid, &xs, &ys);

    let mut best = 0;
    for (i, &a) in tiles.iter().enumerate() {
        for &b in tiles.iter().skip(i + 1) {
            if a.x == b.x || a.y == b.y {
                continue;
            }
            let rect_area = a.area_with(b);
            let sum_inside = query_area_sum(
                &area_prefix,
                x_index[&a.x].min(x_index[&b.x]),
                x_index[&a.x].max(x_index[&b.x]) + 1, // inclusive of tiles, +1 because xs are edges
                y_index[&a.y].min(y_index[&b.y]),
                y_index[&a.y].max(y_index[&b.y]) + 1,
            );
            if sum_inside == rect_area {
                best = best.max(rect_area);
            }
        }
    }

    best
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

fn compress_coords(coords: impl Iterator<Item = i64>) -> Vec<i64> {
    let collected: Vec<i64> = coords.collect();
    let min_v = *collected.iter().min().unwrap();
    let max_v = *collected.iter().max().unwrap();

    let mut set = collected
        .iter()
        .flat_map(|&v| [v - 1, v, v + 1])
        .collect::<Vec<_>>();
    set.push(min_v - 2);
    set.push(max_v + 2);
    set.sort_unstable();
    set.dedup();
    set
}

fn index_map(xs: &[i64]) -> std::collections::HashMap<i64, usize> {
    xs.iter().enumerate().map(|(i, &v)| (v, i)).collect()
}

#[derive(Clone, Copy)]
struct VerticalEdge {
    x: i64,
    y_min: i64,
    y_max: i64,
}

fn collect_vertical_edges(tiles: &[Tile]) -> Vec<VerticalEdge> {
    let mut edges = Vec::new();
    for i in 0..tiles.len() {
        let a = tiles[i];
        let b = tiles[(i + 1) % tiles.len()];
        if a.x == b.x {
            let (y_min, y_max) = if a.y <= b.y { (a.y, b.y) } else { (b.y, a.y) };
            edges.push(VerticalEdge {
                x: a.x,
                y_min,
                y_max,
            });
        }
    }
    edges
}

fn collect_boundary_tiles(tiles: &[Tile]) -> Vec<Tile> {
    let mut set = std::collections::HashSet::new();
    for i in 0..tiles.len() {
        let a = tiles[i];
        let b = tiles[(i + 1) % tiles.len()];
        if a.x == b.x {
            let step = if a.y <= b.y { 1 } else { -1 };
            let mut y = a.y;
            loop {
                set.insert(Tile { x: a.x, y });
                if y == b.y {
                    break;
                }
                y += step;
            }
        } else {
            let step = if a.x <= b.x { 1 } else { -1 };
            let mut x = a.x;
            loop {
                set.insert(Tile { x, y: a.y });
                if x == b.x {
                    break;
                }
                x += step;
            }
        }
    }
    set.into_iter().collect()
}

fn mark_boundary_tiles(
    grid: &mut [Vec<bool>],
    boundary: &[Tile],
    x_index: &std::collections::HashMap<i64, usize>,
    y_index: &std::collections::HashMap<i64, usize>,
) {
    for tile in boundary {
        if let (Some(&xi), Some(&yi)) = (x_index.get(&tile.x), y_index.get(&tile.y)) {
            grid[yi][xi] = true;
        }
    }
}

fn build_inside_grid(xs: &[i64], ys: &[i64], vertical_edges: &[VerticalEdge]) -> Vec<Vec<bool>> {
    let width = xs.len() - 1;
    let height = ys.len() - 1;
    let mut inside = vec![vec![false; width]; height];

    for row in 0..height {
        let y_mid_twice = ys[row] + ys[row + 1]; // midpoint * 2
        let mut intersections = Vec::new();
        for edge in vertical_edges {
            let y_low = edge.y_min;
            let y_high = edge.y_max;
            // Half-open to avoid double counting vertices
            if y_low * 2 <= y_mid_twice && y_mid_twice < y_high * 2 {
                intersections.push(edge.x);
            }
        }
        intersections.sort_unstable();
        for pair in intersections.chunks_exact(2) {
            let left = pair[0];
            let right = pair[1];
            let mut col = match xs.binary_search(&left) {
                Ok(i) => i,
                Err(i) => i.saturating_sub(1),
            };
            while col < width && xs[col] < right {
                inside[row][col] = true;
                col += 1;
            }
        }
    }

    inside
}

fn build_area_prefix(inside: &[Vec<bool>], xs: &[i64], ys: &[i64]) -> Vec<Vec<u64>> {
    let h = inside.len();
    let w = inside[0].len();
    let mut pref = vec![vec![0u64; w + 1]; h + 1];
    for y in 0..h {
        let dy = (ys[y + 1] - ys[y]) as u64;
        for x in 0..w {
            let dx = (xs[x + 1] - xs[x]) as u64;
            let cell_area = if inside[y][x] { dx * dy } else { 0 };
            pref[y + 1][x + 1] = pref[y + 1][x] + pref[y][x + 1] - pref[y][x] + cell_area;
        }
    }
    pref
}

fn query_area_sum(prefix: &[Vec<u64>], x0: usize, x1: usize, y0: usize, y1: usize) -> u64 {
    let a = prefix[y1][x1] as i128;
    let b = prefix[y0][x1] as i128;
    let c = prefix[y1][x0] as i128;
    let d = prefix[y0][x0] as i128;
    let res = a - b - c + d;
    assert!(
        res >= 0,
        "negative area with x0={}, x1={}, y0={}, y1={}, values a={}, b={}, c={}, d={}",
        x0,
        x1,
        y0,
        y1,
        a,
        b,
        c,
        d
    );
    res as u64
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

    #[test]
    fn solve_part_one_returns_sample_answer() {
        let area = solve_part_one(SAMPLE);
        assert_eq!(area, 50);
    }

    #[test]
    fn solve_part_two_returns_sample_answer() {
        let area = solve_part_two(SAMPLE);
        assert_eq!(area, 24);
    }

    #[test]
    fn solve_part_one_returns_puzzle_answer() {
        let input = include_str!("../puzzle-input.txt");
        let area = solve_part_one(input);
        assert_eq!(area, 4_745_816_424);
    }

    #[test]
    fn solve_part_two_returns_puzzle_answer() {
        let input = include_str!("../puzzle-input.txt");
        let area = solve_part_two(input);
        assert_eq!(area, 1_351_617_690);
    }
}
