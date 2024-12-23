use std::{collections::HashMap, fs::File, io::{BufRead, BufReader}, path::Path};

use iter_tools::Itertools;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("total: {:?}", solve("./aoc_input.txt")?);
    Ok(())
}

fn solve<P>(path: P) -> Result<u64, Box<dyn std::error::Error>>
    where P: AsRef<Path>
{
    let sequences = read_file(path)?;
    
    let control_pad = PadLayout::new_control_pad();
    let shortest_keys = init_keypad_shortest_path(&control_pad);
    let shortest_controls = init_controlpad_shortest_path();
    
    let total = sequences.into_iter()
        .map(|seq| eval_route_len::<26>(&seq, &shortest_keys, &shortest_controls).unwrap())
        .map(|(len, seq)| len * seq)
        .sum::<u64>()
    ;

    Ok(total)
}

fn read_file<P>(path: P) -> Result<Vec<String>, Box<dyn std::error::Error>> 
    where P: AsRef<Path>
{
    let reader = BufReader::new(File::open(path)?);
    
    let sequence = reader.lines()
        .map(|s| s.unwrap())
        .collect()
    ;

    Ok(sequence)
}

struct PadLayout {
    width: usize,
    height: usize,
    layout: Vec<Option<char>>,
    index_act: usize,
}

impl PadLayout {
    fn new_key_pad() -> Self {
        let layout = vec![
            Some('7'), Some('8'), Some('9'), 
            Some('4'), Some('5'), Some('6'), 
            Some('1'), Some('2'), Some('3'), 
            None, Some('0'), Some('A'), 
        ];

        Self {
            layout,
            width: 3,
            height: 4,
            index_act: 11,
        }
    }

    fn new_control_pad() -> Self {
        let layout = vec![
            None, Some('^'), Some('A'), 
            Some('<'), Some('v'), Some('>'), 
        ];

        Self {
            layout,
            width: 3,
            height: 2,
            index_act: 2,
        }
    }

    fn from_index(&self, index: usize) -> Point {
        (index % self.width, index / self.width)
    }

    fn to_index(&self, (x, y): Point) -> usize {
        x + y * self.width
    }

    fn can_aimed(&self, mut p0: Point, d0: &[Direction], d1: &[Direction]) -> bool {
        for d in d0 {
            let Some(p) = d.next(p0, self.width, self.height) else {
                return false;
            };
            if self.layout[self.to_index(p)].is_none() {
                return false;
            }
            p0 = p;
        }
        for d in d1 {
            let Some(p) = d.next(p0, self.width, self.height) else {
                return false;
            };
            if self.layout[self.to_index(p)].is_none() {
                return false;
            }
            p0 = p;
        }
        
        true
    }

    fn cost(&self, ch: char) -> usize {
        let Some(index_ch) = self.layout.iter().position(|&pad| pad == Some(ch)) else {
            return 0;
        };

        let (x0, y0) = self.from_index(index_ch);
        let (x1, y1) = self.from_index(self.index_act);

        x0.abs_diff(x1) + y0.abs_diff(y1)
    }
}

type Point = (usize, usize);

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug)]
enum Direction {
    E, N, S, W, 
}

impl Direction {
    fn iter() -> Vec<Direction> {
        vec![Direction::N, Direction::E, Direction::S, Direction::W]
    }

    fn next(&self, (x0, y0): Point, width: usize, height: usize) -> Option<Point> {
        let (x, y) = match self {
            Direction::N if y0 > 0 => (x0, y0 - 1),
            Direction::E => (x0 + 1, y0),
            Direction::S => (x0, y0 + 1),
            Direction::W if x0 > 0 => (x0 -1, y0),
            _ => return None,
        };

        if x >= width { return None; }
        if y >= height { return None; }

        Some((x, y))
    }

    fn to_control_pad(&self) -> char {
        match self {
            Direction::N => '^',
            Direction::E => '>',
            Direction::S => 'v',
            Direction::W => '<',
        }
    }
}

#[derive(PartialEq, Eq, Hash, PartialOrd, Ord, Clone, Debug)]
#[allow(dead_code)]
enum Action {
    Move(Direction),
    Push,
}

type RouteKey = (char, char);

#[derive(PartialEq, Eq, Hash, Clone)]
struct Route {
    path: Vec<Action>,
    distance: usize,
}

impl Route {
    fn of_key(from: char, to: char) -> RouteKey {
        (from, to)
    }

    fn new(path: &[Action], distance: usize) -> Self {
        Self { 
            path: Vec::from(path),
            distance 
        }
    }

