pub fn solve(input: &str) -> u64 {
    let lines: Vec<&str> = input.lines().filter(|l| !l.is_empty()).collect();
    if lines.is_empty() {
        return 0;
    }

    let problem_boundaries = find_problem_boundaries(&lines);
    problem_boundaries
        .iter()
        .map(|(start, end)| solve_problem(&lines, *start, *end))
        .sum()
}

fn find_problem_boundaries(lines: &[&str]) -> Vec<(usize, usize)> {
    let num_data_lines = lines.len() - 1;
    let max_width = lines.iter().map(|l| l.len()).max().unwrap_or(0);

    let mut boundaries = Vec::new();
    let mut problem_start = None;

    for col in 0..=max_width {
        let is_separator = is_separator_column(lines, col, num_data_lines);

        match (is_separator, problem_start) {
            (false, None) => problem_start = Some(col),
            (true, Some(start)) => {
                boundaries.push((start, col));
                problem_start = None;
            }
            _ => {}
        }
    }

    if let Some(start) = problem_start {
        boundaries.push((start, max_width));
    }

    boundaries
}

fn is_separator_column(lines: &[&str], col: usize, num_data_lines: usize) -> bool {
    (0..num_data_lines)
        .all(|row| col >= lines[row].len() || lines[row].chars().nth(col).unwrap_or(' ') == ' ')
}

fn solve_problem(lines: &[&str], start_col: usize, end_col: usize) -> u64 {
    let num_data_lines = lines.len() - 1;
    let op_line = lines[num_data_lines];

    let operation = extract_operation(op_line, start_col, end_col);
    let numbers = extract_numbers_from_problem(lines, start_col, end_col, num_data_lines);

    apply_operation(&numbers, operation)
}

fn extract_operation(op_line: &str, start_col: usize, end_col: usize) -> char {
    op_line
        .chars()
        .skip(start_col)
        .take(end_col - start_col)
        .find(|&ch| ch == '+' || ch == '*')
        .unwrap_or(' ')
}

fn extract_numbers_from_problem(
    lines: &[&str],
    start_col: usize,
    end_col: usize,
    num_data_lines: usize,
) -> Vec<u64> {
    (0..num_data_lines)
        .filter_map(|row| {
            let row_slice = extract_row_slice(lines[row], start_col, end_col);
            parse_number_from_slice(&row_slice)
        })
        .collect()
}

fn extract_row_slice(line: &str, start_col: usize, end_col: usize) -> String {
    line.chars()
        .skip(start_col)
        .take(end_col.saturating_sub(start_col))
        .collect()
}

fn parse_number_from_slice(slice: &str) -> Option<u64> {
    let digits: String = slice.chars().filter(|ch| ch.is_ascii_digit()).collect();
    if digits.is_empty() {
        None
    } else {
        digits.parse().ok()
    }
}

fn apply_operation(numbers: &[u64], operation: char) -> u64 {
    match operation {
        '+' => numbers.iter().sum(),
        '*' => numbers.iter().product(),
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_example_worksheet() {
        let input = "123 328  51 64 \n 45 64  387 23 \n  6 98  215 314\n*   +   *   +  \n";
        let result = solve(input);
        assert_eq!(result, 4277556);
    }
}
