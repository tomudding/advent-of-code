// DO NOT EDIT - AOC
use aoc_proc_macros::aoc;
use aoc_shared_functions::{get_registry, read_lines};
// END DO NOT EDIT - AOC

fn can_make_equation_work(numbers: &[i64], target: i64, allow_concatenate: bool) -> bool {
    let n: usize = numbers.len();
    let operators: Vec<&str> = if allow_concatenate {
        vec![
            "+",
            "*",
            "||",
        ]
    } else {
        vec![
            "+",
            "*",
        ]
    };

    let mut stack: Vec<(i64, i64)> = Vec::new();
    stack.push((1, numbers[0]));

    for i in 1..n {
        let mut new_stack: Vec<(i64, i64)> = Vec::new();

        while let Some((product, sum)) = stack.pop() {
            for &op in &operators {
                let new_sum: i64 = match op {
                    "+" => sum + numbers[i],
                    "*" => sum * numbers[i],
                    "||" => format!("{}{}", sum, numbers[i]).parse::<i64>().unwrap(),
                    _ => unreachable!(),
                };

                new_stack.push((product, new_sum));
            }
        }

        stack = new_stack;
    }

    stack.into_iter().any(|(_, sum)| sum == target)
}

#[aoc(year = "2024", day = "day07", part = "part1")]
fn part1() -> String {
    let equations: Vec<(i64, Vec<i64>)> = parse_input();
    let mut calibration_result: i64 = 0;

    for (test_value, numbers) in equations {
        if can_make_equation_work(&numbers, test_value, false) {
            calibration_result += test_value;
        }
    }

    calibration_result.to_string()
}

#[aoc(year = "2024", day = "day07", part = "part2")]
fn part2() -> String {
    let equations: Vec<(i64, Vec<i64>)> = parse_input();
    let mut calibration_result: i64 = 0;

    for (test_value, numbers) in equations {
        if can_make_equation_work(&numbers, test_value, true) {
            calibration_result += test_value;
        }
    }

    calibration_result.to_string()
}

fn get_file_path() -> String {
    format!("./inputs/2024/day07{}.txt", if cfg!(test) { "-example" } else { "" })
}

fn parse_input() -> Vec<(i64, Vec<i64>)> {
    let mut equations: Vec<(i64, Vec<i64>)> = Vec::new();

    if let Ok(lines) = read_lines(get_file_path()) {
        for value in lines.map_while(Result::ok) {
            let parts: Vec<&str> = value.split(":").collect();
            // Large numbers in input, so i64 is required.
            let test_value: i64 = parts[0].parse().unwrap();
            let numbers: Vec<i64> = parts[1].split_whitespace().map(|s| s.parse().unwrap()).collect();

            equations.push((test_value, numbers));
        }
    }

    equations
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(), "3749");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), "11387");
    }
}
