// DO NOT EDIT - AOC
use aoc_proc_macros::aoc;
use aoc_shared_functions::{get_registry, read_lines};
// END DO NOT EDIT - AOC

use std::collections::{HashSet, VecDeque};

#[aoc(year = "2024", day = "day18", part = "part1")]
fn part1() -> String {
    let bytes: Vec<(isize, isize)> = parse_input();
    // let grid_size: isize = 6; // 6 for example, from 0..=grid_size.
    let grid_size: isize = 70;
    // let number_of_bytes_to_simulate: isize = 12; // 12 for example.
    let number_of_bytes_to_simulate: isize = 1024;

    let start: (isize, isize) = (0, 0);
    let end: (isize, isize) = (grid_size, grid_size);

    let mut grid: Vec<Vec<bool>> = vec![vec![false; grid_size as usize + 1]; grid_size as usize + 1];
    for &(x, y) in bytes.iter().take(number_of_bytes_to_simulate as usize) {
        grid[y as usize][x as usize] = true;
    }

    let directions: [(isize, isize); 4] = [
        (0, -1), // ^
        (1, 0),  // >
        (0, 1),  // v
        (-1, 0), // <
    ];

    let mut queue: VecDeque<((isize, isize), isize)> = VecDeque::new();
    let mut visited: HashSet<(isize, isize)> = HashSet::new();
    let mut min_steps: isize = isize::MAX;

    queue.push_back((start, 0));
    visited.insert(start);

    while let Some(((x, y), steps)) = queue.pop_front() {
        if (x, y) == end {
            min_steps = min_steps.min(steps);
        }

        for &(dx, dy) in &directions {
            let nx: isize = x + dx;
            let ny: isize = y + dy;

            if ny >= 0 && ny < grid.len() as isize && nx >= 0 && nx < grid[0].len() as isize && !grid[ny as usize][nx as usize] && visited.insert((nx, ny)) {
                queue.push_back(((nx, ny), steps + 1));
            }
        }
    }

    min_steps.to_string()
}

#[aoc(year = "2024", day = "day18", part = "part2")]
fn part2() -> String {
    let bytes: Vec<(isize, isize)> = parse_input();
    // let grid_size: isize = 6; // 6 for example, from 0..=grid_size.
    let grid_size: isize = 70;

    let start: (isize, isize) = (0, 0);
    let end: (isize, isize) = (grid_size, grid_size);

    let mut grid: Vec<Vec<bool>> = vec![vec![false; grid_size as usize + 1]; grid_size as usize + 1];
    let directions: [(isize, isize); 4] = [
        (0, -1), // ^
        (1, 0),  // >
        (0, 1),  // v
        (-1, 0), // <
    ];

    for (x, y) in bytes {
        // Let byte fall...
        grid[y as usize][x as usize] = true;

        // Do the magic.
        let mut queue: VecDeque<((isize, isize), isize)> = VecDeque::new();
        let mut visited: HashSet<(isize, isize)> = HashSet::new();

        queue.push_back((start, 0));
        visited.insert(start);

        let mut reached_end: bool = false;

        while let Some(((x, y), steps)) = queue.pop_front() {
            if (x, y) == end {
                reached_end = true;

                break;
            }

            for &(dx, dy) in &directions {
                let nx: isize = x + dx;
                let ny: isize = y + dy;

                if ny >= 0 && ny < grid.len() as isize && nx >= 0 && nx < grid[0].len() as isize && !grid[ny as usize][nx as usize] && visited.insert((nx, ny)) {
                    queue.push_back(((nx, ny), steps + 1));
                }
            }
        }

        if !reached_end {
            return format!("{},{}", x, y);
        }
    }

    unreachable!();
}

fn get_file_path() -> String {
    format!("./inputs/2024/day18{}.txt", if cfg!(test) { "-example" } else { "" })
}

fn parse_input() -> Vec<(isize, isize)> {
    let mut bytes: Vec<(isize, isize)> = Vec::new();

    if let Ok(lines) = read_lines(get_file_path()) {
        for value in lines.map_while(Result::ok) {
            let values: Vec<&str> = value.split(',').collect();
            bytes.push((values[0].parse::<isize>().unwrap(), values[1].parse::<isize>().unwrap()));
        }
    }

    bytes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(), todo!());
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), todo!());
    }
}
