// DO NOT EDIT - AOC
use aoc_proc_macros::aoc;
use aoc_shared_functions::{get_registry, read_lines};
// END DO NOT EDIT - AOC

use std::collections::HashMap;

#[aoc(year = "2024", day = "day01", part = "part1")]
fn part1() -> String {
    let (left_list, right_list): (Vec<i32>, Vec<i32>) = parse_input();
    let mut left_list: Vec<i32> = left_list;
    let mut right_list: Vec<i32> = right_list;

    left_list.sort();
    right_list.sort();

    let sum: i32 = left_list.iter()
        .zip(right_list.iter())
        .map(|(l, r)| (l - r).abs())
        .sum();

    sum.to_string()
}

#[aoc(year = "2024", day = "day01", part = "part2")]
fn part2() -> String {
    let (left_list, right_list): (Vec<i32>, Vec<i32>) = parse_input();
    let mut right_map: HashMap<i32, i32> = HashMap::new();

    for number in right_list {
        *right_map.entry(number).or_insert(0) += 1;
    }

    let sum: i32 = left_list.iter()
        .map(|&number| number * right_map.get(&number).unwrap_or(&0))
        .sum();

    sum.to_string()
}

fn get_file_path() -> String {
    format!("./inputs/2024/day01{}.txt", if cfg!(test) { "-example" } else { "" })
}

fn parse_input() -> (Vec<i32>, Vec<i32>) {
    let mut left_list: Vec<i32> = Vec::new();
    let mut right_list: Vec<i32> = Vec::new();

    if let Ok(lines) = read_lines(get_file_path()) {
        for value in lines.map_while(Result::ok) {
            let parts: Vec<i32> = value.split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();
            left_list.push(parts[0]);
            right_list.push(parts[1]);
        }
    }

    (left_list, right_list)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(), String::from("11"));
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), String::from("31"));
    }
}
