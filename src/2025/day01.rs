// DO NOT EDIT - AOC
use aoc_proc_macros::aoc;
use aoc_shared_functions::{get_registry, read_lines};
// END DO NOT EDIT - AOC

enum Direction {
    Left,
    Right,
}

#[aoc(year = "2025", day = "day01", part = "part1")]
fn part1() -> String {
    let rotations: Vec<(Direction, i32)> = parse_input();
    let mut position: i32 = 50;
    let mut count_zero: i32 = 0;

    for (direction, distance) in rotations {
        position = match direction {
            Direction::Left => (position - distance).rem_euclid(100),
            Direction::Right => (position + distance).rem_euclid(100),
        };

        if position == 0 {
            count_zero += 1;
        }
    }

    count_zero.to_string()
}

#[aoc(year = "2025", day = "day01", part = "part2")]
fn part2() -> String {
    let rotations: Vec<(Direction, i32)> = parse_input();
    let mut position: i32 = 50;
    let mut count_zero: i32 = 0;

    for (direction, distance) in rotations {
        count_zero += match direction {
            Direction::Right => {
                let mut steps_to_next_zero: i32 = (100 - (position.rem_euclid(100))).rem_euclid(100);
                if steps_to_next_zero == 0 {
                    steps_to_next_zero = 100;
                }

                if steps_to_next_zero > distance {
                    0
                } else {
                    1 + (distance - steps_to_next_zero) / 100
                }
            }
            Direction::Left => {
                let mut steps_to_next_zero: i32 = position.rem_euclid(100);
                if steps_to_next_zero == 0 {
                    steps_to_next_zero = 100;
                }

                if steps_to_next_zero > distance {
                    0
                } else {
                    1 + (distance - steps_to_next_zero) / 100
                }
            }
        };

        position = match direction {
            Direction::Left => (position - distance).rem_euclid(100),
            Direction::Right => (position + distance).rem_euclid(100),
        };
    }

    count_zero.to_string()
}

fn get_file_path() -> String {
    format!("./inputs/2025/day01{}.txt", if cfg!(test) { "-example" } else { "" })
}

fn parse_input() -> Vec<(Direction, i32)> {
    let mut rotations = Vec::new();

    if let Ok(lines) = read_lines(get_file_path()) {
        for value in lines.map_while(Result::ok) {
            let (dir_char, dist_str) = value.split_at(1);
            let direction = match dir_char {
                "L" => Direction::Left,
                "R" => Direction::Right,
                _ => panic!("Invalid direction in input"),
            };
            let distance: i32 = dist_str.parse().expect("Invalid number");

            rotations.push((direction, distance));
        }
    }

    rotations
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(), "3");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), "6");
    }
}
