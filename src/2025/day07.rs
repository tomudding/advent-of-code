// DO NOT EDIT - AOC
use aoc_proc_macros::aoc;
use aoc_shared_functions::{get_registry, read_lines};
// END DO NOT EDIT - AOC

#[aoc(year = "2025", day = "day07", part = "part1")]
fn part1() -> String {
    let manifold: Vec<Vec<char>> = parse_input();
    let rows: usize = manifold.len();
    let cols: usize = manifold[0].len();

    let mut beam_grid: Vec<Vec<bool>> = vec![vec![false; cols]; rows + 1];
    let mut total_splits: usize = 1;

    let start_col: usize = manifold[0].iter().position(|&c| c == 'S').unwrap();
    beam_grid[1][start_col] = true; // set first beam position (one row below S)

    for y in 1..(rows - 1) {
        for x in 0..cols {
            let is_beam_present: bool = beam_grid[y][x];

            if is_beam_present {
                // No beam, nothing to propagate.
                continue;
            }

            match manifold[y + 1][x] {
                '.' => {
                    // Empty space: beam continues downward to the next row (y + 1).
                    if y + 1 <= rows {
                        beam_grid[y + 1][x] = true;
                    }
                },
                '^' => {
                    // Splitter: beam splits left and right to the next row (x + 1).
                    total_splits += 1;

                    // Split left.
                    if x > 0 && y + 1 <= rows {
                        beam_grid[y + 1][x - 1] = true;
                    }

                    // Split right.
                    if x < cols - 1 && y + 1 <= rows {
                        beam_grid[y + 1][x + 1] = true;
                    }
                },
                _ => unreachable!(),
            }
        }
    }

    total_splits.to_string()
}

#[aoc(year = "2025", day = "day07", part = "part2")]
fn part2() -> String {
    let manifold: Vec<Vec<char>> = parse_input();
    let rows: usize = manifold.len();
    let cols: usize = manifold[0].len();

    let mut timelines: Vec<Vec<usize>> = vec![vec![0; cols]; rows + 1];
    let mut total_splits: usize = 1;

    let start_col: usize = manifold[0].iter().position(|&c| c == 'S').unwrap();
    timelines[1][start_col] = 1; // set first beam position (one row below S)

    for y in 1..(rows - 1) {
        for x in 0..cols {
            let timeline_count: usize = timelines[y][x];

            if timeline_count == 0 {
                // No beam/timeline, nothing to propagate.
                continue;
            }

            match manifold[y + 1][x] {
                '.' => {
                    // Empty space: beam continues downward to the next row (y + 1).
                    if y + 1 <= rows {
                        timelines[y + 1][x] += timeline_count;
                    }
                },
                '^' => {
                    // Splitter: beam splits left and right to the next row (x + 1).
                    total_splits += timeline_count;

                    // Split left.
                    if x > 0 && y + 1 <= rows {
                        timelines[y + 1][x - 1] += timeline_count;
                    }

                    // Split right.
                    if x < cols - 1 && y + 1 <= rows {
                        timelines[y + 1][x + 1] += timeline_count;
                    }
                },
                _ => unreachable!(),
            }
        }
    }

    total_splits.to_string()
}

fn get_file_path() -> String {
    format!("./inputs/2025/day07{}.txt", if cfg!(test) { "-example" } else { "" })
}

fn parse_input() -> Vec<Vec<char>> {
    let mut manifold: Vec<Vec<char>> = Vec::new();

    if let Ok(lines) = read_lines(get_file_path()) {
        for value in lines.map_while(Result::ok) {
            manifold.push(value.chars().collect());
        }
    }

    manifold
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(), "21");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), "40");
    }
}
