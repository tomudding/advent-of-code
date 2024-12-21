// DO NOT EDIT - AOC
use aoc_proc_macros::aoc;
use aoc_shared_functions::{get_registry, read_lines};
// END DO NOT EDIT - AOC

use std::collections::{HashMap, HashSet, VecDeque};

trait Keypad {
    fn get(&self, x: isize, y: isize) -> Option<char>;
    fn get_position(&self, c: char) -> Option<(isize, isize)>;
    fn height(&self) -> isize;
    fn width(&self) -> isize;
}

const NUMERIC_KEYPAD: [[Option<char>; 3]; 4] = [
    [Some('7'), Some('8'), Some('9')],
    [Some('4'), Some('5'), Some('6')],
    [Some('1'), Some('2'), Some('3')],
    [None,      Some('0'), Some('A')],
];
const DIRECTIONAL_KEYPAD: [[Option<char>; 3]; 2] = [
    [None,      Some('^'), Some('A')],
    [Some('<'), Some('v'), Some('>')],
];

impl<const H: usize, const W: usize> Keypad for [[Option<char>; W]; H] {
    fn get(&self, x: isize, y: isize) -> Option<char> {
        if x < 0 || y < 0 || y >= self.height() || x >= self.width() {
            return None;
        }

        self[y as usize][x as usize]
    }
    fn get_position(&self, c: char) -> Option<(isize, isize)> {
        for y in 0..self.height() {
            for x in 0..self.width() {
                if self.get(x, y) == Some(c) {
                    return Some((x, y));
                }
            }
        }

        None
    }
    fn height(&self) -> isize {
        H as isize
    }
    fn width(&self) -> isize {
        W as isize
    }
}

const DIRECTIONS: [(isize, isize); 4] = [
    (0, -1), // ^
    (1, 0),  // >
    (0, 1),  // v
    (-1, 0), // <
];

const DIRECTION_CHARS: [char; 4] = [
    '^',
    '>',
    'v',
    '<',
];

fn get_path_on_keypad<T: Keypad>(keypad: &T, code: &str) -> String {
    // Tried to use BFS initially, however, I could not get it to work. Probably due to the lack of
    // sleep making it incredibly hard to think.
    let mut path: String = String::new();
    // We always start on the 'A' key of the keypad.
    let mut start: char = 'A';

    // Determine inner path between two consecutive keys in the code on the keypad.
    for end in code.chars() {
        path.push_str(&get_inner_path_on_keypad(keypad, start, end));
        start = end;
    }

    path
}

fn get_inner_path_on_keypad<T: Keypad>(keypad: &T, start: char, end: char) -> String {
    let (sx, sy): (isize, isize) = keypad.get_position(start).unwrap(); // guaranteed to be on the keypad
    let (ex, ey): (isize, isize) = keypad.get_position(end).unwrap(); // guaranteed to be on the keypad
    let (dx, dy): (isize, isize) = (ex - sx, ey - sy);

    let horizontal_moves = if dx > 0 {
        ">".repeat(dx as usize)
    } else {
        "<".repeat(dx.abs() as usize)
    };
    let vertical_moves = if dy > 0 {
        "v".repeat(dy as usize)
    } else {
        "^".repeat(dy.abs() as usize)
    };

    // Choose the most logical diagonal for the moves (always prefers top-right for numeric and
    // bottom-right for directional - this ensures we skip the empty gaps).
    if dx > 0 && keypad.get(sx, ey).is_some() {
        return format!("{}{}A", vertical_moves, horizontal_moves);
    }

    if keypad.get(ex, sy).is_some() {
        return format!("{}{}A", horizontal_moves, vertical_moves);
    }

    if keypad.get(sx, ey).is_some() {
        return format!("{}{}A", vertical_moves, horizontal_moves);
    }

    unreachable!()
}

#[allow(dead_code)]
fn bfs<T: Keypad>(keypad: &T, start: char, target: char) -> String {
    let mut queue: VecDeque<((isize, isize), Vec<char>)> = VecDeque::new();
    let mut visited: HashSet<(isize, isize)> = HashSet::new();

    queue.push_back((keypad.get_position(start).unwrap(), vec![]));

    while let Some(((x, y), path)) = queue.pop_front() {
        if keypad.get(x, y) == Some(target) {
            let mut final_path: Vec<char> = path.clone();
            final_path.push('A');

            return final_path.iter().collect();
        }

        for (i, &(dx, dy)) in DIRECTIONS.iter().enumerate() {
            let nx: isize = x + dx;
            let ny: isize = y + dy;

            if ny >= 0 && ny < keypad.height() && nx >= 0 && nx < keypad.width() {
                if keypad.get(nx, ny).is_some() && !visited.contains(&(nx, ny)) {
                    visited.insert((nx, ny));

                    let mut new_path: Vec<char> = path.clone();
                    new_path.push(DIRECTION_CHARS[i]);
                    queue.push_back(((nx, ny), new_path));
                }
            }
        }
    }

    unreachable!()
}

