// DO NOT EDIT - AOC
use aoc_proc_macros::aoc;
use aoc_shared_functions::{get_registry, read_lines};
// END DO NOT EDIT - AOC

fn count_normal_occurrences(grid: &[Vec<char>]) -> usize {
    let directions: Vec<(i32, i32)> = vec![
        (0, 1),   // ->
        (1, 0),   // \/
        (1, 1),   // _\| (diagonal down right)
        (1, -1),  // |/_ (diagonal down left)
        (0, -1),  // <-
        (-1, 0),  // /\
        (-1, -1), // |\- (diagonal up left)
        (-1, 1),  // -/| (diagonal up right)
    ];

    let mut count: usize = 0;
    let word: Vec<char> = "XMAS".chars().collect();

    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            'inner_outer: for &(dx, dy) in &directions {
                let mut x: isize = col as isize;
                let mut y: isize = row as isize;

                for &letter in &word {
                    if y < 0 || y >= grid.len() as isize || x < 0 || x >= grid[0].len() as isize {
                        // We are outside the grid, skip!
                        continue 'inner_outer;
                    }

                    if grid[y as usize][x as usize] != letter {
                        continue 'inner_outer;
                    }

                    x += dx as isize;
                    y += dy as isize;
                }

                count += 1;
            }
        }
    }

    count
}

fn count_x_mas_occurrences(grid: &[Vec<char>]) -> usize {
    let mut count: usize = 0;

    for row in 1..grid.len() - 1 {
        for col in 1..grid[row].len() - 1 {
            // We do an X-check (instead of the direction method from part1), so if we do not have
            // an A we do not have to check anything.
            if grid[row][col] != 'A' {
                continue;
            }

            // ul .  ur
            // .  A  .
            // dl .  dr
            let ul = grid[row - 1][col - 1];
            let ur = grid[row - 1][col + 1];
            let dl = grid[row + 1][col - 1];
            let dr = grid[row + 1][col + 1];

            if (ul == 'M' && dr == 'S' || ul == 'S' && dr == 'M')
                && (ur == 'M' && dl == 'S' || ur == 'S' && dl == 'M') {
                count += 1;
            }
        }
    }

    count
}

#[aoc(year = "2024", day = "day04", part = "part1")]
fn part1() -> String {
    let grid: Vec<Vec<char>> = parse_input();
    let count = count_normal_occurrences(&grid);

    count.to_string()
}

#[aoc(year = "2024", day = "day04", part = "part2")]
fn part2() -> String {
    let grid: Vec<Vec<char>> = parse_input();
    let count = count_x_mas_occurrences(&grid);

    count.to_string()
}

fn get_file_path() -> String {
    format!("./inputs/2024/day04{}.txt", if cfg!(test) { "-example" } else { "" })
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
        assert_eq!(part1(), "18");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), "9");
    }
}
