// DO NOT EDIT - AOC
use aoc_proc_macros::aoc;
use aoc_shared_functions::{get_registry, read_lines};
// END DO NOT EDIT - AOC

use std::collections::HashMap;

fn blink_stones(stones: &mut HashMap<usize, usize>, num_blinks: usize) -> String {
    for _ in 0..num_blinks {
        let mut next_stones: HashMap<usize, usize> = HashMap::new();

        for (&s, &v) in stones.iter() {
            if s == 0 {
                *next_stones.entry(1).or_default() += v;
            } else if s.to_string().len() % 2 == 0 {
                let x: String = s.to_string();
                *next_stones.entry(x[0..x.len() / 2].parse().unwrap()).or_default() += v;
                *next_stones.entry(x[x.len() / 2..].parse().unwrap()).or_default() += v;
            } else {
                *next_stones.entry(s * 2024).or_default() += v;
            }
        }

        *stones = next_stones;
    }

    stones.values().sum::<usize>().to_string()
}

#[allow(dead_code)]
fn inefficient_blink_stones(stones: &mut Vec<u64>, num_blinks: u64) -> String {
    for _ in 0..num_blinks {
        let mut next_stones: Vec<u64> = Vec::new();

        for &stone in stones.iter() {
            if stone == 0 {
                next_stones.push(1);
            } else if stone.to_string().len() % 2 == 0 {
                let s: String = stone.to_string();
                let mid = s.len() / 2;
                let (left, right) = s.split_at(mid);

                next_stones.push(left.trim_start_matches('0').parse().unwrap_or(0));
                next_stones.push(right.trim_start_matches('0').parse().unwrap_or(0));
            } else {
                next_stones.push(stone * 2024);
            }
        }

        *stones = next_stones;
    }

    stones.len().to_string()
}

#[aoc(year = "2024", day = "day11", part = "part1")]
fn part1() -> String {
    let mut stones: HashMap<usize, usize> = parse_input();

    blink_stones(&mut stones, 25)
}

#[aoc(year = "2024", day = "day11", part = "part2")]
fn part2() -> String {
    let mut stones: HashMap<usize, usize> = parse_input();

    blink_stones(&mut stones, 75)
}

fn get_file_path() -> String {
    format!("./inputs/2024/day11{}.txt", if cfg!(test) { "-example" } else { "" })
}

fn parse_input() -> HashMap<usize, usize> {
    let mut stones: HashMap<usize, usize> = HashMap::new();

    if let Ok(lines) = read_lines(get_file_path()) {
        for value in lines.map_while(Result::ok) {
            for stone in value.split_whitespace() {
                *stones.entry(stone.parse::<usize>().unwrap()).or_default() += 1;
            }
        }
    }

    stones
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(), "55312");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), todo!());
    }
}