#[allow(dead_code)]
fn get_position<T: Keypad>(keypad: &T, target: char) -> Option<(isize, isize)> {
    for y in 0..keypad.height() {
        for x in 0..keypad.width() {
            if keypad.get(x, y) == Some(target) {
                return Some((x, y));
            }
        }
    }

    None
}

#[aoc(year = "2024", day = "day21", part = "part1")]
fn part1() -> String {
    let codes: Vec<String> = parse_input();
    let numeric_routes: Vec<String> = codes.iter().map(|code| get_path_on_keypad(&NUMERIC_KEYPAD, code)).collect();
    let directional_routes_a: Vec<String> = numeric_routes.iter().map(|code| get_path_on_keypad(&DIRECTIONAL_KEYPAD, code)).collect();
    let directional_routes_b: Vec<String> = directional_routes_a.iter().map(|code| get_path_on_keypad(&DIRECTIONAL_KEYPAD, code)).collect();

    let total_complexities: isize = directional_routes_b
        .iter()
        .zip(&codes)
        .map(|(route, line)| (route.len() as isize) * line[..line.len() - 1].parse::<isize>().unwrap())
        .sum();

    total_complexities.to_string()
}

fn get_path_on_keypad_memoization<T: Keypad>(keypad: &T, code: &str) -> HashMap<String, isize> {
    // Memoization to the rescue!
    let mut path: HashMap<String, isize> = HashMap::new();
    let mut start: char = 'A';

    for end in code.chars() {
        *path.entry(get_inner_path_on_keypad(keypad, start, end)).or_insert(0) += 1;
        start = end;
    }

    path
}

#[aoc(year = "2024", day = "day21", part = "part2")]
fn part2() -> String {
    let codes: Vec<String> = parse_input();
    let numeric_routes: Vec<String> = codes.iter().map(|code| get_path_on_keypad(&NUMERIC_KEYPAD, code)).collect();
    // Convert to memoization pattern:
    let mut directional_routes: Vec<HashMap<String, isize>> = numeric_routes.iter().map(|code| {
        let mut m = HashMap::new();
        m.insert(code.clone(), 1);
        m
    }).collect();

    // Do the magic!
    for _ in 0..25 {
        let mut new_directional_routes: Vec<HashMap<String, isize>> = Vec::new();

        for code_and_counts in &directional_routes {
            let mut new_code_route: HashMap<String, isize> = HashMap::new();

            for (code, &count) in code_and_counts.iter() {
                let new_code_and_counts: HashMap<String, isize> = get_path_on_keypad_memoization(&DIRECTIONAL_KEYPAD, code);

                for (new_code, &new_count) in &new_code_and_counts {
                    *new_code_route.entry(new_code.clone()).or_insert(0) += new_count * count;
                }
            }

            new_directional_routes.push(new_code_route);
        }

        directional_routes = new_directional_routes;
    }

    let total_complexities: isize = directional_routes
        .iter()
        .zip(&codes)
        .map(|(route, line)| (route.iter().map(|(code, &count)| (code.len() as isize) * count).sum::<isize>()) * line[..line.len() - 1].parse::<isize>().unwrap())
        .sum();

    total_complexities.to_string()
}


