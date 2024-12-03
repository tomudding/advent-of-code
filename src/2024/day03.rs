// DO NOT EDIT - AOC
use aoc_function_registry::get_registry;
use aoc_proc_macros::aoc;
// END DO NOT EDIT - AOC
// DO NOT EDIT - DEFAULTS
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
// DO NOT EDIT - DEFAULTS

use regex::Regex;

#[aoc(year = "2024", day = "day03", part = "part1")]
fn part1() -> String {
    let sum: i32 = parse_input();

    sum.to_string()
}

#[aoc(year = "2024", day = "day03", part = "part2")]
fn part2() -> String {
    // Why did the example input change :(
    let sum: i32 = parse_input_with_conditions();

    sum.to_string()
}

fn get_file_path() -> String {
    format!("./inputs/2024/day03{}.txt", if cfg!(test) { "-example" } else { "" })
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse_input() -> i32 {
    // Minor cheat by using regex crate.
    let mut total_sum: i32 = 0;
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    if let Ok(lines) = read_lines(get_file_path()) {
        for line in lines {
            if let Ok(value) = line {
                for cap in re.captures_iter(&value) {
                    let x: i32 = cap[1].parse().unwrap();
                    let y: i32 = cap[2].parse().unwrap();
                    total_sum += x * y;
                }
            }
        }
    }

    total_sum
}

fn parse_input_with_conditions() -> i32 {
    // Minor cheat by using regex crate.
    let mut total_sum: i32 = 0;
    let mut mul_enabled: bool = true;

    let re_mul = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let re_do = Regex::new(r"do\(\)").unwrap();
    let re_dont = Regex::new(r"don't\(\)").unwrap();

    if let Ok(lines) = read_lines(get_file_path()) {
        for line in lines {
            if let Ok(value) = line {
                // We cannot just capture everything with the regex and determine the enabled state,
                // we have to go through each line and determine what state we are in.
                let mut current_position = 0;

                while current_position < value.len() {
                    // If we find a do(), set mul enabled and skip forward.
                    if let Some(cap) = re_do.find(&value[current_position..]) {
                        // We need to check that this is actually at the start of where we are now,
                        // as it may happen that a mul() appears before this (such as in the example
                        // that is given).
                        if cap.start() == 0 {
                            mul_enabled = true;
                            current_position += cap.end();

                            continue;
                        }
                    }

                    // If we find a don't(), set mul disabled and skip forward.
                    if let Some(cap) = re_dont.find(&value[current_position..]) {
                        // We need to check that this is actually at the start of where we are now,
                        // as it may happen that a mul() appears before this (such as in the example
                        // that is given).
                        if cap.start() == 0 {
                            mul_enabled = false;
                            current_position += cap.end();

                            continue;
                        }
                    }

                    // If we find a mul(), do the multiplication only if mul is enabled.
                    if let Some(cap) = re_mul.captures(&value[current_position..]) {
                        // We need to check that this is actually at the start of where we are now,
                        // as it may happen that a do() or don't() appears before this influencing
                        // the mul_enabled state.
                        if cap.get(0).unwrap().start() == 0 && mul_enabled {
                            let x: i32 = cap[1].parse().unwrap();
                            let y: i32 = cap[2].parse().unwrap();
                            total_sum += x * y;
                            current_position += cap.get(0).unwrap().end();

                            continue;
                        }
                    }

                    current_position += 1;
                }
            }
        }
    }

    total_sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(), "161");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), "48");
    }
}
