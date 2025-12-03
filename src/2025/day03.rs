// DO NOT EDIT - AOC
use aoc_proc_macros::aoc;
use aoc_shared_functions::{get_registry, read_lines};
// END DO NOT EDIT - AOC

#[aoc(year = "2025", day = "day03", part = "part1")]
fn part1() -> String {
    let banks: Vec<Vec<u8>> = parse_input();

    let total: u128 = banks
        .iter()
        .map(|bank| max_bank_joltage(bank, 2) as u128)
        .sum();

    total.to_string()
}

fn max_bank_joltage(bank: &[u8], battery_size: usize) -> u128 {
    let mut stack: Vec<u8> = Vec::new();
    let mut to_remove: usize = bank.len() - battery_size;

    // Collect all highest values in descending order in the battery bank.
    for &digit in bank {
        while to_remove > 0 && !stack.is_empty() && *stack.last().unwrap() < digit {
            stack.pop();
            to_remove -= 1;
        }

        stack.push(digit);

        if stack.len() == battery_size && to_remove == 0 {
            break;
        }
    }

    // Make sure we only have battery_size turned on.
    stack.truncate(battery_size);

    // Convert to number
    let mut result: u128 = 0;
    for d in stack {
        result = result * 10 + d as u128;
    }

    result
}

#[aoc(year = "2025", day = "day03", part = "part2")]
fn part2() -> String {
    let banks: Vec<Vec<u8>> = parse_input();

    let total: u128 = banks
        .iter()
        .map(|bank| max_bank_joltage(bank, 12))
        .sum();

    total.to_string()
}

fn get_file_path() -> String {
    format!("./inputs/2025/day03{}.txt", if cfg!(test) { "-example" } else { "" })
}

fn parse_input() -> Vec<Vec<u8>> {
    let mut banks: Vec<Vec<u8>> = Vec::new();

    if let Ok(lines) = read_lines(get_file_path()) {
        for value in lines.map_while(Result::ok) {
            let bank: Vec<u8> = value
                .chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect();

            if !bank.is_empty() {
                banks.push(bank);
            }
        }
    }

    banks
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(), "357");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), "3121910778619");
    }
}
