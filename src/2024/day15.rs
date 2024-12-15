// DO NOT EDIT - AOC
use aoc_proc_macros::aoc;
use aoc_shared_functions::{get_registry, read_lines};
// END DO NOT EDIT - AOC

use std::collections::{HashMap, HashSet};

fn move_robot(map: &mut [Vec<char>], initial_position: &mut (isize, isize), current_move: char) {
    let (dx, dy) = match current_move {
        '^' => (0, -1), // ^
        '>' => (1, 0),  // >
        'v' => (0, 1),  // v
        '<' => (-1, 0), // <
        _ => unreachable!(),
    };

    let (x, y) = *initial_position;
    let (mut nx, mut ny): (isize, isize) = (x + dx, y + dy);

    // If the next step is a wall, we can NEVER perform that step.
    if map[ny as usize][nx as usize] == '#' {
        // println!("cannot move, wall");
        return;
    }

    // If the next step is empty, we can ALWAYS perform that step.
    if map[ny as usize][nx as usize] == '.' {
        // println!("can move without pushing");
        *initial_position = (nx, ny);

        return;
    }

    // Count the number of boxes in the current direction and stop after detecting a new box after
    // an empty space.
    let mut boxes_in_current_direction: isize = 0;
    let mut empty_spaces_in_current_direction: isize = 0;
    let mut seen_empty_space: bool = false;

    while map[ny as usize][nx as usize] == 'O' || map[ny as usize][nx as usize] == '.' {
        if map[ny as usize][nx as usize] == 'O' {
            if seen_empty_space {
                break;
            }

            boxes_in_current_direction += 1;
        } else if map[ny as usize][nx as usize] == '.' {
            seen_empty_space = true;
            empty_spaces_in_current_direction += 1;
        }

        nx += dx;
        ny += dy;
    }

    // println!("found {} boxes and {} empty spaces", boxes_in_current_direction, empty_spaces_in_current_direction);

    // Check that we have enough empty space to make the move.
    if empty_spaces_in_current_direction == 0 {
        // println!("not enough space to push all boxes");
        return;
    }

    // Reset next step and start moving boxes.
    (nx, ny) = (x + dx, y + dy);
    map[ny as usize][nx as usize] = '.';
    *initial_position = (nx, ny);

    for _ in 0..boxes_in_current_direction {
        nx += dx;
        ny += dy;

        map[ny as usize][nx as usize] = 'O';
    }
}

fn sum_gps(map: &[Vec<char>], box_character: char) -> isize{
    let mut sum: isize = 0;

    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] == box_character {
                sum += 100 * y as isize + x as isize;
            }
        }
    }

    sum
}

#[allow(dead_code)]
fn print_map(map: &[Vec<char>]) {
    for row in map.iter() {
        for &cell in row.iter() {
            print!("{}", cell);
        }

        println!();
    }

    println!();
}

#[aoc(year = "2024", day = "day15", part = "part1")]
fn part1() -> String {
    let (mut map, mut initial_position, moves): (Vec<Vec<char>>, (isize, isize), Vec<char>) = parse_input();

    // `move` is a reserved keyword :(
    for mv in moves.iter() {
        // println!("Before {}:", mv);
        // print_map(&map);
        move_robot(&mut map, &mut initial_position, *mv);
        // println!("After {}:", mv);
        // print_map(&map);
    }

    let gps: isize = sum_gps(&map, 'O');
    gps.to_string()
}

fn enlarge_map(map: &[Vec<char>]) -> Vec<Vec<char>> {
    let mut enlarged_map: Vec<Vec<char>> = Vec::new();

    for row in map.iter() {
        let mut new_row: Vec<char> = Vec::new();

        for &tile in row.iter() {
            match tile {
                '#' => {
                    new_row.push('#');
                    new_row.push('#');
                },
                'O' => {
                    new_row.push('[');
                    new_row.push(']');
                },
                '.' => {
                    new_row.push('.');
                    new_row.push('.');
                },
                '@' => {
                    new_row.push('@');
                    new_row.push('.');
                },
                _ => unreachable!(),
            }
        }

        enlarged_map.push(new_row);
    }

    enlarged_map
}

