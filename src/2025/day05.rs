// DO NOT EDIT - AOC
use aoc_proc_macros::aoc;
use aoc_shared_functions::{get_registry, read_lines};
// END DO NOT EDIT - AOC

#[aoc(year = "2025", day = "day05", part = "part1")]
fn part1() -> String {
    let (ranges, ids): (Vec<(usize, usize)>, Vec<usize>) = parse_input();

    let fresh_count: usize = ids
        .iter()
        .filter(|&&id| ranges.iter().any(|&(first_id, last_id)| first_id <= id && id <= last_id))
        .count();

    fresh_count.to_string()
}

#[aoc(year = "2025", day = "day05", part = "part2")]
fn part2() -> String {
    let (mut ranges, _ids): (Vec<(usize, usize)>, Vec<usize>) = parse_input();

    ranges.sort_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));

    // Merge overlapping or adjacent ranges.
    let mut merged_ranges: Vec<(usize, usize)> = Vec::with_capacity(ranges.len());
    let mut current_range: (usize, usize) = ranges[0];

    for &(start, end) in &ranges[1..] {
        if start <= current_range.1 + 1 {
            current_range.1 = current_range.1.max(end);
        } else {
            merged_ranges.push(current_range);
            current_range = (start, end);
        }
    }

    // Do not forget about the last range...
    merged_ranges.push(current_range);

    // Account for inclusiveness of last part of the ranges.
    let total: usize = merged_ranges
        .into_iter()
        .map(|(start, end)| end - start + 1)
        .sum();

    total.to_string()
}

fn get_file_path() -> String {
    format!("./inputs/2025/day05{}.txt", if cfg!(test) { "-example" } else { "" })
}

fn parse_input() -> (Vec<(usize, usize)>, Vec<usize>) {
    let mut ranges: Vec<(usize, usize)> = Vec::new();
    let mut ids: Vec<usize> = Vec::new();
    let mut parsing_ranges: bool = true;

    if let Ok(lines) = read_lines(get_file_path()) {
        for value in lines.map_while(Result::ok) {
            if value.trim().is_empty() {
                parsing_ranges = false;
                continue;
            }

            if parsing_ranges {
                let parts: Vec<&str> = value.split("-").collect();
                let start: usize = parts[0].parse::<usize>().unwrap();
                let end: usize = parts[1].parse::<usize>().unwrap();

                ranges.push((start, end));
            } else {
                let id: usize = value.parse::<usize>().unwrap();

                ids.push(id);
            }
        }
    }

    (ranges, ids)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(), "3");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), "14");
    }
}
