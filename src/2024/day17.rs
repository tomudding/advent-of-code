// DO NOT EDIT - AOC
use aoc_proc_macros::aoc;
use aoc_shared_functions::{get_registry, read_lines};
// END DO NOT EDIT - AOC

fn run_program(register_a: isize, register_b: isize, register_c: isize, program: &[isize]) -> String {
    let mut a: isize = register_a;
    let mut b: isize = register_b;
    let mut c: isize = register_c;

    let mut instruction_pointer: usize = 0;
    let mut output: Vec<String> = Vec::new();

    while instruction_pointer < program.len() {
        let opcode = program[instruction_pointer];
        let operand = program[instruction_pointer + 1];

        match opcode {
            0 => { // adv instruction
                let divisor = 2isize.pow(resolve_combo(operand, a, b, c) as u32);
                a /= divisor;
            },
            1 => { // bxl instruction
                b ^= operand;
            },
            2 => { // bst instruction
                b = resolve_combo(operand, a, b, c) % 8;
            },
            3 => { // jnz instruction
                if a != 0 {
                    instruction_pointer = operand as usize;
                    continue;
                }
            },
            4 => { // bxc instruction
                b ^= c;
            },
            5 => { // out instruction
                output.push((resolve_combo(operand, a, b, c) % 8).to_string());
            },
            6 => { // bdv instruction
                let divisor = 2isize.pow(resolve_combo(operand, a, b, c) as u32);
                b = a / divisor;
            },
            7 => { // cdv instruction
                let divisor = 2isize.pow(resolve_combo(operand, a, b, c) as u32);
                c = a / divisor;
            },
            _ => unreachable!(),
        }

        instruction_pointer += 2;
    }

    output.join(",")
}

fn resolve_combo(operand: isize, a: isize, b: isize, c: isize) -> isize {
    match operand {
        0..=3 => operand,
        4 => a,
        5 => b,
        6 => c,
        _ => unreachable!(),
    }
}

#[aoc(year = "2024", day = "day17", part = "part1")]
fn part1() -> String {
    let (register_a, register_b, register_c, program): (isize, isize, isize, Vec<isize>) = parse_input();

    run_program(register_a, register_b, register_c, &program)
}

#[aoc(year = "2024", day = "day17", part = "part2")]
fn part2() -> String {
    let (_, _, _, mut program): (isize, isize, isize, Vec<isize>) = parse_input();

    let mut queue: Vec<isize> = (0..8).collect();
    program.reverse();

    let mut valid_values_of_a: Vec<isize> = Vec::new();

    for &value in program.iter() {
        valid_values_of_a = Vec::new();

        // Execute the program.
        for &a in queue.iter() {
            // 2,4
            let mut b = a % 8;
            // 1,5
            b ^= 5;
            // 7,5
            let c = a / (1 << b);
            // 4,3
            b ^= c;
            // 1,6
            b ^= 6;
            // 0,3 (don't touch a)
            // a = a // (1 << 3)
            // 5,5
            if b % 8 == value {
                valid_values_of_a.push(a);
            }
            // 3,0 (ignored)
        }

        let mut new_queue: Vec<isize> = Vec::new();
        for &v in valid_values_of_a.iter() {
            for i in 0..8 {
                new_queue.push(v * 8 + i);
            }
        }

        queue = new_queue;
    }

    valid_values_of_a.iter().min().unwrap().to_string()
}

fn get_file_path() -> String {
    format!("./inputs/2024/day17{}.txt", if cfg!(test) { "-example" } else { "" })
}

fn parse_input() -> (isize, isize, isize, Vec<isize>) {
    let mut register_a: isize = 0;
    let mut register_b: isize = 0;
    let mut register_c: isize = 0;
    let mut program: Vec<isize> = Vec::new();

    if let Ok(lines) = read_lines(get_file_path()) {
        for value in lines.map_while(Result::ok) {
            if value.starts_with("Register A: ") {
                register_a = value.split_whitespace().nth(2).unwrap().parse().unwrap();
            } else if value.starts_with("Register B: ") {
                register_b = value.split_whitespace().nth(2).unwrap().parse().unwrap();
            } else if value.starts_with("Register C: ") {
                register_c = value.split_whitespace().nth(2).unwrap().parse().unwrap();
            } else if value.starts_with("Program: ") {
                program = value.split_whitespace().nth(1).unwrap().split([',']).map(|num| num.parse().unwrap()).collect();
            }
        }
    }

    (register_a, register_b, register_c, program)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(), "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), "117440");
    }
}
