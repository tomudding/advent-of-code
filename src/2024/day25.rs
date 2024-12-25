// DO NOT EDIT - AOC
use aoc_proc_macros::aoc;
use aoc_shared_functions::{get_registry, read_lines};
// END DO NOT EDIT - AOC

#[aoc(year = "2024", day = "day25", part = "part1")]
fn part1() -> String {
    let (locks, keys): (Vec<Vec<i32>>, Vec<Vec<i32>>) = parse_input();
    let count: usize = keys.iter()
        .flat_map(|key| locks.iter().map(move |lock| (key, lock)))
        .filter(|(key, lock)| (0..5).all(|col| key[col] + lock[col] <= 5))
        .count();

    count.to_string()
}

// #[aoc(year = "2024", day = "day25", part = "part2")]
// fn part2() -> String {
//     let (): () = parse_input();
//
//     todo!()
// }

fn get_file_path() -> String {
    format!("./inputs/2024/day25{}.txt", if cfg!(test) { "-example" } else { "" })
}

fn parse_input() -> (Vec<Vec<i32>>, Vec<Vec<i32>>) {
    let mut locks: Vec<Vec<i32>> = Vec::new();
    let mut keys: Vec<Vec<i32>> = Vec::new();
    
    if let Ok(lines) = read_lines(get_file_path()) {
        // Split again on blocks, way easier.
        for value in lines.map_while(Result::ok).collect::<Vec<_>>().join("\n").split("\n\n") {
            let schematic: Vec<&str> = value.split('\n').collect();
            let heights: Vec<i32> = (0..5)
                .map(|col| schematic.iter().map(|line| line.chars().nth(col).unwrap()).filter(|&c| c == '#').count() as i32 - 1)
                .collect();

            if schematic[0].chars().nth(0).unwrap() == '#' {
                locks.push(heights);
            } else {
                keys.push(heights);
            }
        }
    }

    (locks, keys)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(), "3");
    }

    // #[test]
    // fn test_part2() {
    //     assert_eq!(part2(), todo!());
    // }
}
