// DO NOT EDIT - AOC
use aoc_proc_macros::aoc;
use aoc_shared_functions::{get_registry, read_lines};
// END DO NOT EDIT - AOC

use std::collections::HashMap;

enum Gate {
    AND,
    OR,
    XOR,
}

struct Operation {
    gate: Gate,
    input1: String,
    input2: String,
}

#[aoc(year = "2024", day = "day24", part = "part1")]
fn part1() -> String {
    let (initial_values, operations): (HashMap<String, bool>, Vec<(String, Operation)>) = parse_input();

    let mut wire_values: HashMap<String, bool> = initial_values;
    let mut pending_operations: Vec<(String, Operation)> = operations;

    while !pending_operations.is_empty() {
        let mut new_pending_operations: Vec<(String, Operation)> = Vec::new();

        for (output, op) in pending_operations {
            if let (Some(&val1), Some(&val2)) = (wire_values.get(&op.input1), wire_values.get(&op.input2)) {
                let value: bool = match op.gate {
                    Gate::AND => val1 && val2,
                    Gate::OR => val1 || val2,
                    Gate::XOR => val1 ^ val2,
                };

                wire_values.insert(output, value);
            } else {
                new_pending_operations.push((output, op));
            }
        }

        pending_operations = new_pending_operations;
    }

    let mut result: u64 = 0; // u32 is too small: attempt to shift left with overflow
    for i in 0.. {
        if let Some(&value) = wire_values.get(&format!("z{:02}", i)) {
            result |= (value as u64) << i;
        } else {
            break;
        }
    }

    result.to_string()
}

#[aoc(year = "2024", day = "day24", part = "part2")]
fn part2() -> String {
    let (initial_values, operations) = parse_input();
    // Get expected value.
    let mut x_val: u64 = 0;
    let mut y_val: u64 = 0;

    for i in 0.. {
        if let Some(&value) = initial_values.get(&format!("x{:02}", i)) {
            x_val |= (value as u64) << i;
        } else {
            break;
        }
    }

    for i in 0.. {
        if let Some(&value) = initial_values.get(&format!("y{:02}", i)) {
            y_val |= (value as u64) << i;
        } else {
            break;
        }
    }
    let expected_sum: u64 = x_val + y_val;
    let bit_count: usize = (initial_values.len() / 2) + if expected_sum.leading_zeros() > 0 { 1 } else { 0 };

    let mut expected_values: HashMap<String, bool> = HashMap::new();
    let mut carry: u64 = 0;
    for i in 0..bit_count {
        let x_bit: u64 = (x_val >> i) & 1;
        let y_bit: u64 = (y_val >> i) & 1;
        let sum: u64 = x_bit + y_bit + carry;
        let bit: u64 = sum & 1;

        carry = (sum >> 1) & 1;
        expected_values.insert(format!("z{:02}", i), bit == 1);
    }

    let mut wire_values: HashMap<String, bool> = initial_values;
    let mut pending_operations: Vec<(String, Operation)> = operations;

    while !pending_operations.is_empty() {
        let mut new_pending_operations: Vec<(String, Operation)> = Vec::new();

        for (output, op) in pending_operations {
            if let (Some(&val1), Some(&val2)) = (wire_values.get(&op.input1), wire_values.get(&op.input2)) {
                let value: bool = match op.gate {
                    Gate::AND => val1 && val2,
                    Gate::OR => val1 || val2,
                    Gate::XOR => val1 ^ val2,
                };

                wire_values.insert(output, value);
            } else {
                new_pending_operations.push((output, op));
            }
        }

        pending_operations = new_pending_operations;
    }

    let mut sorted_expected_values: Vec<(String, bool)> = expected_values.iter()
        .map(|(k, &v)| (k.clone(), v))
        .collect();
    let mut sorted_z_values: Vec<(String, bool)> = wire_values.iter()
        .filter(|(k, _)| k.starts_with('z'))
        .map(|(k, &v)| (k.clone(), v))
        .collect();

    sorted_expected_values.sort_by(|a, b| a.0.cmp(&b.0));
    sorted_z_values.sort_by(|a, b| a.0.cmp(&b.0));

    let differences: Vec<String> = sorted_expected_values.iter().zip(&sorted_z_values)
        .map(|((exp_key, exp_val), (_act_key, act_val))| {
            format!("{}: {} {} {}", exp_key, *exp_val as u64, *act_val as u64, if exp_val == act_val { "TRUE" } else { "FALSE" })
        })
        .collect();

    println!("{:#?}", differences);
    // Output:
    // z06: 1 0
    // z07: 0 1
    // z11: 1 0
    // z12: 0 1
    // z31: 1 0
    // z32: 0 1
    // z38: 1 0
    // z39: 1 0
    // z40: 1 0
    // z41: 0 1

    // Solved manually to obtain:
    // z06 <=> fkp
    // z11 <=> ngr
    // z31 <=> mfm
    // bpt <=> krj

    "".to_string()
}

fn get_file_path() -> String {
    format!("./inputs/2024/day24{}.txt", if cfg!(test) { "-example" } else { "" })
}

fn parse_input() -> (HashMap<String, bool>, Vec<(String, Operation)>) {
    let mut initial_values: HashMap<String, bool> = HashMap::new();
    let mut operations: Vec<(String, Operation)> = Vec::new();

    if let Ok(lines) = read_lines(get_file_path()) {
        for value in lines.map_while(Result::ok) {
            if value.contains(":") {
                let parts: Vec<&str> = value.split(": ").collect();
                initial_values.insert(parts[0].to_string(), parts[1] == "1");
            } else if value.contains("->") {
                let parts: Vec<&str> = value.split(" -> ").collect();

                // {input1} {gate} {input2}
                let inputs: Vec<&str> = parts[0].split(' ').collect();
                let input1: String = inputs[0].to_string();
                let gate: Gate = match inputs[1] {
                    "AND" => Gate::AND,
                    "OR" => Gate::OR,
                    "XOR" => Gate::XOR,
                    _ => unreachable!(),
                };
                let input2: String = inputs[2].to_string();

                let output: String = parts[1].to_string();
                operations.push((output, Operation { gate, input1, input2 }));
            }
        }
    }

    (initial_values, operations)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(), "2024");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), todo!());
    }
}
