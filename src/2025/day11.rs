// DO NOT EDIT - AOC
use aoc_proc_macros::aoc;
use aoc_shared_functions::{get_registry, read_lines};
// END DO NOT EDIT - AOC

use std::collections::{HashMap, HashSet};

#[aoc(year = "2025", day = "day11", part = "part1")]
fn part1() -> String {
    let graph: HashMap<String, Vec<String>> = parse_input();

    let mut visited: HashSet<String> = HashSet::new();
    let result: usize = dfs_count(&graph, "you", "out", &mut visited);

    result.to_string()
}

fn dfs_count(
    graph: &HashMap<String, Vec<String>>,
    node: &str,
    goal: &str,
    visited: &mut HashSet<String>,
) -> usize {
    if node == goal {
        return 1;
    }

    if !visited.insert(node.to_string()) {
        return 0;
    }

    let mut total: usize = 0;
    if let Some(children) = graph.get(node) {
        for next in children {
            total += dfs_count(graph, next, goal, visited);
        }
    }

    visited.remove(node);
    total
}

#[aoc(year = "2025", day = "day11", part = "part2")]
fn part2() -> String {
    let graph: HashMap<String, Vec<String>> = parse_input();

    let mut memo: HashMap<(String, String), usize> = HashMap::new();
    let mut visited: HashSet<String> = HashSet::new();

    let svr_dac: usize = dfs_count_memo(&graph, "svr", "dac", &mut memo, &mut visited);
    let dac_fft: usize = dfs_count_memo(&graph, "dac", "fft", &mut memo, &mut visited);
    let fft_out: usize = dfs_count_memo(&graph, "fft", "out", &mut memo, &mut visited);

    let svr_fft: usize = dfs_count_memo(&graph, "svr", "fft", &mut memo, &mut visited);
    let fft_dac: usize = dfs_count_memo(&graph, "fft", "dac", &mut memo, &mut visited);
    let dac_out: usize = dfs_count_memo(&graph, "dac", "out", &mut memo, &mut visited);

    (svr_dac * dac_fft * fft_out + svr_fft * fft_dac * dac_out).to_string()
}

fn dfs_count_memo(
    graph: &HashMap<String, Vec<String>>,
    start: &str,
    goal: &str,
    memo: &mut HashMap<(String, String), usize>,
    visiting: &mut HashSet<String>,
) -> usize {
    if start == goal {
        return 1;
    }
    let key: (String, String) = (start.to_string(), goal.to_string());
    if let Some(&cached) = memo.get(&key) {
        return cached;
    }

    if !visiting.insert(start.to_string()) {
        return 0;
    }

    let mut total: usize = 0;
    if let Some(children) = graph.get(start) {
        for next in children {
            total += dfs_count_memo(graph, next, goal, memo, visiting);
        }
    }

    visiting.remove(start);
    memo.insert(key, total);
    total
}

fn get_file_path() -> String {
    format!("./inputs/2025/day11{}.txt", if cfg!(test) { "-example" } else { "" }) // -example2 for part2
}

fn parse_input() -> HashMap<String, Vec<String>> {
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();

    if let Ok(lines) = read_lines(get_file_path()) {
        for value in lines.map_while(Result::ok) {
            let parts: Vec<&str> = value.split(":").collect();

            let node: String = parts[0].to_string();
            let children: Vec<String> = parts[1]
                .split_whitespace()
                .map(|s| s.to_string())
                .collect();

            graph.insert(node, children);
        }
    }

    graph
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(), "5");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), "2");
    }
}