fn move_robot2(map: &mut [Vec<char>], initial_position: &mut (isize, isize), current_move: char) {
    let (dx, dy) = match current_move {
        '^' => (0, -1), // ^
        '>' => (1, 0),  // >
        'v' => (0, 1),  // v
        '<' => (-1, 0), // <
        _ => unreachable!(),
    };

    let (x, y) = *initial_position;
    let (nx, ny): (isize, isize) = (x + dx, y + dy);

    // If the next step is a wall, we can NEVER perform that step.
    if map[ny as usize][nx as usize] == '#' {
        // println!("cannot move, wall");
        return;
    }

    // If the next step is empty, we can ALWAYS perform that step.
    if map[ny as usize][nx as usize] == '.' {
        // println!("can move without pushing");
        *initial_position = (nx, ny);

        return;
    }

    // We must have hit a box...
    let mut boxes: HashSet<(isize, isize)> = HashSet::new();
    get_connected_boxes(map, &mut boxes, initial_position, &(dx, dy));

    let (nx, ny): (isize, isize) = (initial_position.0 + dx, initial_position.1 + dy);
    if !boxes.is_empty() {
        if !can_move_boxes(map, &boxes, &(dx, dy)) {
            return;
        }

        move_boxes(map, &boxes, &(dx, dy));
    }

    *initial_position = (nx, ny);
}

fn get_connected_boxes(map: &mut [Vec<char>], boxes: &mut HashSet<(isize, isize)>, current_position: &(isize, isize), current_direction: &(isize, isize)) {
    let (nx, ny): (isize, isize) = (current_position.0 + current_direction.0, current_position.1 + current_direction.1);

    let adjacent_boxes: Vec<(isize, isize)> = match map[ny as usize][nx as usize] {
        '[' => vec![(nx, ny), (nx + 1, ny)],
        ']' => vec![(nx, ny), (nx - 1, ny)],
        _ => Vec::new(),
    };

    for adjacent_box in adjacent_boxes {
        if !boxes.contains(&adjacent_box) {
            boxes.insert(adjacent_box);
            get_connected_boxes(map, boxes, &adjacent_box, &current_direction);
        }
    }
}

fn can_move_boxes(map: &mut [Vec<char>], boxes: &HashSet<(isize, isize)>, current_direction: &(isize, isize)) -> bool {
    // `box` is also a reserved keyword :(
    for bx in boxes {
        let (nx, ny): (isize, isize) = (bx.0 + current_direction.0, bx.1 + current_direction.1);

        if map[ny as usize][nx as usize] == '#' {
            return false;
        }
    }

    true
}

fn move_boxes(map: &mut [Vec<char>], boxes: &HashSet<(isize, isize)>, current_direction: &(isize, isize)) {
    let mut current_box_values = HashMap::new();

    // `box` is still a reserved keyword :(
    for bx in boxes {
        current_box_values.insert((bx.0, bx.1), map[bx.1 as usize][bx.0 as usize]);
    }

    // `box` is still a reserved keyword :(
    for bx in boxes {
        let (nx, ny): (isize, isize) = (bx.0 + current_direction.0, bx.1 + current_direction.1);

        // Copy to new location.
        map[ny as usize][nx as usize] = *current_box_values.get(&(bx.0, bx.1)).unwrap();

        // Clean up the previous location.
        let (px, py): (isize, isize) = (bx.0 - current_direction.0, bx.1 - current_direction.1);
        if current_box_values.contains_key(&(px, py)) {
            map[bx.1 as usize][bx.0 as usize] = *current_box_values.get(&(px, py)).unwrap();
        } else {
            map[bx.1 as usize][bx.0 as usize] = '.';
        }
    }
}

#[aoc(year = "2024", day = "day15", part = "part2")]
fn part2() -> String {
    let (map, mut initial_position, moves): (Vec<Vec<char>>, (isize, isize), Vec<char>) = parse_input();
    let mut enlarged_map = enlarge_map(&map);
    initial_position = (initial_position.0 * 2, initial_position.1);

    // `move` is a reserved keyword :(
    for mv in moves.iter() {
        // println!("Before {}:", mv);
        // print_map(&enlarged_map);
        move_robot2(&mut enlarged_map, &mut initial_position, *mv);
        // println!("After {}:", mv);
        // print_map(&enlarged_map);
    }

    let gps: isize = sum_gps(&enlarged_map, '[');
    gps.to_string()
}

fn get_file_path() -> String {
    format!("./inputs/2024/day15{}.txt", if cfg!(test) { "-example" } else { "" })
}

