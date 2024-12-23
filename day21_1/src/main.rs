use std::{collections::HashMap, fs::File, io::{BufRead, BufReader}, path::Path};

use iter_tools::Itertools;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("total: {:?}", solve("./aoc_input.txt")?);
    Ok(())
}

fn solve<P>(path: P) -> Result<usize, Box<dyn std::error::Error>>
    where P: AsRef<Path>
{
    let sequences = read_file(path)?;
    
    let control_pad = PadLayout::new_control_pad();
    let shortest_keys = init_keypad_shortest_path(&control_pad);
    let shortest_controls = init_controlpad_shortest_path();
    
    let total = sequences.into_iter()
        .map(|seq| eval_route_len(&seq, &shortest_keys, &shortest_controls, 2))
        .map(|(len, seq)| len * seq)
        .sum::<usize>()
    ;

    Ok(total)
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

type RouteKey = (char, char);

#[derive(PartialEq, Eq, Hash, Clone)]
struct Route {
    path: Vec<Direction>,
    distance: usize,
}

impl Route {
    fn of_key(from: char, to: char) -> RouteKey {
        (from, to)
    }

    fn new(path: &[Direction], distance: usize) -> Self {
        Self { 
            path: Vec::from(path),
            distance 
        }
    }

    fn reorder_path(&mut self, p0: Point, key_pads: &PadLayout, control_pads: &PadLayout) {
        let map = self.path.iter().group_by(|&d| d.clone()).into_iter().map(|(d, g)| (d.clone(), g.count())).collect::<Vec<_>>();

        if map.len() == 2 {
            let mut lhs = std::iter::repeat(map[0].0.clone()).take(map[0].1).collect::<Vec<_>>();
            let mut rhs = std::iter::repeat(map[1].0.clone()).take(map[1].1).collect::<Vec<_>>();

            self.path = match (key_pads.can_aimed(p0, &lhs, &rhs), key_pads.can_aimed(p0, &rhs, &lhs)) {
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
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
enum Action {
    MoveNum(Vec<Direction>),
    MoveControl(Direction),
    Push,
}

impl Action {
    #[allow(unused)]
    fn moving_paths(&self) -> MovePath {
        match self {
            Action::MoveNum(paths) => MovePath { inner: Box::new(paths.clone().into_iter()) },
            Action::MoveControl(d) => MovePath { inner: Box::new(std::iter::once(d.clone()).into_iter()) },
            Action::Push => MovePath { inner: Box::new(std::iter::empty()) },
        }
    }
}

struct MovePath {
    inner: Box<dyn Iterator<Item = Direction>>,
}

impl Iterator for MovePath {
    type Item = Direction;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
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

fn init_keypad_shortest_path(control_pad: &PadLayout) -> HashMap<RouteKey, Route> {
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

    paths
}

fn init_controlpad_shortest_path() -> HashMap<RouteKey, Route> {
    let pads = PadLayout::new_control_pad();

    let mut paths = HashMap::<RouteKey, Route>::new();

    for from in 0..pads.layout.len() {
        for to in 0..pads.layout.len() {
            let mut processing = vec![false; pads.layout.len()];
            let _ = init_shortest_path_internal(from, to, &pads, &mut paths, &mut processing);
        }
    }

    paths
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

        paths.entry(Route::of_key(from, neighbor)).insert_entry(Route::new(&[d.clone()], index.abs_diff(index_from)));

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
                path.insert(0, d);
                shortest_path = Some(path);
            }
            (Some(shortest), Some(path0)) if shortest.len() > path0.path.len() + 1 => {
                let mut path = path0.path.clone();
                path.insert(0, d);
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

fn find_keypad_path(pattern: &str, shortest_keys: &HashMap<RouteKey, Route>, shortest_controls: &HashMap<RouteKey, Route>, depth: usize) -> Vec<Action> {
    let mut paths = vec![];
    let mut last_arms = vec!['A'; depth + 1];

    for key in pattern.chars() {
        if let Some(route) = shortest_keys.get(&Route::of_key(last_arms[depth], key)) {
            match depth > 0 {
                true => {
                    route.path.iter()
                        .for_each(|d| {
                            find_control_path(d.to_control_pad(), Action::MoveControl(d.clone()), shortest_controls, depth, &mut last_arms, &mut paths);
                        })
                    ;
                    
                    find_control_path('A', Action::Push, shortest_controls, depth, &mut last_arms, &mut paths);
                },
                false => {
                    paths.push(Action::MoveNum(route.path.clone()));
                    paths.push(Action::Push);
                }
            }
        }

        last_arms[depth] = key;
    }

    paths
}

fn find_control_path(to: char, action: Action, shortest_controls: &HashMap<RouteKey, Route>, depth: usize, last_arms: &mut [char], paths: &mut Vec<Action>) {
    match depth > 0 {
        true => {
            find_control_path_internal(to, shortest_controls, depth - 1, last_arms, paths);
        }
        false => {
            paths.push(action);
        }
    }
}

fn find_control_path_internal(to: char, shortest_controls: &HashMap<RouteKey, Route>, depth: usize, last_arms: &mut [char], paths: &mut Vec<Action>) {
    if last_arms[depth] == to {
        return find_control_path('A', Action::Push, shortest_controls, depth, last_arms, paths);
    }
    
    let route = match shortest_controls.get(&Route::of_key(last_arms[depth], to)) {
        Some(route) => {
            route.path.iter().map(|d| (d.to_control_pad(), Action::MoveControl(d.clone()))).collect()
        }
        None => vec![]
    };

    for (arm, action) in route {
        find_control_path(arm, action, shortest_controls, depth, last_arms, paths);
    }

    find_control_path('A', Action::Push, shortest_controls, depth, last_arms, paths);
    last_arms[depth] = to;
}

#[allow(unused)]
fn dump_route(actions: &[Action]) {
    for action in actions {
        match action {
            Action::MoveNum(vec) => {
                vec.iter().for_each(|a| match a {
                    Direction::N => eprint!("^"),
                    Direction::E => eprint!(">"),
                    Direction::S => eprint!("v"),
                    Direction::W => eprint!("<"),
                })
            }
            Action::MoveControl(d) => match d {
                Direction::N => eprint!("^"),
                Direction::E => eprint!(">"),
                Direction::S => eprint!("v"),
                Direction::W => eprint!("<"),
            }
            Action::Push => eprint!("A"),
        }
    }
    eprintln!()
}

fn eval_route_len(pattern: &str, shortest_keys: &HashMap<RouteKey, Route>, shortest_controls: &HashMap<RouteKey, Route>, depth: usize) -> (usize, usize) {
    let route = find_keypad_path(pattern, shortest_keys, shortest_controls, depth);

    dump_route(&route);
    (
        route.len(),
        pattern.trim_end_matches('A').parse::<usize>().unwrap(),
    )
}

#[cfg(test)]
mod tests {
    use crate::*;
    use assert_unordered::*;

    #[test]
    fn solve_example() -> Result<(), Box<dyn std::error::Error>> {
        assert_eq!(126384, solve("./aoc_input_example.txt")?);
        Ok(())
    }

    #[test]
    fn read_file_example() -> Result<(), Box<dyn std::error::Error>> {
        let sequences = read_file("./aoc_input_example.txt")?;
        let expect_sequences = vec!["029A", "980A", "179A", "456A", "379A"];
        assert_eq!(expect_sequences, sequences);
        Ok(())
    }

    #[test]
    fn shortest_keypad_path() -> Result<(), Box<dyn std::error::Error>> {
        let control_pad = PadLayout::new_control_pad();
        let shortest_path = init_keypad_shortest_path(&control_pad);
        assert_eq_unordered!(&vec![Direction::N, Direction::W], &shortest_path[&Route::of_key('5', '7')].path);
        assert_eq_unordered!(&vec![Direction::E, Direction::S], &shortest_path[&Route::of_key('7', '5')].path);
        assert_eq_unordered!(&vec![Direction::E, Direction::E], &shortest_path[&Route::of_key('7', '9')].path);
        assert_eq_unordered!(&vec![Direction::N, Direction::N, Direction::W, Direction::W], &shortest_path[&Route::of_key('3', '7')].path);
        assert_eq_unordered!(&vec![Direction::E, Direction::E, Direction::S, Direction::S], &shortest_path[&Route::of_key('7', '3')].path);
        assert_eq_unordered!(&vec![Direction::E, Direction::E, Direction::S, Direction::S, Direction::S], &shortest_path[&Route::of_key('7', 'A')].path);

        Ok(())
    }

    #[test]
    fn shortest_controlpad_path() -> Result<(), Box<dyn std::error::Error>> {
        let shortest_path = init_controlpad_shortest_path();
        assert_eq_unordered!(&vec![Direction::E, Direction::S], &shortest_path[&Route::of_key('^', '>')].path);
        assert_eq_unordered!(&vec![Direction::N, Direction::W], &shortest_path[&Route::of_key('>', '^')].path);
        assert_eq_unordered!(&vec![Direction::S, Direction::W, Direction::W], &shortest_path[&Route::of_key('A', '<')].path);
        assert_eq_unordered!(&vec![Direction::E, Direction::E, Direction::N], &shortest_path[&Route::of_key('<', 'A')].path);

        Ok(())
    }

    #[test]
    #[allow(non_snake_case)]
    fn find_keypad_route_depth_0() -> Result<(), Box<dyn std::error::Error>> {
        let control_pad = PadLayout::new_control_pad();
        let shortest_keys = init_keypad_shortest_path(&control_pad);
        let shortest_controls = init_controlpad_shortest_path();

        let paths = find_keypad_path("029A", &shortest_keys, &shortest_controls, 0);
        let expected_paths = vec![
            Action::MoveNum(vec![Direction::W]), Action::Push,
            Action::MoveNum(vec![Direction::N]), Action::Push,
            Action::MoveNum(vec![Direction::E, Direction::N, Direction::N]), Action::Push,
            Action::MoveNum(vec![Direction::S, Direction::S, Direction::S]), Action::Push,
        ];
        // <A^A>^^AvvvA

        assert_eq!(expected_paths.len(), paths.len());
        assert_eq_unordered!(expected_paths[0].moving_paths().collect::<Vec<_>>(), paths[0].moving_paths().collect());
        assert_eq!(expected_paths[1], paths[1]);
        assert_eq_unordered!(&expected_paths[2].moving_paths().collect::<Vec<_>>(), &paths[2].moving_paths().collect());
        assert_eq!(&expected_paths[3], &paths[3]);
        assert_eq_unordered!(&expected_paths[4].moving_paths().collect::<Vec<_>>(), &paths[4].moving_paths().collect());
        assert_eq!(&expected_paths[5], &paths[5]);
        assert_eq_unordered!(&expected_paths[6].moving_paths().collect::<Vec<_>>(), &paths[6].moving_paths().collect());
        assert_eq!(&expected_paths[7], &paths[7]);

        Ok(())
    }

    #[test]
    fn find_keypad_route_to_1_depth_0() -> Result<(), Box<dyn std::error::Error>> {
        let control_pad = PadLayout::new_control_pad();
        let shortest_keys = init_keypad_shortest_path(&control_pad);
        let shortest_controls = init_controlpad_shortest_path();

        let paths = find_keypad_path("1", &shortest_keys, &shortest_controls, 0);
        let expected_paths = vec![
            Action::MoveNum(vec![Direction::N, Direction::W, Direction::W]), Action::Push,
        ];
        // ^<<A

        assert_eq!(expected_paths.len(), paths.len());
        assert_eq_unordered!(expected_paths[0].moving_paths().collect::<Vec<_>>(), paths[0].moving_paths().collect());
        assert_eq!(expected_paths[1], paths[1]);

        Ok(())
    }

    #[test]
    fn find_keypad_route_to_9_depth_1() -> Result<(), Box<dyn std::error::Error>> {
        let control_pad = PadLayout::new_control_pad();
        let shortest_keys = init_keypad_shortest_path(&control_pad);
        let shortest_controls = init_controlpad_shortest_path();

        let paths = find_keypad_path("9", &shortest_keys, &shortest_controls, 1);
        let expected_paths = vec![
            Action::MoveControl(Direction::W), Action::Push, Action::Push, Action::Push,
            Action::MoveControl(Direction::E), Action::Push,
        ];
        // <AAA>A
        // ^^^A
        
        assert_eq!(expected_paths, paths);
        Ok(())
    }

    #[test]
    fn find_keypad_route_to_1_depth_1() -> Result<(), Box<dyn std::error::Error>> {
        let control_pad = PadLayout::new_control_pad();
        let shortest_keys = init_keypad_shortest_path(&control_pad);
        let shortest_controls = init_controlpad_shortest_path();

        let paths = find_keypad_path("1", &shortest_keys, &shortest_controls, 1);
        let expected_paths = vec![
            Action::MoveControl(Direction::W), Action::Push, 
            Action::MoveControl(Direction::S), Action::MoveControl(Direction::W), Action::Push, Action::Push, 
            Action::MoveControl(Direction::E), Action::MoveControl(Direction::E), Action::MoveControl(Direction::N), Action::Push,
        ];
        // <Av<AA>>^A
        // ^<<A
        
        assert_eq!(expected_paths, paths);
        Ok(())
    }

    #[test]
    fn find_keypad_route_to_9_depth_2() -> Result<(), Box<dyn std::error::Error>> {
        let control_pad = PadLayout::new_control_pad();
        let shortest_keys = init_keypad_shortest_path(&control_pad);
        let shortest_controls = init_controlpad_shortest_path();

        let paths = find_keypad_path("9", &shortest_keys, &shortest_controls, 2);
        let expected_paths = vec![
            Action::MoveControl(Direction::S), Action::MoveControl(Direction::W), Action::MoveControl(Direction::W), Action::Push, 
            Action::MoveControl(Direction::E), Action::MoveControl(Direction::E), Action::MoveControl(Direction::N), Action::Push, Action::Push, Action::Push,
            Action::MoveControl(Direction::S), Action::Push,
            Action::MoveControl(Direction::N), Action::Push,
        ];
        // v<<A>>^AAAvA^A
        // <AAA>A
        // ^^^A
        
        assert_eq!(expected_paths, paths);
        Ok(())
    }

    #[test]
    fn find_keypad_route_to_1_depth_7() -> Result<(), Box<dyn std::error::Error>> {
        let control_pad = PadLayout::new_control_pad();
        let shortest_keys = init_keypad_shortest_path(&control_pad);
        let shortest_controls = init_controlpad_shortest_path();

        let paths = find_keypad_path("7", &shortest_keys, &shortest_controls, 2);
        let expected_paths = vec![
            Action::MoveControl(Direction::S), Action::MoveControl(Direction::W), Action::MoveControl(Direction::W), Action::Push, 
            Action::MoveControl(Direction::E), Action::MoveControl(Direction::E), Action::MoveControl(Direction::N), Action::Push, Action::Push, Action::Push,
            Action::MoveControl(Direction::S), Action::MoveControl(Direction::W), Action::Push, 
            Action::MoveControl(Direction::W), Action::Push, 
            Action::MoveControl(Direction::E), Action::MoveControl(Direction::E), Action::MoveControl(Direction::N), Action::Push, Action::Push,
            Action::MoveControl(Direction::S), Action::Push, Action::Push,
            Action::MoveControl(Direction::N), Action::MoveControl(Direction::W), Action::Push,
            Action::MoveControl(Direction::E), Action::Push,
        ];
        // v<<A>>^AAAv<A<A>>^AAvAA^<A>A
        // <AAAv<AAA>>^A
        // ^^^<<<A

        dump_route(&paths);
        
        assert_eq!(expected_paths, paths);
        Ok(())
    }

    #[test]
    fn eval_route_len_example() -> Result<(), Box<dyn std::error::Error>> {
        let control_pad = PadLayout::new_control_pad();
        let shortest_keys = init_keypad_shortest_path(&control_pad);
        let shortest_controls = init_controlpad_shortest_path();

        assert_eq!((68, 29),  eval_route_len("029A", &shortest_keys, &shortest_controls, 2));
        // <v<A>>^AAAvA^A<vA<AA>>^AvAA<^A>A<v<A>A>^AAAvA<^A>A<vA>^A<A>A
        // v<<A>>^AAAvA^Av<A<AA>>^AvAA^<A>Av<A<A>>^AAAvA^<A>Av<A>^A<A>A
        assert_eq!((60, 980), eval_route_len("980A", &shortest_keys, &shortest_controls, 2));
        // <v<A>>^A<vA<A>>^AAvAA<^A>A<v<A>>^AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A
        // v<<A>>^Av<A<A>>^AAvAA^<A>Av<<A>>^AAvA^Av<A>^AA<A>Av<A<A>>^AAAvA^<A>A
        assert_eq!((68, 179), eval_route_len("179A", &shortest_keys, &shortest_controls, 2));
        assert_eq!((64, 456), eval_route_len("456A", &shortest_keys, &shortest_controls, 2));
        // <v<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A
        // v<<A>>^AvA^Av<<A>>^AAv<A<A>>^AAvAA^<A>Av<A>^AA<A>Av<A<A>>^AAAvA^<A>A
        assert_eq!((64, 379), eval_route_len("379A", &shortest_keys, &shortest_controls, 2));
        Ok(())
    }

    #[test]
    fn eval_route_len_example_part_2() -> Result<(), Box<dyn std::error::Error>> {
        let control_pad = PadLayout::new_control_pad();
        let shortest_keys = init_keypad_shortest_path(&control_pad);
        let shortest_controls = init_controlpad_shortest_path();

        assert_eq!((68, 29),  eval_route_len("0", &shortest_keys, &shortest_controls, 25));

        Ok(())
    }
}