#[aoc(year = "2024", day = "day21", part = "part2", function = "naive")]
fn part2_naive() -> String {
    // This takes too long to compute :) (but it is quite fast up to p and r).
    let codes: Vec<String> = parse_input();
    let numeric_routes: Vec<String> = codes.iter().map(|code| get_path_on_keypad(&NUMERIC_KEYPAD, code)).collect();
    let directional_routes_a: Vec<String> = numeric_routes.iter().map(|code| get_path_on_keypad(&DIRECTIONAL_KEYPAD, code)).collect();
    let directional_routes_b: Vec<String> = directional_routes_a.iter().map(|code| get_path_on_keypad(&DIRECTIONAL_KEYPAD, code)).collect();
    let directional_routes_c: Vec<String> = directional_routes_b.iter().map(|code| get_path_on_keypad(&DIRECTIONAL_KEYPAD, code)).collect();
    let directional_routes_d: Vec<String> = directional_routes_c.iter().map(|code| get_path_on_keypad(&DIRECTIONAL_KEYPAD, code)).collect();
    let directional_routes_e: Vec<String> = directional_routes_d.iter().map(|code| get_path_on_keypad(&DIRECTIONAL_KEYPAD, code)).collect();
    let directional_routes_f: Vec<String> = directional_routes_e.iter().map(|code| get_path_on_keypad(&DIRECTIONAL_KEYPAD, code)).collect();
    let directional_routes_g: Vec<String> = directional_routes_f.iter().map(|code| get_path_on_keypad(&DIRECTIONAL_KEYPAD, code)).collect();
    let directional_routes_h: Vec<String> = directional_routes_g.iter().map(|code| get_path_on_keypad(&DIRECTIONAL_KEYPAD, code)).collect();
    let directional_routes_i: Vec<String> = directional_routes_h.iter().map(|code| get_path_on_keypad(&DIRECTIONAL_KEYPAD, code)).collect();
    let directional_routes_j: Vec<String> = directional_routes_i.iter().map(|code| get_path_on_keypad(&DIRECTIONAL_KEYPAD, code)).collect();
    let directional_routes_k: Vec<String> = directional_routes_j.iter().map(|code| get_path_on_keypad(&DIRECTIONAL_KEYPAD, code)).collect();
    let directional_routes_l: Vec<String> = directional_routes_k.iter().map(|code| get_path_on_keypad(&DIRECTIONAL_KEYPAD, code)).collect();
    let directional_routes_m: Vec<String> = directional_routes_l.iter().map(|code| get_path_on_keypad(&DIRECTIONAL_KEYPAD, code)).collect();
    let directional_routes_n: Vec<String> = directional_routes_m.iter().map(|code| get_path_on_keypad(&DIRECTIONAL_KEYPAD, code)).collect();
    let directional_routes_o: Vec<String> = directional_routes_n.iter().map(|code| get_path_on_keypad(&DIRECTIONAL_KEYPAD, code)).collect();
    let directional_routes_p: Vec<String> = directional_routes_o.iter().map(|code| get_path_on_keypad(&DIRECTIONAL_KEYPAD, code)).collect();
    let directional_routes_q: Vec<String> = directional_routes_p.iter().map(|code| get_path_on_keypad(&DIRECTIONAL_KEYPAD, code)).collect();
    let directional_routes_r: Vec<String> = directional_routes_q.iter().map(|code| get_path_on_keypad(&DIRECTIONAL_KEYPAD, code)).collect();
    let directional_routes_s: Vec<String> = directional_routes_r.iter().map(|code| get_path_on_keypad(&DIRECTIONAL_KEYPAD, code)).collect();
    let directional_routes_t: Vec<String> = directional_routes_s.iter().map(|code| get_path_on_keypad(&DIRECTIONAL_KEYPAD, code)).collect();
    let directional_routes_u: Vec<String> = directional_routes_t.iter().map(|code| get_path_on_keypad(&DIRECTIONAL_KEYPAD, code)).collect();
    let directional_routes_v: Vec<String> = directional_routes_u.iter().map(|code| get_path_on_keypad(&DIRECTIONAL_KEYPAD, code)).collect();
    let directional_routes_w: Vec<String> = directional_routes_v.iter().map(|code| get_path_on_keypad(&DIRECTIONAL_KEYPAD, code)).collect();
    let directional_routes_x: Vec<String> = directional_routes_w.iter().map(|code| get_path_on_keypad(&DIRECTIONAL_KEYPAD, code)).collect();
    let directional_routes_y: Vec<String> = directional_routes_x.iter().map(|code| get_path_on_keypad(&DIRECTIONAL_KEYPAD, code)).collect();

    let total_complexities: isize = directional_routes_y
        .iter()
        .zip(&codes)
        .map(|(route, line)| (route.len() as isize) * line[..line.len() - 1].parse::<isize>().unwrap())
        .sum();

    total_complexities.to_string()
}

fn get_file_path() -> String {
    format!("./inputs/2024/day21{}.txt", if cfg!(test) { "-example" } else { "" })
}

fn parse_input() -> Vec<String> {
    let mut codes: Vec<String> = Vec::new();

    if let Ok(lines) = read_lines(get_file_path()) {
        for value in lines.map_while(Result::ok) {
            codes.push(value.trim().to_string());
        }
    }

    codes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(), "126384");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), todo!());
    }
}
