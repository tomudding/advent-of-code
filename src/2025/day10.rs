use std::collections::{HashSet, VecDeque};
// DO NOT EDIT - AOC
use aoc_proc_macros::aoc;
use aoc_shared_functions::{get_registry, read_lines};
// END DO NOT EDIT - AOC

use good_lp::{default_solver, Expression, Solution, SolverModel, variable, variables};

struct Machine {
    target_mask: usize,
    button_masks: Vec<usize>,
    // Part2.
    joltage_buttons: Vec<Vec<usize>>,
    joltage_target: Vec<usize>,
}

#[aoc(year = "2025", day = "day10", part = "part1")]
fn part1() -> String {
    let machines: Vec<Machine> = parse_input();
    let total: usize = machines.iter().map(min_presses).sum();

    total.to_string()
}

fn min_presses(machine: &Machine) -> usize {
    let mut visited: HashSet<usize> = HashSet::new();
    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();

    // Start with all lights off
    queue.push_back((0, 0));
    visited.insert(0);

    while let Some((state, steps)) = queue.pop_front() {
        if state == machine.target_mask {
            return steps;
        }

        for &button_mask in &machine.button_masks {
            let new_state = state ^ button_mask;

            if !visited.contains(&new_state) {
                visited.insert(new_state);
                queue.push_back((new_state, steps + 1));
            }
        }
    }

    usize::MAX
}

#[aoc(year = "2025", day = "day10", part = "part2")]
fn part2() -> String {
    let machines: Vec<Machine> = parse_input();
    let total: usize = machines.iter().map(min_presses_ilp).sum();

    total.to_string()
}

fn min_presses_ilp(machine: &Machine) -> usize {
    let mut vars = variables!();
    let mut button_vars = Vec::new();

    // There are no assumptions on max; so only use min.
    for _ in &machine.joltage_buttons {
        button_vars.push(vars.add(variable().integer().min(0)));
    }

    // Build constraints: we want to minimise the number of button presses for each joltage counter.
    let mut problem = vars.minimise(button_vars.iter().sum::<Expression>()).using(default_solver);

    for (j, &target) in machine.joltage_target.iter().enumerate() {
        let mut expr = Expression::from(0);

        for (i, button) in machine.joltage_buttons.iter().enumerate() {
            if button.contains(&j) {
                expr += button_vars[i];
            }
        }

        problem = problem.with(expr.eq(target as u32)); // Does not support usize.
    }

    let solution = problem.solve().unwrap();
    button_vars
        .iter()
        .map(|v| solution.value(*v) as usize)
        .sum()
}

fn get_file_path() -> String {
    format!("./inputs/2025/day10{}.txt", if cfg!(test) { "-example" } else { "" })
}

fn parse_input() -> Vec<Machine> {
    let mut machines: Vec<Machine> = Vec::new();

    if let Ok(lines) = read_lines(get_file_path()) {
        for value in lines.map_while(Result::ok) {
            let parts: Vec<&str> = value.split_whitespace().collect();

            // Parse indicator lights.
            let indicator: &str = parts[0].trim_matches(|c| c == '[' || c == ']');
            let mut target_mask: usize = 0;
            for (i, ch) in indicator.chars().enumerate() {
                if ch == '#' {
                    target_mask |= 1 << i;
                }
            }

            // Parse buttons.
            let mut button_masks: Vec<usize> = Vec::new();
            let mut joltage_buttons: Vec<Vec<usize>> = Vec::new();
            for p in &parts[1..(parts.len() - 1)] {
                if p.starts_with('(') {
                    let button_idx: Vec<usize> = p
                        .trim_matches(|c| c == '(' || c == ')')
                        .split(',')
                        .map(|s| s.parse::<usize>().unwrap())
                        .collect();

                    let mut button_mask: usize = 0;
                    for n in &button_idx {
                        button_mask |= 1 << n;
                    }

                    button_masks.push(button_mask);

                    // Part2.
                    joltage_buttons.push(button_idx);
                }
            }

            // Parse joltage (part2).
            let joltage_target: Vec<usize> = parts[parts.len() - 1]
                .trim_matches(|c| c == '{' || c == '}')
                .split(',')
                .map(|s| s.parse::<usize>().unwrap())
                .collect();

            machines.push(Machine {
                target_mask,
                button_masks,
                // Part2.
                joltage_buttons,
                joltage_target,
            });
        }
    }

    machines
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(), "7");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), "33");
    }
}
