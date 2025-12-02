// DO NOT EDIT - AOC
use aoc_proc_macros::aoc;
use aoc_shared_functions::{get_registry, read_lines};
// END DO NOT EDIT - AOC

#[aoc(year = "2025", day = "day02", part = "part1")]
fn part1() -> String {
    let ranges: Vec<(usize, usize)> = parse_input();
    let mut total: usize = 0;

    for (a, b) in ranges {
        total += get_count_repeated_sequences_in_range(a, b);
    }

    total.to_string()
}

fn get_count_repeated_sequences_in_range(a: usize, b: usize) -> usize {
    let mut sum: usize = 0;
    let s: String = b.to_string();

    // Based on input given, k can never be larger than 5 (id in our ranges is max 2k digits long).
    for k in 1..=s.len() / 2 {
        let base: usize = 10_usize.pow(k as u32);
        let factor: usize = base + 1;
        let min_x_digits = base / 10;
        let max_x_digits = base - 1;

        let x_from_a: usize = (a + factor - 1) / factor;
        let x_from_b: usize = b / factor;

        let x_lo: usize = min_x_digits.max(x_from_a);
        let x_hi: usize = max_x_digits.min(x_from_b);

        if x_lo <= x_hi {
            let n: usize = x_hi - x_lo + 1;
            let sum_x: usize = n * (x_lo + x_hi) / 2;

            sum += sum_x * factor;
        }
    }

    sum
}

#[aoc(year = "2025", day = "day02", part = "part2")]
fn part2() -> String {
    let ranges: Vec<(usize, usize)> = parse_input();
    let mut total: usize = 0;

    for (start, end) in ranges.iter() {
        // Brrrr, just bruteforce. Ranges are short enough.
        for id in *start..=*end {
            if is_invalid_id(id) {
                total += id;
            }
        }
    }

    total.to_string()
}

fn is_invalid_id(n: usize) -> bool {
    let s: String = n.to_string();

    // Be smart, determine max size of the repeated sequences (up to half of the original id).
    for p in 1..=s.len() / 2 {
        // An id is invalid if some potential sequence of length p fits in the id...
        if s.len() % p == 0 {
            // ...and we can construct such sequence of length p from first p digits.
            if s[..p].repeat(s.len() / p) == s {
                return true;
            }
        }
    }

    false
}

fn get_file_path() -> String {
    format!("./inputs/2025/day02{}.txt", if cfg!(test) { "-example" } else { "" })
}

fn parse_input() -> Vec<(usize, usize)> {
    let mut out: Vec<(usize, usize)> = Vec::new();

    if let Ok(lines) = read_lines(get_file_path()) {
        for line in lines.map_while(Result::ok) {
            for range in line.split(',') {
                let parts: Vec<&str> = range.split("-").collect();
                let start: usize = parts[0].parse::<usize>().unwrap();
                let end: usize = parts[1].parse::<usize>().unwrap();

                out.push((start, end));
            }

            break;
        }
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(), "1227775554");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), "4174379265");
    }
}
