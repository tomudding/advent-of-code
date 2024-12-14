// DO NOT EDIT - AOC
use aoc_proc_macros::aoc;
use aoc_shared_functions::{get_registry, read_lines};
// END DO NOT EDIT - AOC

use std::collections::HashSet;

#[aoc(year = "2024", day = "day14", part = "part1")]
fn part1() -> String {
    let robots: Vec<((isize, isize), (isize, isize))> = parse_input();
    // Let's make the example smaller, such that you have to adjust your code after testing...
    let width: isize = 101;
    let height: isize = 103;

    let positions: Vec<(isize, isize)> = robots.into_iter().map(|(cpos, cvel)| {
        // Run for 100 seconds.
        let mut x: isize = (cpos.0 + cvel.0 * 100) % width;
        let mut y: isize = (cpos.1 + cvel.1 * 100) % height;

        // Wrap around grid if necessary.
        if x < 0 {
            x += width
        }

        if y < 0 {
            y += height;
        }

        (x, y)
    }).collect();

    let middle_x = width / 2;
    let middle_y = height / 2;

    let mut quadrants: Vec<usize> = vec![0; 4];
    for &(x, y) in &positions {
        // Robots in the middle do not count.
        if x == middle_x || y == middle_y {
            continue;
        }

        if x < middle_x && y < middle_y {
            quadrants[0] += 1;
        } else if x > middle_x && y < middle_y {
            quadrants[1] += 1;
        } else if x < middle_x && y > middle_y {
            quadrants[2] += 1;
        } else {
            quadrants[3] += 1;
        }
    }

    let safety_factor: usize = quadrants.iter().product::<usize>();
    safety_factor.to_string()
}

#[aoc(year = "2024", day = "day14", part = "part2")]
fn part2() -> String {
    let robots: Vec<((isize, isize), (isize, isize))> = parse_input();
    let width: isize = 101;
    let height: isize = 103;

    let mut seconds: isize = 0;

    // A wild guess, but if all robots have a unique position they must be displaying a
    // Christmas tree, no?
    // This part is so unclear, no idea what they expect from us...
    loop {
        let mut visited: HashSet<(isize, isize)> = HashSet::new();

        seconds += 1;

        // Move all robots for a `seconds` period of time.
        for &(cpos, cvel) in &robots {
            let mut x: isize = (cpos.0 + cvel.0 * seconds) % width;
            let mut y: isize = (cpos.1 + cvel.1 * seconds) % height;

            // Wrap around grid if necessary.
            if x < 0 {
                x += width
            }

            if y < 0 {
                y += height;
            }

            visited.insert((x, y));
        }

        // Check number of unique locations against number of robots.
        if visited.len() == robots.len() {
            break;
        }
    }

    seconds.to_string()
}

fn get_file_path() -> String {
    format!("./inputs/2024/day14{}.txt", if cfg!(test) { "-example" } else { "" })
}

fn parse_input() -> Vec<((isize, isize), (isize, isize))> {
    let mut robots: Vec<((isize, isize), (isize, isize))> = Vec::new();

    if let Ok(lines) = read_lines(get_file_path()) {
        for value in lines.map_while(Result::ok) {
            let parts: Vec<&str> = value.split_whitespace().collect();
            let positions: &str = parts[0].split('=').nth(1).unwrap();
            let velocities: &str = parts[1].split('=').nth(1).unwrap();

            let real_positions: Vec<&str> = positions.split(',').map(|s| s.trim()).collect();
            let real_velocities: Vec<&str> = velocities.split(',').map(|s| s.trim()).collect();

            robots.push((
                (real_positions[0].parse().unwrap(), real_positions[1].parse().unwrap()),
                (real_velocities[0].parse().unwrap(), real_velocities[1].parse().unwrap()),
            ));
        }
    }

    robots
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(), "12");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), todo!());
    }
}
