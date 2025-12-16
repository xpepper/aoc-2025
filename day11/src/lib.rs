// Advent of Code 2025 - Day 11: Reactor
// Part 1: Count paths from 'you' to 'out'

use std::collections::HashMap;

#[derive(Debug)]
struct ReactorGraph {
    adjacency: HashMap<String, Vec<String>>,
}

impl ReactorGraph {
    fn from_str(input: &str) -> Self {
        let adjacency = Self::parse_adjacency(input);
        ReactorGraph { adjacency }
    }

    fn parse_adjacency(input: &str) -> HashMap<String, Vec<String>> {
        input
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty())
            .map(Self::parse_line)
            .collect()
    }

    fn parse_line(line: &str) -> (String, Vec<String>) {
        let mut parts = line.split(':');
        let parent = parts
            .next()
            .expect("every line should have a parent label")
            .trim()
            .to_string();
        let children = parts
            .next()
            .map(|rest| rest.split_whitespace().map(str::to_string).collect())
            .unwrap_or_default();
        (parent, children)
    }

    fn count_paths(&self, source: &str, target: &str) -> u128 {
        let mut memo = HashMap::new();
        self.dfs(source, target, &mut memo)
    }

    fn dfs(&self, current: &str, target: &str, memo: &mut HashMap<String, u128>) -> u128 {
        if current == target {
            return 1;
        }

        if let Some(&cached) = memo.get(current) {
            return cached;
        }

        let count = self.adjacency.get(current).map_or(0, |children| {
            children
                .iter()
                .map(|child| self.dfs(child, target, memo))
                .sum()
        });

        memo.insert(current.to_string(), count);
        count
    }
}

pub fn solve_part1(input: &str) -> u128 {
    let graph = ReactorGraph::from_str(input);
    graph.count_paths("you", "out")
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
    aaa: you hhh
    you: bbb ccc
    bbb: ddd eee
    ccc: ddd eee fff
    ddd: ggg
    eee: out
    fff: out
    ggg: out
    hhh: ccc fff iii
    iii: out
    ";

    #[test]
    fn example_part_one() {
        assert_eq!(5, solve_part1(EXAMPLE));
    }
}
