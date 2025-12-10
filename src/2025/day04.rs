// DO NOT EDIT - AOC
use aoc_proc_macros::aoc;
use aoc_shared_functions::{get_registry, read_lines};
// END DO NOT EDIT - AOC

#[aoc(year = "2025", day = "day04", part = "part1")]
fn part1() -> String {
    let grid: Vec<Vec<char>> = parse_input();
    let mut accessible_count: i32 = 0;

    let rows: usize = grid.len();
    let cols: usize = grid[0].len();

    let directions: [(isize, isize); 8] = [
        (-1, -1), (-1, 0), (-1, 1),
        (0,  -1),          (0,  1),
        (1,  -1), (1,  0), (1,  1),
    ];

    for y in 0..rows {
        for x in 0..cols {
            if grid[y][x] == '@' {
                let mut adjacent_rolls: i32 = 0;

                for &(dy, dx) in directions.iter() {
                    let ny = y as isize + dy;
                    let nx = x as isize + dx;

                    if ny >= 0 && ny < rows as isize && nx >= 0 && nx < cols as isize
                        && grid[ny as usize][nx as usize] == '@' {
                            adjacent_rolls += 1;
                    }
                }

                if adjacent_rolls < 4 {
                    accessible_count += 1;
                }
            }
        }
    }

    accessible_count.to_string()
}

#[aoc(year = "2025", day = "day04", part = "part2")]
fn part2() -> String {
    let mut grid: Vec<Vec<char>> = parse_input();
    let mut removed_total: i32 = 0;

    let rows: usize = grid.len();
    let cols: usize = grid[0].len();

    let directions: [(isize, isize); 8] = [
        (-1, -1), (-1, 0), (-1, 1),
        (0,  -1),          (0,  1),
        (1,  -1), (1,  0), (1,  1),
    ];

    loop {
        let mut to_remove: Vec<(usize, usize)> = Vec::new();

        for y in 0..rows {
            for x in 0..cols {
                if grid[y][x] == '@' {
                    let mut adjacent_rolls: i32 = 0;

                    for &(dy, dx) in directions.iter() {
                        let ny = y as isize + dy;
                        let nx = x as isize + dx;

                        if ny >= 0 && ny < rows as isize && nx >= 0 && nx < cols as isize
                            && grid[ny as usize][nx as usize] == '@' {
                                adjacent_rolls += 1;
                        }
                    }

                    if adjacent_rolls < 4 {
                        to_remove.push((y, x));
                    }
                }
            }
        }

        if to_remove.is_empty() {
            break;
        }

        // Remove them
        for (y, x) in to_remove {
            grid[y][x] = '.';
            removed_total += 1;
        }
    }

    removed_total.to_string()
}

fn get_file_path() -> String {
    format!("./inputs/2025/day04{}.txt", if cfg!(test) { "-example" } else { "" })
}

fn parse_input() -> Vec<Vec<char>> {
    let mut grid: Vec<Vec<char>> = Vec::new();

    if let Ok(lines) = read_lines(get_file_path()) {
        for value in lines.map_while(Result::ok) {
            grid.push(value.chars().collect());
        }
    }

    grid
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(), "13");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), "43");
    }
}
