// DO NOT EDIT - AOC
use aoc_proc_macros::aoc;
use aoc_shared_functions::{get_registry, read_lines};
// END DO NOT EDIT - AOC

enum Operation {
    Add,
    Mul,
}

struct Problem {
    numbers: Vec<usize>,
    operation: Operation,
}

#[aoc(year = "2025", day = "day06", part = "part1")]
fn part1() -> String {
    let problems: Vec<Problem> = parse_input();
    let mut total: usize = 0;

    for problem in problems {
        total += match problem.operation {
            Operation::Add => problem.numbers.into_iter().sum::<usize>(),
            Operation::Mul => problem.numbers.into_iter().product(),
        };
    }

    total.to_string()
}

#[aoc(year = "2025", day = "day06", part = "part2")]
fn part2() -> String {
    let problems: Vec<Problem> = parse_input2();
    let mut total: usize = 0;

    for problem in problems {
        total += match problem.operation {
            Operation::Add => problem.numbers.into_iter().sum::<usize>(),
            Operation::Mul => problem.numbers.into_iter().product(),
        };
    }

    total.to_string()
}

fn get_file_path() -> String {
    format!("./inputs/2025/day06{}.txt", if cfg!(test) { "-example" } else { "" })
}

fn parse_input() -> Vec<Problem> {
    let mut problems: Vec<Problem> = Vec::new();
    let mut rows: Vec<Vec<String>> = Vec::new();

    if let Ok(lines) = read_lines(get_file_path()) {
        for line in lines.map_while(Result::ok) {
            let parts: Vec<String> = line
                .split_whitespace()
                .map(|s| s.to_string())
                .collect();

            rows.push(parts);
        }
    }

    // Last row contains operators
    let bottom_row: Vec<String> = rows.pop().unwrap();
    for (column_idx, operation_char) in bottom_row.iter().enumerate() {
        let operation: Operation = match operation_char.as_str() {
            "+" => Operation::Add,
            "*" => Operation::Mul,
            _ => unreachable!(),
        };

        let mut numbers: Vec<usize> = Vec::new();
        for row in &rows {
            if column_idx < row.len() {
                numbers.push(row[column_idx].parse::<usize>().unwrap());
            }
        }

        problems.push(Problem { numbers, operation });
    }

    problems
}

fn parse_input2() -> Vec<Problem> {
    let mut problems: Vec<Problem> = Vec::new();
    let mut rows: Vec<String> = Vec::new();

    if let Ok(iter) = read_lines(get_file_path()) {
        for value in iter.map_while(Result::ok) {
            rows.push(value);
        }
    }

    // No point in keeping the whole thing as a String, already go to char level. Cannot pop like
    // with part1 due to owner constraints.
    let bottom_row: &str = rows.last().map(|s| s.as_str()).unwrap();
    let number_rows: &[String] = &rows[..rows.len() - 1];
    // This only works for real input, not the test input because my IDE keeps stripping spaces.
    let width: usize = number_rows[0].len();
    let mut block_end: usize = width;

    // .bytes also an option instead of .char_indices() since only simple characters?
    for (column_idx, operation_char) in bottom_row.char_indices().rev() {
        // Only parse back to block_end if we have an operation, because we know they delimit the
        // columns for each problem.
        if operation_char == '+' || operation_char == '*' {
            let operation: Operation = match operation_char {
                '+' => Operation::Add,
                '*' => Operation::Mul,
                _ => unreachable!(),
            };

            let mut numbers: Vec<usize> = Vec::new();
            // Parsing direction does not matter because column based, so simply scan LTR.
            for digit_column in column_idx..block_end {
                let mut digits_str: String = String::new();
                for row in number_rows {
                    let c: char = row.chars().nth(digit_column).unwrap();

                    if c.is_ascii_digit() {
                        digits_str.push(c);
                    }
                }

                if !digits_str.is_empty() {
                    numbers.push(digits_str.parse::<usize>().unwrap());
                }
            }

            problems.push(Problem { numbers, operation });
            block_end = column_idx;
        }
    }

    problems
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(), "4277556");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), "3263827");
    }
}
