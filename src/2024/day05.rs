// DO NOT EDIT - AOC
use aoc_function_registry::get_registry;
use aoc_proc_macros::aoc;
// END DO NOT EDIT - AOC
// DO NOT EDIT - DEFAULTS
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
// DO NOT EDIT - DEFAULTS

use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::HashSet;

fn is_correct_order(update: &[usize], page_ordering_rules: &Vec<(usize, usize)>) -> bool {
    let mut position: HashMap<usize, usize> = HashMap::new();

    for (i, &page) in update.iter().enumerate() {
        position.insert(page, i);
    }

    for &(x, y) in page_ordering_rules {
        if let (Some(&px), Some(&py)) = (position.get(&x), position.get(&y)) {
            if px > py {
                return false;
            }
        }
    }

    true
}

#[aoc(year = "2024", day = "day05", part = "part1")]
fn part1() -> String {
    let (page_ordering_rules, updates): (Vec<(usize, usize)>, Vec<Vec<usize>>) = parse_input();
    let mut total_sum: usize = 0;

    for update in &updates {
        if is_correct_order(update, &page_ordering_rules) {
            total_sum += update[update.len() / 2];
        }
    }

    total_sum.to_string()
}

fn fix_update_order(update: &[usize], page_ordering_rules: &Vec<(usize, usize)>) -> Vec<usize> {
    let mut rule_map: HashMap<usize, HashSet<usize>> = HashMap::new();
    let mut ordered_update: Vec<usize> = update.to_owned();

    for &(x, y) in page_ordering_rules {
        rule_map.entry(x).or_default().insert(y);
    }

    ordered_update.sort_by(|a, b| {
        if rule_map.get(a).map_or(false, |set| set.contains(b)) {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    });

    ordered_update
}

#[aoc(year = "2024", day = "day05", part = "part2")]
fn part2() -> String {
    let (page_ordering_rules, updates): (Vec<(usize, usize)>, Vec<Vec<usize>>) = parse_input();
    let mut total_sum: usize = 0;

    for update in &updates {
        if !is_correct_order(update, &page_ordering_rules) {
            let ordered_update = fix_update_order(update, &page_ordering_rules);
            total_sum += ordered_update[ordered_update.len() / 2];
        }
    }

    total_sum.to_string()
}

fn get_file_path() -> String {
    format!("./inputs/2024/day05{}.txt", if cfg!(test) { "-example" } else { "" })
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse_input() -> (Vec<(usize, usize)>, Vec<Vec<usize>>) {
    let mut page_ordering_rules: Vec<(usize, usize)> = Vec::new();
    let mut updates: Vec<Vec<usize>> = Vec::new();
    let mut is_rule = true;

    if let Ok(lines) = read_lines(get_file_path()) {
        for value in lines.map_while(Result::ok) {
            // Split input on the one empty line there is.
            if value.trim().is_empty() {
                is_rule = false;

                continue;
            }

            if is_rule {
                let parts: Vec<&str> = value.split('|').collect();
                page_ordering_rules.push((parts[0].parse().unwrap(), parts[1].parse().unwrap()));
            } else {
                let update: Vec<usize> = value.split(',').map(|s| s.parse().unwrap()).collect();
                updates.push(update);
            }
        }
    }

    (page_ordering_rules, updates)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(), "143");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), "123");
    }
}
