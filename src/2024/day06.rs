// DO NOT EDIT - AOC
use aoc_function_registry::get_registry;
use aoc_proc_macros::aoc;
// END DO NOT EDIT - AOC
// DO NOT EDIT - DEFAULTS
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
// DO NOT EDIT - DEFAULTS

use std::collections::HashSet;

fn predict_guard_path(map: &[Vec<char>], initial_position: Option<(i32, i32)>) -> (HashSet<(i32, i32)>, bool) {
    // The possible directions, ordered such that you can always take a right turn.
    let directions: [(i32, i32); 4] = [
        (0, -1), // ^
        (1, 0),  // >
        (0, 1),  // v
        (-1, 0), // <
    ];

    let mut visited_positions: HashSet<(i32, i32)> = HashSet::new();
    let mut loop_detection: HashSet<((i32, i32), i32)> = HashSet::new();

    // We use the `Option` for `initial_position` to determine whether we are doing part 1 or 2.
    let mut current_position: (i32, i32) = initial_position.unwrap_or_else(|| get_start(map));
    let mut current_direction: i32 = 0;

    loop {
        visited_positions.insert(current_position);
        loop_detection.insert((current_position, current_direction));

        let (dx, dy) = directions[current_direction as usize];
        let new_position: (i32, i32) = (current_position.0 + dx, current_position.1 + dy);

        // Check if guard will leave the map.
        if new_position.0 < 0 || new_position.1 < 0 || new_position.0 >= map[0].len() as i32 || new_position.1 >= map.len() as i32 {
            break;
        }

        if map[new_position.1 as usize][new_position.0 as usize] == '#' || map[new_position.1 as usize][new_position.0 as usize] == 'O' {
            // Hit obstacle, turn right.
            current_direction = (current_direction + 1) % 4;
        } else {
            current_position = new_position;
            visited_positions.insert(current_position);
        }

        // `initial_position.is_some()` is a safety check against part 1, however, it does not appear
        // to be actually necessary (but better safe than sorry).
        if initial_position.is_some() && loop_detection.contains(&(current_position, current_direction)) {
            return (visited_positions, true);
        }
    }

    (visited_positions, false)
}

fn get_start(map: &[Vec<char>]) -> (i32, i32) {
    for (y, row) in map.iter().enumerate() {
        for (x, &c) in row.iter().enumerate() {
            if c == '^' {
                return (x as i32, y as i32);
            }
        }
    }

    panic!("No starting position of guard found...");
}

fn find_possible_obstructions(map: &[Vec<char>]) -> HashSet<(i32, i32)> {
    let mut possible_obstruction_positions: HashSet<(i32, i32)> = HashSet::new();
    let initial_position: (i32, i32) = get_start(map);

    // let total_positions: usize = map.len() * map[0].len() - 1;
    // let mut checked_positions: i32 = 0;

    // Place new obstruction 'O' where there is not yet a '#' (or it is the starting position).
    for y in 0..map.len() as i32 {
        for x in 0..map[0].len() as i32 {
            if (x, y) != initial_position && map[y as usize][x as usize] == '.' {
                let mut obstructed_map: Vec<Vec<char>> = map.to_vec();
                obstructed_map[y as usize][x as usize] = 'O';

                let (_, loop_detected): (HashSet<(i32, i32)>, bool) = predict_guard_path(&obstructed_map, Some(initial_position));

                // checked_positions += 1;
                // if checked_positions % 130 == 0 {
                //     println!("Progress: {:.0}%", (checked_positions as f64 / total_positions as f64) * 100.0);
                // }

                if loop_detected {
                    possible_obstruction_positions.insert((x, y));
                }
            }
        }
    }

    possible_obstruction_positions
}

#[aoc(year = "2024", day = "day06", part = "part1")]
fn part1() -> String {
    let map: Vec<Vec<char>> = parse_input();
    let (visited_positions, _): (HashSet<(i32, i32)>, bool) = predict_guard_path(&map, None);

    visited_positions.len().to_string()
}

#[aoc(year = "2024", day = "day06", part = "part2")]
fn part2() -> String {
    let map: Vec<Vec<char>> = parse_input();
    let possible_obstruction_positions = find_possible_obstructions(&map);

    possible_obstruction_positions.len().to_string()
}

fn get_file_path() -> String {
    format!("./inputs/2024/day06{}.txt", if cfg!(test) { "-example" } else { "" })
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse_input() -> Vec<Vec<char>> {
    let mut map: Vec<Vec<char>> = Vec::new();

    if let Ok(lines) = read_lines(get_file_path()) {
        for value in lines.map_while(Result::ok) {
            map.push(value.chars().collect());
        }
    }

    map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(), "41");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), "6");
    }
}
