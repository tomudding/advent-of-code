// DO NOT EDIT - AOC
use aoc_proc_macros::aoc;
use aoc_shared_functions::{get_registry, read_lines};
// END DO NOT EDIT - AOC

// Debugging function and early attempt at doing this.
#[allow(dead_code)]
fn disk_mapper(files: &[usize], free_spaces: &[usize]) -> Vec<Option<usize>> {
    let mut result: Vec<Option<usize>> = Vec::new();
    let mut file_id: usize = 0;

    for (file, free_space) in files.iter().rev().zip(free_spaces.iter()) {
        // What will part 2 do? We probably need to change something here... Perhaps the file size?
        for _ in 0..*file {
            result.push(Some(file_id));
        }

        for _ in 0..*free_space {
            result.push(None);
        }

        file_id += 1;
    }

    // If there are more files than free spaces (i.e. no free space after last file), we have to
    // make sure we process the last file as well.
    if files.len() > free_spaces.len() {
        for _ in 0..*files.get(free_spaces.len()).unwrap() {
            result.push(Some(file_id));
        }
    }

    result
}

fn compact_disk_map(files: &mut [(usize, usize, usize)], free_spaces: &mut [(usize, usize)], compacted_disk_map: &mut [Option<usize>]) {
    // We need to move files left-to-right, so reverse the order of files.
    for &(file_position, file_size, file_id) in files.iter().rev() {
        for (space_i, &(space_position, space_size)) in free_spaces.iter().enumerate() {
            // Check if we have free space to the left of the file and that it is large enough.
            if space_position < file_position && file_size <= space_size {
                // Move file into position and update newly left space at original position of file.
                for i in 0..file_size {
                    compacted_disk_map[file_position + i] = None;
                    compacted_disk_map[space_position + i] = Some(file_id);
                }

                // Update for the next iteration in `files.iter().rev()`.
                free_spaces[space_i] = (space_position + file_size, space_size - file_size);

                break;
            }
        }
    }
}

fn checksum(compacted_disk_map: &[Option<usize>]) -> String {
    let checksum: usize = compacted_disk_map.iter()
        .enumerate()
        .filter_map(|(i, &c)| c.map(|file_id| i * file_id))
        .sum();

    checksum.to_string()
}

#[aoc(year = "2024", day = "day09", part = "part1")]
fn part1() -> String {
    let (mut files, mut free_spaces, mut compacted_disk_map): (Vec<(usize, usize, usize)>, Vec<(usize, usize)>, Vec<Option<usize>>) = parse_input(false);
    // let disk_map: Vec<Option<usize>> = disk_mapper(&files, &free_spaces);
    // println!("{:?}", disk_map);
    compact_disk_map(&mut files, &mut free_spaces, &mut compacted_disk_map);

    checksum(&compacted_disk_map)
}

#[aoc(year = "2024", day = "day09", part = "part2")]
fn part2() -> String {
    // Only changes required in input parsing to ensure that we have whole blocks of files instead
    // of single parts of files following each other.
    let (mut files, mut free_spaces, mut compacted_disk_map): (Vec<(usize, usize, usize)>, Vec<(usize, usize)>, Vec<Option<usize>>) = parse_input(true);
    compact_disk_map(&mut files, &mut free_spaces, &mut compacted_disk_map);

    checksum(&compacted_disk_map)
}

fn get_file_path() -> String {
    format!("./inputs/2024/day09{}.txt", if cfg!(test) { "-example" } else { "" })
}

fn parse_input(part2: bool) -> (Vec<(usize, usize, usize)>, Vec<(usize, usize)>, Vec<Option<usize>>) {
    let mut files: Vec<(usize, usize, usize)> = Vec::new();
    let mut free_spaces: Vec<(usize, usize)> = Vec::new();
    let mut compacted_disk_map: Vec<Option<usize>> = Vec::new();
    let mut file_id: usize = 0;
    let mut position: usize = 0;
    let mut is_file: bool = true;

    if let Ok(lines) = read_lines(get_file_path()) {
        for value in lines.map_while(Result::ok) {
            for digit in value.chars() {
                let length: usize = digit.to_digit(10).unwrap() as usize;

                if is_file {
                    // We want to move whole blocks of files for part 2. So insert them outside of
                    // loop for the full length of that block (instead of adding single parts of the
                    // file in the loop (with length 1).
                    if part2 {
                        files.push((position, length, file_id));
                    }

                    for _ in 0..length {
                        if !part2 {
                            files.push((position, 1, file_id));
                        }

                        compacted_disk_map.push(Some(file_id));

                        position += 1;
                    }

                    file_id += 1;
                } else {
                    free_spaces.push((position, length));

                    for _ in 0..length {
                        compacted_disk_map.push(None);
                        position += 1;
                    }
                }

                is_file = !is_file;
            }
        }
    }

    (files, free_spaces, compacted_disk_map)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(), "1928");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), "2858");
    }
}
