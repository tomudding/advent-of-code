// DO NOT EDIT - AOC
use aoc_proc_macros::aoc;
use aoc_shared_functions::{get_registry, read_lines};
// END DO NOT EDIT - AOC

use std::collections::HashSet;
use std::collections::VecDeque;

fn calculate_trailheads(map: &[Vec<i8>], trailheads: &[(usize, usize)], part2: bool) -> usize {
    let mut total_score: usize = 0;

    for &(x, y) in trailheads.iter() {
        if part2 {
            total_score += count_distinct_trails(map, x, y);
        } else {
            total_score += count_reachable_nines(map, x, y);
        }
    }

    total_score
}

fn count_reachable_nines(map: &[Vec<i8>], x: usize, y: usize) -> usize {
    let directions: [(i8, i8); 4] = [
        (0, -1), // ^
        (1, 0),  // >
        (0, 1),  // v
        (-1, 0), // <
    ];

    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut queue: VecDeque<(usize, usize, i8)> = VecDeque::new();
    let mut score: usize = 0;

    queue.push_back((x, y, 0));

    while let Some((cx, cy, height)) = queue.pop_front() {
        if visited.contains(&(cx, cy)) || map[cy][cx] != height {
            continue;
        }

        visited.insert((cx, cy));

        if map[cy][cx] == 9 {
            score += 1;

            continue;
        }

        for &(dx, dy) in &directions {
            let nx: i8 = cx as i8 + dx;
            let ny: i8 = cy as i8 + dy;

            if ny >= 0 && ny < map.len() as i8 && nx >= 0 && nx < map[0].len() as i8 {
                queue.push_back((nx as usize, ny as usize, height + 1));
            }
        }
    }

    score
}

fn count_distinct_trails(map: &[Vec<i8>], x: usize, y: usize) -> usize {
    let directions: [(i8, i8); 4] = [
        (0, -1), // ^
        (1, 0),  // >
        (0, 1),  // v
        (-1, 0), // <
    ];

    let mut distinct_trails: HashSet<Vec<(usize, usize)>> = HashSet::new();
    let mut queue: VecDeque<(usize, usize, Vec<(usize, usize)>, i8)> = VecDeque::new();

    queue.push_back((x, y, vec![(x, y)], 0));

    while let Some((cx, cy, path, height)) = queue.pop_front() {
        if map[cy][cx] != height {
            continue;
        }

        if map[cy][cx] == 9 {
            distinct_trails.insert(path.clone());

            continue;
        }

        for &(dx, dy) in &directions {
            let nx: i8 = cx as i8 + dx;
            let ny: i8 = cy as i8 + dy;

            if ny >= 0 && ny < map.len() as i8 && nx >= 0 && nx < map[0].len() as i8 {
                let mut new_path = path.clone();

                new_path.push((nx as usize, ny as usize));
                queue.push_back((nx as usize, ny as usize, new_path, height + 1));
            }
        }
    }

    distinct_trails.len()
}


#[allow(dead_code)]
fn find_nines(map: &[Vec<i8>], x: usize, y: usize) -> usize {
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut score: usize = 0;

    // Dit werkt dus niet zonder queue, tijd om te herschrijven :/ (ben ik al wakker?)
    if dfs(map, &mut visited, x, y, 0) {
        score += 1;
    }

    // println!("----------------------");
    // for (ry, row) in visited.iter().enumerate() {
    //     for (rx, col) in row.iter().enumerate() {
    //         if rx == x.try_into().unwrap() && ry == y.try_into().unwrap() {
    //             print!("S")
    //         } else if *col {
    //             print!("X")
    //         } else {
    //             print!(".")
    //         }
    //     }
    //     println!("");
    // }

    score
}

#[allow(dead_code)]
fn dfs(map: &[Vec<i8>], visited: &mut HashSet<(usize, usize)>, x: usize, y: usize, height: i8) -> bool {
    if visited.contains(&(x, y)) || map[y][x] != height {
        return false;
    }

    visited.insert((x, y));

    // Check if end of trail.
    if map[y][x] == 9 {
        return true;
    }

    let directions: [(i8, i8); 4] = [
        (0, -1), // ^
        (1, 0),  // >
        (0, 1),  // v
        (-1, 0), // <
    ];

    for &(dx, dy) in &directions {
        let nx: i8 = x as i8 + dx;
        let ny: i8 = y as i8 + dy;

        if ny >= 0 && ny < map.len() as i8 && nx >= 0 && nx < map[0].len() as i8 {
            if dfs(map, visited, nx as usize, ny as usize, height + 1) {
                return true;
            }
        }
    }

    false
}

#[aoc(year = "2024", day = "day10", part = "part1")]
fn part1() -> String {
    let (map, trailheads): (Vec<Vec<i8>>, Vec<(usize, usize)>) = parse_input();
    let result: usize = calculate_trailheads(&map, &trailheads, false);

    result.to_string()
}

#[aoc(year = "2024", day = "day10", part = "part2")]
fn part2() -> String {
    let (map, trailheads): (Vec<Vec<i8>>, Vec<(usize, usize)>) = parse_input();
    let result: usize = calculate_trailheads(&map, &trailheads, true);

    result.to_string()
}

fn get_file_path() -> String {
    format!("./inputs/2024/day10{}.txt", if cfg!(test) { "-example" } else { "" })
}

fn parse_input() -> (Vec<Vec<i8>>, Vec<(usize, usize)>) {
    let mut map: Vec<Vec<i8>> = Vec::new();
    let mut trailheads: Vec<(usize, usize)> = Vec::new();
    let mut y: usize = 0;

    if let Ok(lines) = read_lines(get_file_path()) {
        for value in lines.map_while(Result::ok) {
            let row: Vec<i8> = value.chars().map(|c| c.to_digit(10).unwrap() as i8).collect();

            for (x, &height) in row.iter().enumerate() {
                if height == 0 {
                    trailheads.push((x, y));
                }
            }

            // Needs to go here to prevent a `.clone()` if it is above the for-loop.
            map.push(row);

            y += 1;
        }
    }

    (map, trailheads)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(), "36");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), "81");
    }
}
