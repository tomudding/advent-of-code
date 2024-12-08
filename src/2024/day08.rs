// DO NOT EDIT - AOC
use aoc_proc_macros::aoc;
use aoc_shared_functions::{get_registry, read_lines};
// END DO NOT EDIT - AOC

use std::collections::HashSet;

fn get_antinodes(antennas: &[(isize, isize, char)], max_x: isize, max_y: isize, part2: bool) -> HashSet<(isize, isize)> {
    let mut antinodes: HashSet<(isize, isize)> = HashSet::new();

    for (i, &(x1, y1, frequency_antenna_1)) in antennas.iter().enumerate() {
        for &(x2, y2, frequency_antenna_2) in antennas.iter().skip(i + 1) {
            if frequency_antenna_1 == frequency_antenna_2 {
                let dx: isize = x2 - x1;
                let dy: isize = y2 - y1;

                if !part2 {
                    let antinode_1: (isize, isize) = (x1 - dx, y1 - dy);
                    let antinode_2: (isize, isize) = (x2 + dx, y2 + dy);

                    if within_bounds(antinode_1, max_x, max_y) {
                        antinodes.insert(antinode_1);
                    }

                    if within_bounds(antinode_2, max_x, max_y) {
                        antinodes.insert(antinode_2);
                    }
                } else {
                    let mut antinode_1: (isize, isize) = (x1, y1);
                    let mut antinode_2: (isize, isize) = (x2, y2);

                    while within_bounds(antinode_1, max_x, max_y) || within_bounds(antinode_2, max_x, max_y) {
                        if within_bounds(antinode_1, max_x, max_y) {
                            antinodes.insert(antinode_1);
                        }

                        if within_bounds(antinode_2, max_x, max_y) {
                            antinodes.insert(antinode_2);
                        }

                        antinode_1.0 -= dx;
                        antinode_1.1 -= dy;
                        antinode_2.0 += dx;
                        antinode_2.1 += dy;
                    }
                }
            }
        }
    }

    antinodes
}

fn within_bounds(position: (isize, isize), max_x: isize, max_y: isize) -> bool {
    let (x, y): (isize, isize) = position;

    x >= 0 && x <= max_x && y >= 0 && y <= max_y
}

#[aoc(year = "2024", day = "day08", part = "part1")]
fn part1() -> String {
    // Incorrect attempts: 333 (no higher/lower), caused by max_y being 1 too high.
    let (antennas, (max_x, max_y)): (Vec<(isize, isize, char)>, (isize, isize)) = parse_input();
    let antinodes: HashSet<(isize, isize)> = get_antinodes(&antennas, max_x, max_y, false);

    antinodes.len().to_string()
}

#[aoc(year = "2024", day = "day08", part = "part2")]
fn part2() -> String {
    let (antennas, (max_x, max_y)): (Vec<(isize, isize, char)>, (isize, isize)) = parse_input();
    let antinodes: HashSet<(isize, isize)> = get_antinodes(&antennas, max_x, max_y, true);

    antinodes.len().to_string()
}

fn get_file_path() -> String {
    format!("./inputs/2024/day08{}.txt", if cfg!(test) { "-example" } else { "" })
}

fn parse_input() -> (Vec<(isize, isize, char)>, (isize, isize)) {
    // Do not use a grid this time, it is frustrating to work with.
    let mut antennas: Vec<(isize, isize, char)> = Vec::new();
    let mut max_x: isize = 0;
    let mut y: isize = 0;

    if let Ok(lines) = read_lines(get_file_path()) {
        for value in lines.map_while(Result::ok) {
            for (x, ch) in value.chars().enumerate() {
                max_x = x as isize;

                if ch == '.' {
                    continue;
                }

                antennas.push((x as isize, y, ch));
            }

            y += 1;
        }
    }

    // Did you know that this - 1 for `y` IS EXTREMELY IMPORTANT!
    (antennas, (max_x, y - 1))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(), "14");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), "34");
    }
}
