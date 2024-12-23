// DO NOT EDIT - AOC
use aoc_proc_macros::aoc;
use aoc_shared_functions::{get_registry, read_lines};
// END DO NOT EDIT - AOC

use std::collections::{BTreeSet, HashMap, HashSet};

fn find_groups(computers: &HashMap<String, BTreeSet<String>>) -> HashSet<BTreeSet<String>> {
    let mut groups = HashSet::new();

    for (a, neighbors) in computers {
        for b in neighbors {
            if let Some(common_neighbors) = computers.get(b) {
                for c in common_neighbors.intersection(neighbors) {
                    // BTreeSet needed as a HashSet cannot be used as a key for HashSet.
                    let mut group: BTreeSet<String> = BTreeSet::new();
                    group.insert(a.clone());
                    group.insert(b.clone());
                    group.insert(c.clone());

                    groups.insert(group);
                }
            }
        }
    }

    groups
}

#[aoc(year = "2024", day = "day23", part = "part1")]
fn part1() -> String {
    let computers: HashMap<String, BTreeSet<String>> = parse_input();
    let groups: HashSet<BTreeSet<String>> = find_groups(&computers);
    let groups_starting_with_t: usize = groups
        .into_iter()
        .filter(|triangle| triangle.iter().any(|name| name.starts_with('t'))).count();

    groups_starting_with_t.to_string()
}

// https://stackoverflow.com/q/60835260/5914775 / https://en.wikipedia.org/wiki/Bron–Kerbosch_algorithm#With_pivoting (thanks to @JustSamuel for the suggestion)
// Compiler complained about the `mut`s, so `mut`s there are...
fn bron_kerbosch(computers: &HashMap<String, BTreeSet<String>>, cliques: &mut Vec<BTreeSet<String>>, mut r: BTreeSet<String>, mut p: BTreeSet<String>, mut x: BTreeSet<String>) {
    if p.is_empty() && x.is_empty() {
        cliques.push(r);

        return;
    }

    let u: String = p.union(&x).next().unwrap().clone();
    let binding: BTreeSet<String> = BTreeSet::new(); // Compiler complained about this (without it the &BTreeSet::new() in the line below would be freed while in use).
    let neighbors: &BTreeSet<String> = computers.get(&u).unwrap_or(&binding);

    let p_without_neighbors: BTreeSet<String> = p.difference(neighbors).cloned().collect::<BTreeSet<_>>();
    for v in p_without_neighbors {
        r.insert(v.clone());

        // Using `BTreeSet::intersection(a, b).cloned.collect()` is cleaner than `a.intersection(b).cloned().collect::<BTreeSet<_>>()`.
        let p_new: BTreeSet<String> = BTreeSet::intersection(&p, computers.get(&v).unwrap_or(&BTreeSet::new())).cloned().collect();
        let x_new: BTreeSet<String> = BTreeSet::intersection(&x, computers.get(&v).unwrap_or(&BTreeSet::new())).cloned().collect();

        bron_kerbosch(computers, cliques, r.clone(), p_new, x_new);

        r.remove(&v);
        p.remove(&v);
        x.insert(v);
    }
}

#[aoc(year = "2024", day = "day23", part = "part2")]
fn part2() -> String {
    let computers: HashMap<String, BTreeSet<String>> = parse_input();

    let mut cliques: Vec<BTreeSet<String>> = Vec::new();
    bron_kerbosch(&computers, &mut cliques, BTreeSet::new(), computers.keys().cloned().collect(), BTreeSet::new());
    let largest_clique: BTreeSet<String> = cliques.into_iter().max_by_key(|clique| clique.len()).unwrap_or(BTreeSet::new());
    let mut computer_names: Vec<String> = largest_clique.into_iter().collect();

    computer_names.sort();
    computer_names.join(",")
}

fn get_file_path() -> String {
    format!("./inputs/2024/day23{}.txt", if cfg!(test) { "-example" } else { "" })
}

fn parse_input() -> HashMap<String, BTreeSet<String>> {
    let mut computers: HashMap<String, BTreeSet<String>> = HashMap::new();

    if let Ok(lines) = read_lines(get_file_path()) {
        for value in lines.map_while(Result::ok) {
            let parts: Vec<&str> = value.split("-").collect();
            let a: String = parts[0].to_string();
            let b: String = parts[1].to_string();

            computers.entry(a.clone()).or_insert_with(BTreeSet::new).insert(b.clone()); // clone needed as value is moved
            computers.entry(b).or_insert_with(BTreeSet::new).insert(a);
        }
    }

    computers
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
        assert_eq!(part2(), "co,de,ka,ta");
    }
}
