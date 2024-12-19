// DO NOT EDIT - AOC
use aoc_proc_macros::aoc;
use aoc_shared_functions::{get_registry, read_lines};
// END DO NOT EDIT - AOC

use std::collections::{HashSet, HashMap};

fn can_make_design(towel_patterns: &HashSet<String>, design: &str) -> bool {
    if design.is_empty() {
        return true;
    }

    for pattern in towel_patterns {
        if design.starts_with(pattern) {
            if can_make_design(&towel_patterns, &design[pattern.len()..]) {
                return true;
            }
        }
    }

    false
}

#[aoc(year = "2024", day = "day19", part = "part1")]
fn part1() -> String {
    let (towel_patters, designs): (HashSet<String>, Vec<String>) = parse_input();
    let mut possible_designs: isize = 0;

    for design in designs {
        if can_make_design(&towel_patters, &design) {
            possible_designs += 1;
        }
    }

    possible_designs.to_string()
}

// "consider introducing a named lifetime parameter" => so 'a necessary (thanks compiler!)
fn count_ways_to_make_design<'a>(towel_patterns: &HashSet<String>, design: &'a str, known_counts: &mut HashMap<&'a str, usize>) -> usize {
    if design.is_empty() {
        return 1;
    }

    if let Some(&count) = known_counts.get(design) {
        return count;
    }

    let mut total_count: usize = 0;
    for pattern in towel_patterns {
        if design.starts_with(pattern) {
            total_count += count_ways_to_make_design(&towel_patterns, &design[pattern.len()..], known_counts);
        }
    }

    known_counts.insert(design, total_count);

    total_count
}

#[aoc(year = "2024", day = "day19", part = "part2")]
fn part2() -> String {
    let (towel_patters, designs): (HashSet<String>, Vec<String>) = parse_input();
    let mut possible_designs: usize = 0;

    for design in designs {
        possible_designs += count_ways_to_make_design(&towel_patters, &design, &mut HashMap::new());
    }

    possible_designs.to_string()
}

fn get_file_path() -> String {
    format!("./inputs/2024/day19{}.txt", if cfg!(test) { "-example" } else { "" })
}

fn parse_input() -> (HashSet<String>, Vec<String>) {
    let mut towel_patterns: HashSet<String> = HashSet::new();
    let mut designs: Vec<String> = Vec::new();
    let mut is_towel_patterns: bool = true;

    if let Ok(lines) = read_lines(get_file_path()) {
        for value in lines.map_while(Result::ok) {
            if value.is_empty() {
                is_towel_patterns = false;

                continue;
            }

            if is_towel_patterns {
                for pattern in value.split([',']) {
                    towel_patterns.insert(String::from(pattern.trim()));
                }
            } else {
                designs.push(value);
            }
        }
    }

    (towel_patterns, designs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(), "6");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), "16");
    }
}
