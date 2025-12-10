// DO NOT EDIT - AOC
use aoc_proc_macros::aoc;
use aoc_shared_functions::{get_registry, read_lines};
// END DO NOT EDIT - AOC

#[derive(Debug)]
struct JunctionBox {
    x: u128,
    y: u128,
    z: u128,
}

impl JunctionBox {
    fn distance_between(&self, other: &Self) -> i128 {
        let dx: i128 = (self.x - other.x) as i128;
        let dy: i128 = (self.y - other.y) as i128;
        let dz: i128 = (self.z - other.z) as i128;

        dx * dx + dy * dy + dz * dz
    }
}

// Use distance_between as first key, to allow for sorting based on distance without needing an
// extra function to do this.
#[derive(Debug,Eq,PartialEq,Ord,PartialOrd)]
struct Connection {
    distance_between: i128,
    s: usize,
    e: usize,
}

#[derive(Debug)]
struct DSU {
    parent: Vec<usize>,
    size: Vec<usize>,
    // Part2.
    circuits: usize,
}

impl DSU {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![1; n],
            // Part2.
            circuits: n,
        }
    }

    fn find(&mut self, i: usize) -> usize {
        let p: usize = self.parent[i];

        if p != i {
            self.parent[i] = self.find(p);
        }

        self.parent[i]
    }

    fn union(&mut self, i: usize, j: usize) -> bool {
        let root_i: usize = self.find(i);
        let root_j: usize = self.find(j);

        if root_i != root_j {
            if self.size[root_i] < self.size[root_j] {
                self.parent[root_i] = root_j;
                self.size[root_j] += self.size[root_i];
            } else {
                self.parent[root_j] = root_i;
                self.size[root_i] += self.size[root_j];
            }

            // Part2.
            self.circuits -= 1;

            true
        } else {
            false
        }
    }

    fn circuit_sizes(&mut self) -> Vec<usize> {
        self.parent
            .iter()
            .enumerate()
            .filter_map(|(i, &parent)| {
                // Only consider the root of a set.
                if i == parent {
                    Some(self.size[i])
                } else {
                    None
                }
            })
            .collect()
    }
}

#[aoc(year = "2025", day = "day08", part = "part1")]
fn part1() -> String {
    let junction_boxes: Vec<JunctionBox> = parse_input();
    let n = junction_boxes.len();

    // Be naive, just determine _all_ possible connections.
    let mut connections: Vec<Connection> = Vec::new();
    for s in 0..n {
        for e in (s + 1)..n {
            connections.push(Connection {
                distance_between: junction_boxes[s].distance_between(&junction_boxes[e]),
                s,
                e,
            });
        }
    }

    // Sort by distance_between.
    connections.sort_unstable();
    let connections_to_process = connections.iter().take(1000); // example was 10 connections, not 20 :c

    let mut dsu = DSU::new(n);
    for connection in connections_to_process {
        dsu.union(connection.s, connection.e);
    }

    let mut sizes = dsu.circuit_sizes();
    sizes.sort_unstable_by(|a, b| b.cmp(a));

    let a = sizes[0] as u128;
    let b = sizes[1] as u128;
    let c = sizes[2] as u128;

    (a * b * c).to_string()
}

#[aoc(year = "2025", day = "day08", part = "part2")]
fn part2() -> String {
    let junction_boxes: Vec<JunctionBox> = parse_input();
    let n = junction_boxes.len();

    // Part1.
    let mut connections: Vec<Connection> = Vec::new();
    for s in 0..n {
        for e in (s + 1)..n {
            connections.push(Connection {
                distance_between: junction_boxes[s].distance_between(&junction_boxes[e]),
                s,
                e,
            });
        }
    }

    // Sort by distance_between.
    connections.sort_unstable();

    let mut dsu = DSU::new(n);
    for connection in connections {
        if dsu.union(connection.s, connection.e) {
            let last_edge: (usize, usize) = (connection.s, connection.e);

            if dsu.circuits == 1 {
                let xs = junction_boxes[last_edge.0].x as i128;
                let xe = junction_boxes[last_edge.1].x as i128;

                return (xs * xe).to_string();
            }
        }
    }

    unreachable!()
}

fn get_file_path() -> String {
    format!("./inputs/2025/day08{}.txt", if cfg!(test) { "-example" } else { "" })
}

fn parse_input() -> Vec<JunctionBox> {
    let mut points: Vec<JunctionBox> = Vec::new();

    if let Ok(lines) = read_lines(get_file_path()) {
        for value in lines.map_while(Result::ok) {
            let parts: Vec<&str> = value.split(",").collect();
            let x: u128 = parts[0].parse::<u128>().unwrap();
            let y: u128 = parts[1].parse::<u128>().unwrap();
            let z: u128 = parts[2].parse::<u128>().unwrap();

            points.push(JunctionBox { x, y, z });
        }
    }

    points
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(), "40");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), "25272");
    }
}
