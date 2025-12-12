// DO NOT EDIT - AOC
use aoc_proc_macros::aoc;
use aoc_shared_functions::{get_registry, read_lines};
// END DO NOT EDIT - AOC

use std::collections::HashSet;

type Shape = HashSet<(isize, isize)>;

struct Region {
    width: usize,
    height: usize,
    shape_counts: Vec<usize>,
}

#[aoc(year = "2025", day = "day12", part = "part1")]
fn part1() -> String {
    let (densities, regions): (Vec<usize>, Vec<Region>) = parse_input();
    let mut count: usize = 0;

    for region in regions {
        let width: usize = region.width;
        let height: usize = region.height;
        let counts: &Vec<usize> = &region.shape_counts;

        let mut min_space: usize = 0;
        let mut total_presents: usize = 0;

        for (i, &count) in counts.iter().enumerate() {
            if let Some(&density) = densities.get(i) {
                min_space += count * density;
                total_presents += count;
            }
        }

        // Prune areas.
        if min_space > width * height {
            continue;
        }

        // Smart heuristic.
        if total_presents <= (width / 3) * (height / 3) {
            count += 1;
        }
    }

    count.to_string()
}

fn get_file_path() -> String {
    format!("./inputs/2025/day12{}.txt", if cfg!(test) { "-example" } else { "" })
}

fn parse_input() -> (Vec<usize>, Vec<Region>) {
    let mut raw_shapes: Vec<Shape> = Vec::new();
    let mut regions: Vec<Region> = Vec::new();
    let mut current_shape_index: usize = 0;
    let mut current_shape_grid: Vec<String> = Vec::new();

    if let Ok(lines) = read_lines(get_file_path()) {
        for value in lines.map_while(Result::ok) {
            if value.trim().is_empty() { continue; }

            if current_shape_index < 6 {
                if value.ends_with(':') {
                    if !current_shape_grid.is_empty() {
                        let mut shape: HashSet<(isize, isize)> = Shape::new();
                        for (y, row) in current_shape_grid.iter().enumerate() {
                            for (x, char) in row.chars().enumerate() {
                                if char == '#' {
                                    shape.insert((y as isize, x as isize));
                                }
                            }
                        }

                        if raw_shapes.len() <= current_shape_index {
                            raw_shapes.resize(current_shape_index + 1, Shape::new());
                        }

                        raw_shapes[current_shape_index] = shape;
                        current_shape_grid.clear();
                    }

                    // New shape header
                    if let Ok(idx_str) = value.trim_end_matches(':').parse::<usize>() {
                        current_shape_index = idx_str;
                    }
                } else if value.contains('#') || value.contains('.') {
                    current_shape_grid.push(value.to_string());
                } else {
                    // This is the last shape.
                    if !current_shape_grid.is_empty() {
                        let mut shape: HashSet<(isize, isize)> = Shape::new();
                        for (y, row) in current_shape_grid.iter().enumerate() {
                            for (x, char) in row.chars().enumerate() {
                                if char == '#' {
                                    shape.insert((y as isize, x as isize));
                                }
                            }
                        }

                        if raw_shapes.len() <= current_shape_index {
                            raw_shapes.resize(current_shape_index + 1, Shape::new());
                        }
                        raw_shapes[current_shape_index] = shape;
                    }

                    // Parse first region.
                    let parts: Vec<&str> = value.split(": ").collect();
                    let size_parts: Vec<&str> = parts[0].split('x').collect();
                    let width: usize = size_parts[0].parse::<usize>().unwrap();
                    let height: usize = size_parts[1].parse::<usize>().unwrap();
                    let shape_counts: Vec<usize> = parts[1]
                        .split_whitespace()
                        .map(|s| s.parse::<usize>().unwrap())
                        .collect();

                    regions.push(Region { width, height, shape_counts });
                }
            } else {
                // Region parsing
                let parts: Vec<&str> = value.split(": ").collect();
                let size_parts: Vec<&str> = parts[0].split('x').collect();
                let width: usize = size_parts[0].parse::<usize>().unwrap();
                let height: usize = size_parts[1].parse::<usize>().unwrap();
                let shape_counts: Vec<usize> = parts[1]
                    .split_whitespace()
                    .map(|s| s.parse::<usize>().unwrap())
                    .collect();

                regions.push(Region { width, height, shape_counts });
            }
        }
    }

    let densities: Vec<usize> = raw_shapes.into_iter().map(|shape| shape.len()).collect();

    (densities, regions)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(), "2");
    }
}
