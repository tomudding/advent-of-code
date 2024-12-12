use std::collections::HashSet;
// DO NOT EDIT - AOC
use aoc_proc_macros::aoc;
use aoc_shared_functions::{get_registry, read_lines};
// END DO NOT EDIT - AOC

#[aoc(year = "2024", day = "day12", part = "part1")]
fn part1() -> String {
    let map: Vec<Vec<char>> = parse_input();

    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut total_price: isize = 0;

    let directions: [(isize, isize); 4] = [
        (0, -1), // ^
        (1, 0),  // >
        (0, 1),  // v
        (-1, 0), // <
    ];

    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if !visited.contains(&(x, y)) {
                let region_type: char = map[y][x];
                let mut stack: Vec<(usize, usize)> = vec![(x, y)];
                let mut region_area: isize = 0;
                let mut region_perimeter: isize = 0;

                while let Some((cx, cy)) = stack.pop() {
                    if visited.contains(&(cx, cy)) || map[cy][cx] != region_type {
                        continue;
                    }

                    visited.insert((cx, cy));
                    region_area += 1;

                    for &(dx, dy) in &directions {
                        let nx: isize = cx as isize + dx;
                        let ny: isize = cy as isize + dy;

                        if ny < 0 || nx < 0 || ny >= map.len() as isize || nx >= map[0].len() as isize || map[ny as usize][nx as usize] != region_type {
                            region_perimeter += 1;
                        } else if !visited.contains(&(nx as usize, ny as usize)) {
                            stack.push((nx as usize, ny as usize));
                        }
                    }
                }

                total_price += region_area * region_perimeter;
            }
        }
    }

    total_price.to_string()
}

fn map_region(map: &[Vec<char>], x: usize, y: usize, directions: &[(isize, isize)], visited: &mut HashSet<(usize, usize)>) -> HashSet<(usize, usize)> {
    let mut region: HashSet<(usize, usize)> = HashSet::new();
    let mut stack: Vec<(usize, usize)> = Vec::new();
    let region_type: char = map[y][x];

    stack.push((x, y));
    region.insert((x, y));

    while let Some((cx, cy)) = stack.pop() {
        for &(dx, dy) in directions {
            let nx: isize = cx as isize + dx;
            let ny: isize = cy as isize + dy;

            if ny >= 0 && nx >= 0 && ny < map.len() as isize && nx < map[0].len() as isize && map[ny as usize][nx as usize] == region_type && !visited.contains(&(nx as usize, ny as usize)) {
                visited.insert((nx as usize, ny as usize));
                stack.push((nx as usize, ny as usize));
                region.insert((nx as usize, ny as usize));
            }
        }
    }

    region
}

fn count_sides(region: &HashSet<(usize, usize)>, directions: &[(isize, isize)]) -> usize {
    let mut visited: HashSet<(usize, usize, isize, isize)> = HashSet::new();
    let mut sides: usize = 0;

    for &(x, y) in region {
        for &(dx, dy) in directions {
            let nx: isize = x as isize + dx;
            let ny: isize = y as isize + dy;

            if !region.contains(&(nx as usize, ny as usize)) {
                let mut nnx: isize = x as isize;
                let mut nny: isize = y as isize;

                while region.contains(&((nnx + dy) as usize, (nny + dx) as usize)) && !region.contains(&((nnx + dx) as usize, (nny + dy) as usize)) {
                    nnx += dy;
                    nny += dx;
                }

                if !visited.contains(&(nnx as usize, nny as usize, dx, dy)) {
                    visited.insert((nnx as usize, nny as usize, dx, dy));
                    sides += 1;
                }
            }
        }
    }
    sides
}

#[aoc(year = "2024", day = "day12", part = "part2")]
fn part2() -> String {
    let map: Vec<Vec<char>> = parse_input();

    let mut regions: Vec<HashSet<(usize, usize)>> = Vec::new();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut total_price: usize = 0;

    let directions: [(isize, isize); 4] = [
        (0, -1), // ^
        (1, 0),  // >
        (0, 1),  // v
        (-1, 0), // <
    ];

    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if !visited.contains(&(x, y)) {
                regions.push(map_region(&map, x, y, &directions, &mut visited));
            }
        }
    }

    for region in &regions {
        total_price += region.len() * count_sides(region, &directions);
    }

    total_price.to_string()
}

fn get_file_path() -> String {
    format!("./inputs/2024/day12{}.txt", if cfg!(test) { "-example" } else { "" })
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
        assert_eq!(part1(), "1930");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), "1206");
    }
}
