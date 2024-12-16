// DO NOT EDIT - AOC
use aoc_proc_macros::aoc;
use aoc_shared_functions::{get_registry, read_lines};
// END DO NOT EDIT - AOC

use std::collections::{BinaryHeap, HashMap, HashSet};
use std::cmp::Ordering;

#[derive(Eq, PartialEq)]
struct State {
    position: (isize, isize, isize),
    score: isize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.score.cmp(&self.score) // Not `self.score.cmp(other.score)` as we need min-heap!
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn find_best_score(map: &[Vec<char>], start: (isize, isize, isize), end: (isize, isize), part2: bool) -> isize {
    let directions: [(isize, isize); 4] = [
        (1, 0),  // >
        (0, 1),  // v
        (-1, 0), // <
        (0, -1), // ^
    ];
    let mut queue: BinaryHeap<State> = BinaryHeap::new();
    let mut visited: HashMap<(isize, isize, isize), isize> = HashMap::new();
    let mut linked_visits: HashMap<(isize, isize, isize), (isize, isize, isize)> = HashMap::new();
    let mut best_score = isize::MAX;

    queue.push(State { position: start, score: 0 });
    visited.insert(start, 0);

    while let Some(State { position: (x, y, current_direction), score }) = queue.pop() {
        if (x, y) == end {
            if score > best_score {
                break;
            }

            best_score = score;
        }

        for &new_direction in &[(current_direction + 3) % 4, (current_direction + 1) % 4] {
            let next_position: (isize, isize, isize) = (x, y, new_direction);
            let new_score: isize = score + 1000;

            if visited.contains_key(&next_position) && *visited.get(&next_position).unwrap() < new_score {
                continue;
            }

            visited.insert(next_position, new_score);
            linked_visits.insert(next_position, (x, y, current_direction));
            queue.push(State { position: next_position, score: new_score });
        }

        let (dx, dy): (isize, isize) = directions[current_direction as usize];
        let next_position: (isize, isize, isize) = (x + dx, y + dy, current_direction);
        let new_score: isize = score + 1;

        if map[next_position.1 as usize][next_position.0 as usize] == '#' {
            continue;
        }

        if visited.contains_key(&next_position) && *visited.get(&next_position).unwrap() <= new_score {
            continue;
        }

        visited.insert(next_position, new_score);
        linked_visits.insert(next_position, (x, y, current_direction));
        queue.push(State { position: next_position, score: new_score });
    }

    if part2 {
        let mut best_path_tiles: HashSet<(isize, isize)> = HashSet::new();
        let mut current: (isize, isize, isize) = (end.0, end.1, 0);

        while let Some(&previous) = linked_visits.get(&current) {
            best_path_tiles.insert((current.0, current.1));
            current = previous;
        }

        best_path_tiles.insert((start.0, start.1));

        best_path_tiles.len() as isize
    } else {
        best_score
    }
}

#[aoc(year = "2024", day = "day16", part = "part1")]
fn part1() -> String {
    // Attempts: 103496
    let (map, start, end): (Vec<Vec<char>>, (isize, isize, isize), (isize, isize)) = parse_input();
    let best_score: isize = find_best_score(&map, start, end, false);

    best_score.to_string()
}

#[aoc(year = "2024", day = "day16", part = "part2")]
fn part2() -> String {
    let (map, start, end): (Vec<Vec<char>>, (isize, isize, isize), (isize, isize)) = parse_input();
    let best_score: isize = find_best_score(&map, start, end, true);

    best_score.to_string()
}

fn get_file_path() -> String {
    format!("./inputs/2024/day16{}.txt", if cfg!(test) { "-example2" } else { "" })
}

fn parse_input() -> (Vec<Vec<char>>, (isize, isize, isize), (isize, isize)) {
    let mut map: Vec<Vec<char>> = Vec::new();
    let mut start: (isize, isize, isize) = (0, 0, 0);
    let mut end: (isize, isize) = (0, 0);

    let mut y: isize = 0;

    if let Ok(lines) = read_lines(get_file_path()) {
        for value in lines.map_while(Result::ok) {
            let mut row: Vec<char> = value.chars().collect();

            if let Some(x) = row.iter().position(|&c| c == 'S') {
                start = (x as isize, y, 0);
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
        assert_eq!(part1(), "7036");
    }

    // #[test]
    // fn test_part1_2() {
    //     assert_eq!(part1(), "11048");
    // }


    #[test]
    fn test_part2() {
        assert_eq!(part2(), "45");
    }

    // #[test]
    // fn test_part2_2() {
    //     assert_eq!(part2(), "64");
    // }
}
