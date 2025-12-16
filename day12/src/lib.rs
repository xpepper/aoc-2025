// ABOUTME: Christmas Tree Farm - fitting presents into regions under trees

#[derive(Debug, PartialEq, Clone)]
pub struct Shape {
    pub index: usize,
    pub cells: Vec<(usize, usize)>,
}

pub fn parse_shape(input: &str) -> Shape {
    let lines: Vec<&str> = input.trim().lines().collect();
    let index = parse_shape_index(lines[0]);
    let cells = parse_shape_cells(&lines[1..]);

    Shape { index, cells }
}

fn parse_shape_index(header: &str) -> usize {
    header.trim_end_matches(':').parse().unwrap()
}

fn parse_shape_cells(shape_lines: &[&str]) -> Vec<(usize, usize)> {
    let mut cells = Vec::new();
    for (y, line) in shape_lines.iter().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if ch == '#' {
                cells.push((x, y));
            }
        }
    }
    cells
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_single_shape() {
        let input = "0:\n###\n##.\n##.";
        let shape = parse_shape(input);

        assert_eq!(shape.index, 0);
        assert_eq!(
            shape.cells,
            vec![(0, 0), (1, 0), (2, 0), (0, 1), (1, 1), (0, 2), (1, 2)]
        );
    }

    #[test]
    fn test_parse_shape_with_different_index() {
        let input = "4:\n###\n#..\n###";
        let shape = parse_shape(input);

        assert_eq!(shape.index, 4);
        assert_eq!(
            shape.cells,
            vec![(0, 0), (1, 0), (2, 0), (0, 1), (0, 2), (1, 2), (2, 2)]
        );
    }
}
