// DO NOT EDIT - AOC
use aoc_proc_macros::aoc;
use aoc_shared_functions::{get_registry, read_lines};
// END DO NOT EDIT - AOC

use std::collections::HashMap;

#[aoc(year = "2025", day = "day09", part = "part1")]
fn part1() -> String {
    let points: Vec<(usize, usize)> = parse_input();
    let mut max_area: usize = 0;

    let n = points.len();
    for i in 0..n {
        let (x1, y1): (usize, usize) = points[i];

        for j in (i + 1)..n {
            let (x2, y2): (usize, usize) = points[j];
            let width: usize = (x1 as isize - x2 as isize).unsigned_abs() + 1;
            let height: usize = (y1 as isize - y2 as isize).unsigned_abs() + 1;
            let area = width * height;

            if area > max_area {
                max_area = area;
            }
        }
    }

    max_area.to_string()
}

#[aoc(year = "2025", day = "day09", part = "part2")]
fn part2() -> String {
    let points: Vec<(usize, usize)> = parse_input();
    let mut max_area: usize = 0;

    let mut rows: HashMap<usize, (usize, usize)> = HashMap::new();

    for segment in points.windows(2) {
        let (x1, y1): (usize, usize) = segment[0];
        let (x2, y2): (usize, usize) = segment[1];

        let left: usize = x1.min(x2);
        let right: usize = x1.max(x2);

        let update_row = |map: &mut HashMap<usize, (usize, usize)>, y: usize| {
            map.entry(y)
                .and_modify(|row| {
                    row.0 = row.0.min(left);
                    row.1 = row.1.max(right);
                })
                .or_insert((left, right));
        };

        if y1 == y2 {
            // Vertical line.
            update_row(&mut rows, y1);
        } else {
            // Block.
            let y_min: usize = y1.min(y2);
            let y_max: usize = y1.max(y2);

            // Inefficiently fill the block.
            for y in y_min..=y_max {
                update_row(&mut rows, y);
            }
        }
    }

    for (i, &(x1, y1)) in points.iter().enumerate() {
        for &(x2, y2) in points.iter().skip(i + 1) {
            let y_min: usize = y1.min(y2);
            let y_max: usize = y1.max(y2);

            // Check if the rectangle is invalid by searching for the first invalid row.
            let invalid: bool = (y_min..=y_max)
                .position(|y| {
                    match rows.get(&y) {
                        Some(&(left, right)) => {
                            let min_x: usize = x1.min(x2);
                            let max_x: usize = x1.max(x2);
                            !(left <= min_x && right >= max_x)
                        }
                        None => true,
                    }
                })
                .is_some();

            if !invalid {
                let width: usize = (x1.max(x2) as isize - x1.min(x2) as isize).unsigned_abs();
                let height: usize = (y_max as isize - y_min as isize).unsigned_abs();
                let area: usize = (width + 1) * (height + 1);

                if area > max_area {
                    max_area = area;
                }
            }
        }
    }

    max_area.to_string()
}

fn get_file_path() -> String {
    format!("./inputs/2025/day09{}.txt", if cfg!(test) { "-example" } else { "" })
}

fn parse_input() -> Vec<(usize, usize)> {
    let mut points: Vec<(usize, usize)> = Vec::new();

    if let Ok(lines) = read_lines(get_file_path()) {
        for value in lines.map_while(Result::ok) {
            let parts: Vec<&str> = value.split(",").collect();
            let x: usize = parts[0].parse::<usize>().unwrap();
            let y: usize = parts[1].parse::<usize>().unwrap();

            points.push((x, y));
        }
    }

    points
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(), "50");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), "24");
    }
}
