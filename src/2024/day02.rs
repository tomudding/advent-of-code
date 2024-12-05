// DO NOT EDIT - AOC
use aoc_function_registry::get_registry;
use aoc_proc_macros::aoc;
// END DO NOT EDIT - AOC
// DO NOT EDIT - DEFAULTS
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
// DO NOT EDIT - DEFAULTS

fn is_safe_report(report: &[i32]) -> bool {
    if report.len() < 2 {
        return false;
    }

    let mut is_increasing: bool = true;
    let mut is_decreasing: bool = true;

    for i in 1..report.len() {
        let diff = report[i] - report[i - 1];

        if diff.abs() < 1 || diff.abs() > 3 {
            return false;
        }

        if diff <= 0 {
            is_increasing = false;
        }

        if diff >= 0 {
            is_decreasing = false;
        }
    }

    is_increasing || is_decreasing
}

fn is_safe_with_dampener(report: &[i32]) -> bool {
    for i in 0..report.len() {
        let mut modified_report = report.to_owned();
        modified_report.remove(i);

        if is_safe_report(&modified_report) {
            return true;
        }
    }

    false
}

#[aoc(year = "2024", day = "day02", part = "part1")]
fn part1() -> String {
    let reports: Vec<Vec<i32>> = parse_input();
    let safe_count = reports.iter()
        .filter(|&report| is_safe_report(report))
        .count();

    safe_count.to_string()
}

#[aoc(year = "2024", day = "day02", part = "part2")]
fn part2() -> String {
    let reports: Vec<Vec<i32>> = parse_input();
    let safe_count = reports.iter()
        .filter(|&report| is_safe_report(report) || is_safe_with_dampener(report))
        .count();

    safe_count.to_string()
}

fn get_file_path() -> String {
    format!("./inputs/2024/day02{}.txt", if cfg!(test) { "-example" } else { "" })
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse_input() -> Vec<Vec<i32>> {
    let mut reports: Vec<Vec<i32>> = Vec::new();

    if let Ok(lines) = read_lines(get_file_path()) {
        for value in lines.map_while(Result::ok) {
            let levels: Vec<i32> = value.split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();

            reports.push(levels);
        }
    }

    reports
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(), "2");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), "4");
    }
}
