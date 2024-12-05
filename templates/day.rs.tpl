// DO NOT EDIT - AOC
use aoc_function_registry::get_registry;
use aoc_proc_macros::aoc;
// END DO NOT EDIT - AOC
// DO NOT EDIT - DEFAULTS
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
// DO NOT EDIT - DEFAULTS

#[aoc(year = "{YEAR}", day = "{DAY}", part = "part1")]
fn part1() -> String {
    let (): () = parse_input();

    todo!()
}

#[aoc(year = "{YEAR}", day = "{DAY}", part = "part2")]
fn part2() -> String {
    let (): () = parse_input();

    todo!()
}

fn get_file_path() -> String {
    format!("./inputs/{YEAR}/{DAY}{}.txt", if cfg!(test) { "-example" } else { "" })
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
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
