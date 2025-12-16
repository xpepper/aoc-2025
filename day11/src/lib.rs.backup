use std::collections::HashMap;

#[derive(Debug)]
struct ReactorGraph {
    adjacency: HashMap<String, Vec<String>>,
}

impl ReactorGraph {
    fn from_str(input: &str) -> Self {
        let adjacency = input
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty())
            .map(|line| {
                let mut parts = line.split(':');
                let parent = parts
                    .next()
                    .expect("every line should have a parent label")
                    .trim()
                    .to_string();
                let children = parts
                    .next()
                    .map(|rest| rest.split_whitespace().map(str::to_string).collect())
                    .unwrap_or_else(Vec::new);
                (parent, children)
            })
            .collect();

        ReactorGraph { adjacency }
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

    fn count_paths(&self, source: &str, target: &str) -> u128 {
        let mut memo = HashMap::new();
        self.dfs(source, target, &mut memo)
    }
}

pub fn solve_part1(input: &str) -> u128 {
    ReactorGraph::from_str(input).count_paths("you", "out")
}

pub fn solve_part2(input: &str) -> u128 {
    let graph = ReactorGraph::from_str(input);

    let from = |a: &str, b: &str| graph.count_paths(a, b);

    from("svr", "dac") * from("dac", "fft") * from("fft", "out")
        + from("svr", "fft") * from("fft", "dac") * from("dac", "out")
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

    #[test]
    fn samples_part_two() {
        const INPUT: &str = "\
        svr: dac alt
        alt: dac
        dac: mid1 mid2
        mid1: fft
        mid2: fft
        fft: out
        ";

        assert_eq!(4, solve_part2(INPUT));
    }
}
