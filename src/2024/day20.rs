// DO NOT EDIT - AOC
use aoc_proc_macros::aoc;
use aoc_shared_functions::{get_registry, read_lines};
// END DO NOT EDIT - AOC

use std::collections::{HashMap, HashSet, VecDeque};

fn bfs(map: &[Vec<char>], start: (isize, isize), end: (isize, isize)) -> HashMap<(isize, isize), isize> {
    let directions: [(isize, isize); 4] = [
        (0, -1), // ^
        (1, 0),  // >
        (0, 1),  // v
        (-1, 0), // <
    ];

    let mut queue: VecDeque<((isize, isize), isize)> = VecDeque::new();
    let mut visited: HashMap<(isize, isize), isize> = HashMap::new();

    queue.push_back((start, 0));
    visited.insert(start, 0);

    while let Some(((x, y), distance)) = queue.pop_front() {
        if (x, y) == end {
            // There is only one path from start to end.
            break;
        }

        for &(dx, dy) in &directions {
            let nx: isize = x + dx;
            let ny: isize = y + dy;

            if ny >= 0 && ny < map.len() as isize && nx >= 0 && nx < map[0].len() as isize && map[ny as usize][nx as usize] == '.' && !visited.contains_key(&(nx, ny)) {
                queue.push_back(((nx, ny), distance + 1));
                visited.insert((nx, ny), distance + 1);
            }
        }
    }

    visited
}

#[aoc(year = "2024", day = "day20", part = "part1")]
fn part1() -> String {
    let (map, start, end): (Vec<Vec<char>>, (isize, isize), (isize, isize)) = parse_input();
    let normal_path: HashMap<(isize, isize), isize> = bfs(&map, start, end);

    let directions: [(isize, isize); 4] = [
        (0, -1), // ^
        (1, 0),  // >
        (0, 1),  // v
        (-1, 0), // <
    ];

    let mut cheats: isize = 0;
    for (&(x, y), &normal_steps) in normal_path.iter() {
        for &(dx, dy) in &directions {
            let (nx, ny): (isize, isize) = (x + dx, y + dy);
            let (nnx, nny): (isize, isize) = (nx + dx, ny + dy);

            // If we have not covered a place in the grid it must be a wall, check whether we can
            // jump it (in two picoseconds; one for the wall and one for the next place). Cheats we
            // find must save 100 picoseconds or more.
            if !normal_path.contains_key(&(nx, ny)) && normal_path.contains_key(&(nnx, nny)) && normal_path.get(&(nnx, nny)).unwrap() - normal_steps >= 102 {
                cheats += 1;
            }
        }
    }

    cheats.to_string()
}

fn find_cheat_endpoints(normal_path: &HashMap<(isize, isize), isize>, start: (isize, isize)) -> HashSet<(isize, isize)> {
    let (x, y): (isize, isize) = start;
    let mut output: HashSet<(isize, isize)> = HashSet::new();

    for dx in -20isize..=20isize {
        let max_dy: isize = 20 - dx.abs();

        for dy in -max_dy..=max_dy {
            if normal_path.contains_key(&(x + dx, y + dy)) {
                output.insert((x + dx, y + dy));
            }
        }
    }

    output
}

fn manhattan_distance(start: (isize, isize), end: (isize, isize)) -> isize {
    (start.0 - end.0).abs() + (start.1 - end.1).abs()
}

#[aoc(year = "2024", day = "day20", part = "part2")]
fn part2() -> String {
    let (map, start, end): (Vec<Vec<char>>, (isize, isize), (isize, isize)) = parse_input();
    let normal_path: HashMap<(isize, isize), isize> = bfs(&map, start, end);

    let mut cheats: isize = 0;
    for (&coordinate, &normal_steps) in normal_path.iter() {
        let potential_cheat_reach: HashSet<(isize, isize)> = find_cheat_endpoints(&normal_path, coordinate);

        for &potential_cheat in potential_cheat_reach.iter() {
            if normal_path.get(&potential_cheat).unwrap() - normal_steps - manhattan_distance(coordinate, potential_cheat) >= 100 {
                cheats += 1;
            }
        }
    }

    cheats.to_string()
}

fn get_file_path() -> String {
    format!("./inputs/2024/day20{}.txt", if cfg!(test) { "-example" } else { "" })
}

fn parse_input() -> (Vec<Vec<char>>, (isize, isize), (isize, isize)) {
    let mut map: Vec<Vec<char>> = Vec::new();
    let mut start: (isize, isize) = (0, 0);
    let mut end: (isize, isize) = (0, 0);

    let mut y: isize = 0;

    if let Ok(lines) = read_lines(get_file_path()) {
        for value in lines.map_while(Result::ok) {
            let mut row: Vec<char> = value.chars().collect();

            if let Some(x) = row.iter().position(|&c| c == 'S') {
                start = (x as isize, y);
                row[x] = '.';
            }

            if let Some(x) = row.iter().position(|&c| c == 'E') {
                end = (x as isize, y);
                row[x] = '.';
            }

            map.push(row);

            y += 1;
        }
    }

    (map, start, end)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(), "");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), todo!());
    }
}