    fn reorder_path(&mut self, p0: Point, key_pads: &PadLayout, control_pads: &PadLayout) {
        let map = self.path.iter()
            .filter_map(|actiond| match actiond {
                Action::Move(d) => Some(d.clone()),
                Action::Push => None,
            })
            .group_by(|d| d.clone()).into_iter()
            .map(|(d, g)| (d.clone(), g.count()))
            .collect::<Vec<_>>()
        ;

        if map.len() == 2 {
            let mut lhs = std::iter::repeat(map[0].0.clone()).take(map[0].1).collect::<Vec<_>>();
            let mut rhs = std::iter::repeat(map[1].0.clone()).take(map[1].1).collect::<Vec<_>>();

            let path = match (key_pads.can_aimed(p0, &lhs, &rhs), key_pads.can_aimed(p0, &rhs, &lhs)) {
                (true, false) => {
                    lhs.append(&mut rhs);
                    lhs
                }
                (true, true) if control_pads.cost(map[0].0.to_control_pad()) >= control_pads.cost(map[1].0.to_control_pad()) => {
                    lhs.append(&mut rhs);
                    lhs
                }
                (false, true) |
                (true, true) => {
                    rhs.append(&mut lhs);
                    rhs
                }
                (false, false) => return,
            };

            self.path = path.into_iter().map(|d| Action::Move(d.clone())).collect();
        }
    }
}

fn init_keypad_shortest_path(control_pad: &PadLayout) -> HashMap<RouteKey, String> {
    let pads = PadLayout::new_key_pad();

    let mut paths = HashMap::<RouteKey, Route>::new();

    for (index_from, _) in pads.layout.iter().enumerate() {
        for (index_to, _) in pads.layout.iter().enumerate() {
            let mut processing = vec![false; pads.layout.len()];

            if let Some(key) = init_shortest_path_internal(index_from, index_to, &pads, &mut paths, &mut processing) {
                if let Some(route) = paths.get_mut(&key) {
                    route.reorder_path(pads.from_index(index_from), &pads, &control_pad);
                }
            }
        }
    }

    format_shortest_route(&paths)
}

fn init_controlpad_shortest_path() -> HashMap<RouteKey, String> {
    let pads = PadLayout::new_control_pad();

    let mut paths = HashMap::<RouteKey, Route>::new();

    for (index_from, from) in pads.layout.iter().enumerate() {
        for (index_to, to) in pads.layout.iter().enumerate() {
            let mut processing = vec![false; pads.layout.len()];

            if index_from == index_to {
                match (from, to) {
                    (Some(from), Some(to)) => {
                        paths.insert((*from, *to), Route::new(&vec![], 0));
                    }
                    _ => {}
                }
            }

            let _ = init_shortest_path_internal(index_from, index_to, &pads, &mut paths, &mut processing);
        }
    }

    format_shortest_route(&paths)
}

fn init_shortest_path_internal(index_from: usize, index_to: usize, pads: &PadLayout, paths: &mut HashMap<RouteKey, Route>, processing: &mut [bool]) -> Option<RouteKey> {
    if processing[index_from] {
        return None;
    }
    processing[index_from] = true;

    let Some(from) = pads.layout[index_from] else {
        return None;
    };
    let Some(to) = pads.layout[index_to] else {
        return None;
    };
    if from == to {
        return None;
    }
    if paths.contains_key(&Route::of_key(from, to)) {
        return Some(Route::of_key(from, to));
    }

    let mut shortest_path = None;
    let p0 = pads.from_index(index_from);

    for d in Direction::iter() {
        let Some(p) = d.next(p0, pads.width, pads.height) else {
            continue;
        };

        let index = pads.to_index(p);
        let Some(neighbor) = pads.layout[index] else {
            continue;
        };

        paths.entry(Route::of_key(from, neighbor)).insert_entry(Route::new(&[Action::Move(d.clone())], index.abs_diff(index_from)));

        let route0 = match paths.get(&Route::of_key(neighbor, to)) {
            Some(path0) => Some(path0.clone()),
            None if neighbor == to => Some(Route::new(&[], 0)),
            None => {
                match init_shortest_path_internal(index, index_to, pads, paths, processing) {
                    Some(k) => Some(paths[&k].clone()),
                    None => None,
                }
                
            }
        };

        match (&shortest_path, route0) {
            (None, Some(path0)) => {
                let mut path = path0.path.clone();
                path.insert(0, Action::Move(d));
                shortest_path = Some(path);
            }
            (Some(shortest), Some(path0)) if shortest.len() > path0.path.len() + 1 => {
                let mut path = path0.path.clone();
                path.insert(0, Action::Move(d));
                shortest_path = Some(path);
            }
            _ => {}
        }
    }

    match shortest_path.clone() {
        Some(mut route) => {
            route.sort();
            let key = Route::of_key(from, to);
            paths.entry(key).insert_entry(Route::new(&route, index_to.abs_diff(index_from)));
            Some(key)
        }
        None => None
    }
}

