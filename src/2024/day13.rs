// DO NOT EDIT - AOC
use aoc_proc_macros::aoc;
use aoc_shared_functions::{get_registry, read_lines};
// END DO NOT EDIT - AOC

use good_lp::{default_solver, Solution, SolverModel, variable, variables};

fn solve_system(a: (i64, i64), b: (i64, i64), prize: (i64, i64), part2: bool) -> i64 {
    let (a_x, a_y): (i64, i64) = a;
    let (b_x, b_y): (i64, i64) = b;
    let (mut prize_x, mut prize_y): (i64, i64) = prize;

    if part2 {
        prize_x += 10_000_000_000_000;
        prize_y += 10_000_000_000_000;
    }

    // Original system of equations:
    // a_x * a_{presses} + b_x * b_{presses} = prize_x
    // a_y * a_{presses} + b_y * b_{presses} = prize_y
    //
    // Then solve for b_{presses} (multiply the above with a_y and a_x respectively):
    // b = \frac{prize_y \cdot a_x - prize_x \cdot a_y}{b_y \cdot a_x - b_x \cdot a_y}
    //
    // Similarly, solve for a_{presses}:
    // a = \frac{prize_x - b \cdot b_x}{a_x}
    //
    // It remains to show that:
    // a_x \cdot a + b_x \cdot b == prize_x
    // a_y \cdot a + b_y \cdot b == prize_y
    let b = (prize_y * a_x - prize_x * a_y) / (b_y * a_x - b_x * a_y);
    let a = (prize_x - b * b_x) / a_x;

    // If they are not equal, there is no solution.
    if (a_x * a + b_x * b, a_y * a + b_y * b) != (prize_x, prize_y) {
        return 0;
    }

    // Otherwise, there is a solution (by definition this is minimal):
    3 * a + b
}

fn play_machine(a: (f64, f64), b: (f64, f64), prize: (f64, f64), part2: bool) -> Option<f64> {
    let (a_x, a_y): (f64, f64) = a;
    let (b_x, b_y): (f64, f64) = b;
    let (mut prize_x, mut prize_y): (f64, f64) = prize;

    if part2 {
        prize_x += 10_000_000_000_000.0;
        prize_y += 10_000_000_000_000.0;
    }

    let mut vars = variables!();
    let a_presses;
    let b_presses;

    if part2 {
        a_presses = vars.add(variable().integer().min(0));
        b_presses = vars.add(variable().integer().min(0));
    } else {
        // assumption: "(...) pressed no more than 100 times to win a prize."
        a_presses = vars.add(variable().integer().min(0).max(99));
        b_presses = vars.add(variable().integer().min(0).max(99));
    }

    let objective = 3.0 * a_presses + b_presses;

    let solution = vars.minimise(objective)
        .using(default_solver)
        .with((a_presses * a_x + b_x * b_presses).eq(prize_x))
        .with((a_y * a_presses + b_y * b_presses).eq(prize_y))
        .solve();

    match solution {
        Ok(solution) => {
            Some(solution.eval(3.0 * a_presses + b_presses))
        }
        Err(_) => None,
    }
}

#[aoc(year = "2024", day = "day13", part = "part1", function = "external")]
fn part1() -> String {
    // Attempts: 43368, 39976, 39992
    let machines: Vec<((f64, f64), (f64, f64), (f64, f64))> = parse_input();
    let mut total_tokens: f64 = 0.0;

    for (a, b, prize) in machines {
        total_tokens += play_machine(a, b, prize, false).unwrap_or(0.0);
    }

    total_tokens.to_string()
}

#[aoc(year = "2024", day = "day13", part = "part1")]
fn part1_native() -> String {
    let machines: Vec<((f64, f64), (f64, f64), (f64, f64))> = parse_input();
    let mut total_tokens: i64 = 0;

    for (a, b, prize) in machines {
        total_tokens += solve_system((a.0 as i64, a.1 as i64), (b.0 as i64, b.1 as i64), (prize.0 as i64, prize.1 as i64), false);
    }

    total_tokens.to_string()
}

#[aoc(year = "2024", day = "day13", part = "part2", function = "external")]
fn part2() -> String {
    let machines: Vec<((f64, f64), (f64, f64), (f64, f64))> = parse_input();
    let mut total_tokens: f64 = 0.0;

    for (a, b, prize) in machines {
        total_tokens += play_machine(a, b, prize, true).unwrap_or(0.0);
    }

    total_tokens.to_string()
}

#[aoc(year = "2024", day = "day13", part = "part2")]
fn part2_native() -> String {
    let machines: Vec<((f64, f64), (f64, f64), (f64, f64))> = parse_input();
    let mut total_tokens: i64 = 0;

    for (a, b, prize) in machines {
        total_tokens += solve_system((a.0 as i64, a.1 as i64), (b.0 as i64, b.1 as i64), (prize.0 as i64, prize.1 as i64), true);
    }

    total_tokens.to_string()
}

fn get_file_path() -> String {
    format!("./inputs/2024/day13{}.txt", if cfg!(test) { "-example" } else { "" })
}

fn parse_input() -> Vec<((f64, f64), (f64, f64), (f64, f64))> {
    let mut machines: Vec<((f64, f64), (f64, f64), (f64, f64))> = Vec::new();

    let mut a_x: f64 = 0.0;
    let mut a_y: f64 = 0.0;
    let mut b_x: f64 = 0.0;
    let mut b_y: f64 = 0.0;

    if let Ok(lines) = read_lines(get_file_path()) {
        for value in lines.map_while(Result::ok) {
            if value.starts_with("Button A: ") {
                let parts: Vec<&str> = value.split(['+', ',']).collect();

                a_x = parts[1].trim().parse().unwrap();
                a_y = parts[3].trim().parse().unwrap();
            } else if value.starts_with("Button B: ") {
                let parts: Vec<&str> = value.split(['+', ',']).collect();

                b_x = parts[1].trim().parse().unwrap();
                b_y = parts[3].trim().parse().unwrap();
            } else if value.starts_with("Prize: ") {
                let parts: Vec<&str> = value.split(['=', ',']).collect();

                // Cargo complains about defining these outside of this for-loop.
                let prize_x: f64 = parts[1].trim().parse().unwrap();
                let prize_y: f64 = parts[3].trim().parse().unwrap();

                machines.push(((a_x, a_y), (b_x, b_y), (prize_x, prize_y)));
            }
        }
    }

    machines
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(), "480");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), todo!());
    }
}
