// DO NOT EDIT - AOC
use aoc_proc_macros::aoc;
use aoc_shared_functions::{get_registry, read_lines};
// END DO NOT EDIT - AOC

use std::collections::{HashMap, HashSet};

fn generate_secret_number(mut secret: isize, iterations: usize) -> isize {
    for _ in 0..iterations {
        secret = mix_and_prune(secret, secret * 64);
        secret = mix_and_prune(secret, secret / 32);
        secret = mix_and_prune(secret, secret * 2048);
    }

    secret
}

fn mix_and_prune(secret: isize, value: isize) -> isize {
    (secret ^ value) % 16777216
}

#[aoc(year = "2024", day = "day22", part = "part1")]
fn part1() -> String {
    let secret_numbers: Vec<isize> = parse_input();
    let mut total_secrets: isize = 0;

    for secret in secret_numbers {
        total_secrets += generate_secret_number(secret, 2000);
    }

    total_secrets.to_string()
}

fn generate_secret_numbers(mut secret: isize, iterations: usize) -> Vec<(isize, isize)> {
    // Same as `generate_secret_number()` but with the added pattern generation.
    let mut last: isize = secret % 10;
    let mut patterns: Vec<(isize, isize)> = Vec::with_capacity(iterations);

    for _ in 0..iterations {
        secret = mix_and_prune(secret, secret * 64);
        secret = mix_and_prune(secret, secret / 32);
        secret = mix_and_prune(secret, secret * 2048);

        let ones_digit: isize = secret % 10;
        patterns.push((ones_digit - last, ones_digit));
        last = ones_digit;
    }

    patterns
}

#[aoc(year = "2024", day = "day22", part = "part2")]
fn part2() -> String {
    let secret_numbers: Vec<isize> = parse_input();
    let mut banana_sales: HashMap<(isize, isize, isize, isize), isize> = HashMap::new();

    for &secret in &secret_numbers {
        let patterns: Vec<(isize, isize)> = generate_secret_numbers(secret, 2000);

        let mut seen: HashSet<(isize, isize, isize, isize)> = HashSet::new();
        patterns.windows(4)
            .map(|window| ((window[0].0, window[1].0, window[2].0, window[3].0), window[3].1))
            .for_each(|(pattern, bananas)| {
                if seen.insert(pattern) {
                    // Only update if we have not seen this pattern previously.
                    *banana_sales.entry(pattern).or_insert(0) += bananas;
                }
            });
    }

    banana_sales.values().max().unwrap().to_string()
}

fn get_file_path() -> String {
    format!("./inputs/2024/day22{}.txt", if cfg!(test) { "-example" } else { "" })
}

fn parse_input() -> Vec<isize> {
    let mut secret_numbers: Vec<isize> = Vec::new();

    if let Ok(lines) = read_lines(get_file_path()) {
        for value in lines.map_while(Result::ok) {
            secret_numbers.push(value.trim().parse::<isize>().unwrap());
        }
    }

    secret_numbers
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(), "37327623");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), "23"); // my part 2 says "24"... ¯\_(ツ)_/¯
    }
}
