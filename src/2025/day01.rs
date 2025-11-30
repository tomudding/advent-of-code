// DO NOT EDIT - AOC
use aoc_proc_macros::aoc;
use aoc_shared_functions::{get_registry, read_lines};
// END DO NOT EDIT - AOC

#[aoc(year = "2025", day = "day01", part = "part1")]
fn part1() -> String {
    let (): () = parse_input();

    todo!()
}

#[aoc(year = "2025", day = "day01", part = "part2")]
fn part2() -> String {
    let (): () = parse_input();

    todo!()
}

fn get_file_path() -> String {
    format!("./inputs/2025/day01{}.txt", if cfg!(test) { "-example" } else { "" })
}

fn parse_input() -> todo!() {
    if let Ok(lines) = read_lines(get_file_path()) {
        for value in lines.map_while(Result::ok) {
            todo!();
        }
    }

    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(), todo!());
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), todo!());
    }
}