fn format_shortest_route(paths: &HashMap<RouteKey, Route>) -> HashMap<RouteKey, String> {
    paths.into_iter()
        .map(|(key, route)| {
            let path = route.path.iter()
                .map(|d| match d {
                    Action::Move(d) => d.to_control_pad(),
                    Action::Push => 'A',
                })
                .collect::<String>()
            ;
            (key.clone(), path)
        })
        .collect::<HashMap<_, _>>()
}

struct RouteCache<const N: usize> {
    histories: [HashMap<RouteKey, u64>; N],
}

impl<const N: usize> RouteCache<N> {
    fn new<'a>(route_keys: impl Iterator<Item = &'a RouteKey>) -> Self {
        let tmpl = route_keys
            .map(|k| (k.clone(), 0u64))
            .collect::<HashMap<_, _>>()
        ;

        Self {
            histories: std::array::from_fn(|_| tmpl.clone()),
        }
    }
}

fn input_numbers<const N: usize>(code: &str, initial_state: char, shortest_routes: &HashMap<RouteKey, String>, cache: &mut RouteCache<N>) {
    let mut last_state = initial_state;
    let mut last_char = initial_state;

    for ch in code.chars() {
        if let Some(route) = shortest_routes.get(&(last_char, ch)) {
            for pair in std::iter::once(last_state).chain(route.chars()).collect::<Vec<_>>().windows(2) {
                if let Some(c) = cache.histories[N-1].get_mut(&Route::of_key(pair[0], pair[1])) {
                    *c += 1;
                }
                last_state = pair[1];
            }
            if let Some(c) = cache.histories[N-1].get_mut(&(last_state, 'A')) {
                *c += 1;
            }
            last_state = 'A';
        }
        last_char = ch;
    }
}

fn input_cursors_internal<const N: usize>(shortest_routes: &HashMap<RouteKey, String>, depth: usize, cache: &mut RouteCache<N>) {
    let (next_hists, prev_hists) = cache.histories.split_at_mut(depth);

    for (k, count0) in &prev_hists[0] {
        if let Some(route) = shortest_routes.get(&k.clone()) {
            for pair in format!("A{}A", route).chars().collect::<Vec<_>>().windows(2) {
                if let Some(c) = next_hists[depth-1].get_mut(&Route::of_key(pair[0], pair[1])) {
                    *c += count0;
                }
            }
        }
    }
}

fn eval_route_len<const N: usize>(code: &str, key_pad_routes: &HashMap<RouteKey, String>, control_pad_routes: &HashMap<RouteKey, String>) -> Result<(u64, u64), Box<dyn std::error::Error>> {
    let mut cache = RouteCache::<N>::new(control_pad_routes.keys());

    input_numbers::<N>(code, 'A', &key_pad_routes, &mut cache);

    for depth in (1..N).rev() {
        input_cursors_internal::<N>(&control_pad_routes, depth, &mut cache);
    }

    let len = cache.histories[0].iter()
        .map(|(_, count)| *count)
        .sum::<u64>()
    ;

    Ok((len, code.trim_end_matches('A').parse::<u64>()?))
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn resolve_part_1() -> Result<(), Box<dyn std::error::Error>> {
        const DEPTH: usize = 3;

        let control_pad = PadLayout::new_control_pad();
        let shortest_keys = init_keypad_shortest_path(&control_pad);
        let shortest_controls = init_controlpad_shortest_path();

        assert_eq!((68, 29),  eval_route_len::<DEPTH>("029A", &shortest_keys, &shortest_controls)?);
        assert_eq!((60, 980), eval_route_len::<DEPTH>("980A", &shortest_keys, &shortest_controls)?);
        assert_eq!((68, 179), eval_route_len::<DEPTH>("179A", &shortest_keys, &shortest_controls)?);
        assert_eq!((64, 456), eval_route_len::<DEPTH>("456A", &shortest_keys, &shortest_controls)?);
        assert_eq!((64, 379), eval_route_len::<DEPTH>("379A", &shortest_keys, &shortest_controls)?);
        Ok(())
    }
}