fn parse_input() -> (Vec<Vec<char>>, (isize, isize), Vec<char>) {
    let mut map: Vec<Vec<char>> = Vec::new();
    let mut moves: Vec<char> = Vec::new();
    let mut initial_position: (isize, isize) = (0, 0);

    let mut r: isize = 0;
    let mut parse_moves = false;

    if let Ok(lines) = read_lines(get_file_path()) {
        for value in lines.map_while(Result::ok) {
            if parse_moves {
                moves.extend(value.chars());
            } else if value.is_empty() {
                parse_moves = true;
            } else {
                let mut row: Vec<char> = value.chars().collect();

                if let Some(c) = row.iter().position(|&c| c == '@') {
                    initial_position = (c as isize, r);

                    // Hide robot from map, makes movement checking easier.
                    row[c] = '.';
                }

                map.push(row);
                r += 1;
            }
        }
    }

    (map, initial_position, moves)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(), "10092");
    }

    // #[test]
    // fn test_part1_small() {
    //     assert_eq!(part1(), "2028");
    // }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), "9021");
    }
}

// Another attempt at doing part 2 using the implementation of part 1, failed due to the complexity
// of having to add logic for pushing triangles:
#[allow(dead_code)]
fn move_robot_part2(map: &mut [Vec<char>], initial_position: &mut (isize, isize), current_move: char, box_character: char) {
    let (dx, dy) = match current_move {
        '^' => (0, -1), // ^
        '>' => (1, 0),  // >
        'v' => (0, 1),  // v
        '<' => (-1, 0), // <
        _ => unreachable!(),
    };

    let (x, y) = *initial_position;
    let (mut nx, mut ny): (isize, isize) = (x + dx, y + dy);

    // If the next step is a wall, we can NEVER perform that step.
    if map[ny as usize][nx as usize] == '#' {
        println!("cannot move, wall");
        return;
    }

    // If the next step is empty, we can ALWAYS perform that step.
    if map[ny as usize][nx as usize] == '.' {
        println!("can move without pushing");
        *initial_position = (nx, ny);

        return;
    }

    // Count the number of boxes in the current direction and stop after detecting a new box after
    // an empty space.
    let mut boxes_in_current_direction: isize = 0;
    let mut empty_spaces_in_current_direction: isize = 0;
    let mut seen_empty_space: bool = false;

    while map[ny as usize][nx as usize] == box_character || (box_character == '[' && map[ny as usize][nx as usize] == ']')
        || map[ny as usize][nx as usize] == '.' {
        if map[ny as usize][nx as usize] == box_character {
            if seen_empty_space {
                break;
            }

            boxes_in_current_direction += 1;
        } else if map[ny as usize][nx as usize] == '.' {
            seen_empty_space = true;
            empty_spaces_in_current_direction += 1;
        }

        nx += dx;
        ny += dy;
    }

    println!("found {} boxes and {} empty spaces", boxes_in_current_direction, empty_spaces_in_current_direction);

    // Check that we have enough empty space to make the move.
    if empty_spaces_in_current_direction == 0 {
        println!("not enough space to push all boxes");
        return;
    }

    // Reset next step and start moving boxes.
    (nx, ny) = (x + dx, y + dy);
    map[ny as usize][nx as usize] = '.';
    *initial_position = (nx, ny);

    for _ in 0..boxes_in_current_direction {
        // For part 2:
        let original_box: char = map[ny as usize][nx as usize];

        nx += dx;
        ny += dy;

        if box_character == 'O' {
            map[ny as usize][nx as usize] = 'O';
        } else {
            if original_box == box_character {
                map[ny as usize][nx as usize] = '[';
                map[ny as usize][(nx + 1) as usize] = ']';
            } else {
                map[ny as usize][nx as usize] = ']';
                map[ny as usize][(nx - 1) as usize] = '[';
            }

            match current_move {
                '^' | 'v' => {
                    if original_box == box_character {
                        map[(ny - dy) as usize][(nx - dx + 1) as usize] = '.';
                    } else {
                        map[(ny - dy) as usize][(nx - dx - 1) as usize] = '.';
                    }
                },
                '<' | '>' => {
                    nx += dx + dx;
                    ny += dy + dy;
                },
                _ => unreachable!(),
            }
        }
    }
}