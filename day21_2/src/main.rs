use std::{collections::{HashMap, HashSet}, fs::File, hash::BuildHasherDefault, io::{BufRead, BufReader}, path::Path, u64};

type StableHashMap<K, V> = HashMap<K, V, std::hash::BuildHasherDefault<std::hash::DefaultHasher>>;
type StableHashSet<V> = HashSet<V, BuildHasherDefault<std::hash::DefaultHasher>>;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("total: {:?}", solve("./aoc_input.txt")?);
    Ok(())
}

fn solve<P>(path: P) -> Result<u64, Box<dyn std::error::Error>>
    where P: AsRef<Path>
{
    let sequences = read_file(path)?;
    
    let shortest_keys = init_keypad_shortest_path();
    let shortest_controls = init_controlpad_shortest_path();
    
    let total = sequences.into_iter()
    // .map(|seq| eval_route_len::<26>(&seq, &shortest_keys, &shortest_controls).unwrap())
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
        }
    }

    fn from_index(&self, index: usize) -> Point {
        (index % self.width, index / self.width)
    }

    fn to_index(&self, (x, y): Point) -> usize {
        x + y * self.width
    }

    fn cost(&self, actions: &[Action]) -> usize {
        let mut cost = 0;

        for pair in actions.windows(2) {
            match (&pair[0], &pair[1]) {
                (Action::Move(d1), Action::Move(d2)) => {
                    cost += 1;
                    if !Direction::is_straight(&d1, &d2) {
                        cost += 10;
                    }
                }
                _ => {}
            }
        }

        cost
    }

    fn distance(&self, index_1: usize, index_2: usize) -> usize {
        let (x0, y0) = self.from_index(index_1);
        let (x1, y1) = self.from_index(index_2);

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

    fn is_straight(d1: &Direction, d2: &Direction) -> bool {
        match (d1, d2) {
            (Direction::E, Direction::E) | (Direction::E, Direction::W) => true,
            (Direction::N, Direction::N) | (Direction::S, Direction::S) => true,
            _ => false,
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

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct Route {
    actions: Vec<Action>,
    distance: usize,
}

impl Route {
    fn of_key(from: char, to: char) -> RouteKey {
        (from, to)
    }

    fn new(path: &[Action]) -> Self {
        Self { 
            actions: Vec::from(path),
            distance: path.len(),
        }
    }

    fn prepend_action_path(&mut self, action: Action) {
        self.actions.insert(0, action);
        self.distance += 1;
    }
}

fn init_keypad_shortest_path() -> StableHashMap<RouteKey, Vec<String>> {
    let pads = PadLayout::new_key_pad();

    let mut paths = StableHashMap::<RouteKey, Vec<Route>>::default();

    for (index_from, _) in pads.layout.iter().enumerate() {
        for (index_to, _) in pads.layout.iter().enumerate() {
            let mut processing = vec![false; pads.layout.len()];

            init_shortest_path_internal(index_from, index_to, &pads, &mut paths, &mut processing);
        }
    }

    format_shortest_route(&pads, &paths)
}

fn init_controlpad_shortest_path() -> StableHashMap<RouteKey, Vec<String>> {
    let pads = PadLayout::new_control_pad();

    let mut paths = StableHashMap::<RouteKey, Vec<Route>>::default();

    for (index_from, from) in pads.layout.iter().enumerate() {
        for (index_to, to) in pads.layout.iter().enumerate() {
            let mut processing = vec![false; pads.layout.len()];

            if index_from == index_to {
                match (from, to) {
                    (Some(from), Some(to)) => {
                        paths.entry((*from, *to))
                        .or_insert_with(|| Vec::<Route>::from_iter([Route::new(&vec![])]));
                    }
                    _ => {}
                }
            }

            let _ = init_shortest_path_internal(index_from, index_to, &pads, &mut paths, &mut processing);
        }
    }

    format_shortest_route(&pads, &paths)
}

fn init_shortest_path_internal<'a>(index_from: usize, index_to: usize, pads: &PadLayout, paths: &mut StableHashMap<RouteKey, Vec<Route>>, processing: &mut [bool]) -> Option<Vec<Route>> {
    let Some(from) = pads.layout[index_from] else {
        return None;
    };
    let Some(to) = pads.layout[index_to] else {
        return None;
    };
    if let Some(routes) = paths.get(&Route::of_key(from, to)) {
        if processing[index_from] {
            return Some(routes.clone());
        }
    }
    if from == to {
        paths.insert(Route::of_key(from, to), Vec::<Route>::from_iter([Route::new(&[])]));
        return None;
    }
    if processing[index_from] {
        return None;
    }
    processing[index_from] = true;

    let p0 = pads.from_index(index_from);

    for d in Direction::iter() {
        let Some(p) = d.next(p0, pads.width, pads.height) else {
            continue;
        };

        let index = pads.to_index(p);
        let Some(neighbor) = pads.layout[index] else {
            continue;
        };

        let route = Route::new(&[Action::Move(d.clone())]);
        paths.entry(Route::of_key(from, neighbor))
            .or_insert_with(|| Vec::from_iter([route]))
        ;

        let internal_route = match paths.get(&Route::of_key(neighbor, to)) {
            Some(routes) => Some(routes.clone()),
            None => init_shortest_path_internal(index, index_to, pads, paths, processing) 
        };

        if let Some(routes) = internal_route {
            if pads.distance(index, index_to) + 1 == pads.distance(index_from, index_to) {
                paths.entry(Route::of_key(from, to))
                    .and_modify(|candidates| {
                        routes.iter()
                            .cloned()
                            .map(|mut route| {
                                route.prepend_action_path(Action::Move(d.clone()));
                                route
                            })
                            .for_each(|route| candidates.push(route))
                        ;
                    })
                    .or_insert_with(|| {
                        routes.iter()
                            .cloned()
                            .map(|mut route| {
                                route.prepend_action_path(Action::Move(d.clone()));
                                route
                            })
                            .collect::<Vec<_>>()
                    })
                ;
            }
        }
    }

    paths.get(&Route::of_key(from, to)).and_then(|routes| Some(routes.clone()))
}

fn format_shortest_route(pads: &PadLayout, paths: &StableHashMap<RouteKey, Vec<Route>>) -> StableHashMap<RouteKey, Vec<String>> {
    let mut results = StableHashMap::default();

    for (key, routes) in paths {
        let mut candidates = StableHashSet::<(String, usize)>::default();
        let mut min_cost = usize::MAX;

        for route in routes {
            let cost = pads.cost(&route.actions);
            min_cost = usize::min(cost, min_cost);

            let candidate = route.actions.iter()
                .map(|path| match path {
                    Action::Move(d) => d.to_control_pad(),
                    Action::Push => 'A',
                })
                .collect::<String>()
            ;
            candidates.insert((candidate, cost));
        }

        results.entry(key.clone()).or_insert_with(|| {
            candidates.into_iter()
                .filter(|(_, cost)| *cost == min_cost)
                .map(|(route, _)| route)
                .collect::<Vec<_>>()
        });
    }

    results
}

#[allow(unused)]
fn dump_shortest_routes(tag: &str, routes: &StableHashMap<RouteKey, Vec<String>>) {
    eprintln!(">>>> SHORTEST {tag}");

    for (key, route) in routes {
        eprintln!("[{:<8?}] {:?}", key, route);
    }
}

#[derive(Debug)]
enum EvalError {
    RouteNotFound(char, char),
}
impl std::fmt::Display for EvalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EvalError::RouteNotFound(from, to) => write!(f, "Route not found: ({from} -> {to})"),
        }
    }
}
impl std::error::Error for EvalError {}

struct Cache<const N: usize> {
    entries: Vec<StableHashMap<RouteKey, CacheEntry>>,
}
impl<const N: usize> Cache<N> {
    fn new() -> Self {
        Self {
            entries: vec![StableHashMap::<RouteKey, CacheEntry>::default(); N],
        }
    }

    fn has_entry(&self, route: &RouteKey, depth: usize) -> bool {
        self.entries[depth].contains_key(route)
    }

    fn get_mut<'a>(& 'a mut self, route: &RouteKey, depth: usize) -> &'a mut CacheEntry {
        self.entries[depth].entry(route.clone()).or_insert_with(|| CacheEntry::new(route))    
    }
}

#[derive(Clone)]
#[allow(unused)]
struct CacheEntry {
    route: RouteKey,
    items: StableHashMap<RouteKey, u64>,
}
impl CacheEntry {
    fn new(route: &RouteKey) -> Self {
        Self {
            route: route.clone(),
            items: StableHashMap::<RouteKey, u64>::default(),
        }
    }

    fn total_len(&self) -> u64 {
        total_length(&self.items)
    }

    fn merge(&mut self, history: &StableHashMap<RouteKey, u64>, count: u64) {
        merge_history(history, &mut self.items, count);
    }
}

fn total_length(history: &StableHashMap<(char, char), u64>) -> u64 {
    history.into_iter()
    .map(|(_, count)| *count)
    .sum::<u64>()
}

fn merge_history(from: &StableHashMap<(char, char), u64>, to: &mut StableHashMap<(char, char), u64>, rate: u64) {
    for (k, v) in from {
        to.entry(k.clone()).and_modify(|c| {
            *c += *v / rate
        })
        .or_insert_with(|| {
            *v / rate
        });
    }
}

fn input_numbers(route: &str) -> StableHashMap<RouteKey, u64> {
    let mut history = StableHashMap::<RouteKey, u64>::default();
    let mut last_state = 'A';

    for pair in format!("A{route}").chars().collect::<Vec<_>>().windows(2) {
        history.entry(Route::of_key(pair[0], pair[1]))
            .and_modify(|c| *c += 1)
            .or_insert_with(|| 1)
        ;
        last_state = pair[1];
    }
    history.entry(Route::of_key(last_state, 'A'))
        .and_modify(|c| *c += 1)
        .or_insert_with(|| 1)
    ;
    history
}

fn input_cursors<const N: usize>(shortest_routes: &StableHashMap<RouteKey, Vec<String>>, depth: usize, prev_history: &StableHashMap<(char, char), u64>, cache: &mut Cache<N>) -> Result<(u64, StableHashMap<(char, char), u64>), Box<dyn std::error::Error>> {
    if depth == 0 {
        return Ok((total_length(prev_history), prev_history.clone()));
    }

    let mut total_len: u64 = 0;
    let mut shortest_history = StableHashMap::<(char, char), u64>::default();

    for ((from, to), count0) in prev_history.iter() {
        let Some(candidates) = shortest_routes.get(&&Route::of_key(*from, *to)) else {
            return Err(Box::new(EvalError::RouteNotFound(*from, *to)));
        };

        let mut current_history = None;
        let route_key = Route::of_key(*from, *to);

        if cache.has_entry(&route_key, depth) {
            let cache_entry = cache.get_mut(&route_key, depth);
            current_history = Some(cache_entry.items.clone());

            total_len += cache_entry.total_len() * *count0;
        }
        else {
            let mut shortest_len: u64 = u64::MAX;

            for route in candidates {
                let mut candidate_history = StableHashMap::<(char, char), u64>::default();

                for pair in format!("A{}A", route).chars().collect::<Vec<_>>().windows(2) {
                    candidate_history.entry(Route::of_key(pair[0], pair[1]))
                        .and_modify(|c| *c += *count0)
                        .or_insert_with(|| *count0)
                    ;
                }
    
                let (len, history) = input_cursors::<N>(shortest_routes, depth - 1, &candidate_history, cache)?;

                if len < shortest_len {
                    current_history = Some(history);
                    shortest_len = len;
                }
            }

            if let Some(current_history) = current_history.as_ref() {
                let cache_entry = cache.get_mut(&route_key, depth);
                cache_entry.merge(current_history, *count0);
            }
            
            total_len += shortest_len
        }

        if let Some(history) = current_history {
            merge_history(&history, &mut shortest_history, 1);
        }
    }

    Ok((total_len, shortest_history))
}

fn eval_route_len_internal<const N: usize>(code: &str, key_pad_routes: &StableHashMap<RouteKey, Vec<String>>, control_pad_routes: &StableHashMap<RouteKey, Vec<String>>) -> Result<(u64, u64, StableHashMap<RouteKey, u64>), Box<dyn std::error::Error>> {
    let mut total_len: u64 = 0;
    let mut shortest_history = StableHashMap::<(char, char), u64>::default();
    let mut cache = Cache::<N>::new();

    for pair in format!("A{code}").chars().collect::<Vec<_>>().windows(2) {
        let route_key = Route::of_key(pair[0], pair[1]);
        let Some(candidates) = key_pad_routes.get(&route_key) else {
            return Err(Box::new(EvalError::RouteNotFound(pair[0], pair[1])));
        };

        let mut shortest_len: u64 = u64::MAX;
        let mut current_history = None;

        for route in candidates {
            let candidate_history = input_numbers(&route);

            let (len, history) = match N < 1 {
                true => {
                    (total_length(&candidate_history), candidate_history)
                }
                false => {
                    input_cursors(&control_pad_routes, N - 1, &candidate_history, &mut cache)?
                }
            };
            if len < shortest_len {
                shortest_len = len;
                current_history = Some(history);
            }
        }

        if let Some(current_history) = current_history {
            merge_history(&current_history, &mut shortest_history, 1);
        }

        // eprintln!("{} [{}] ch: {} len: {}", code, N, pair[1], shortest_len);
        total_len += shortest_len;
    }

    Ok((total_len, code.trim_end_matches('A').parse::<u64>()?, shortest_history))
}

fn eval_route_len<const N: usize>(code: &str, key_pad_routes: &StableHashMap<RouteKey, Vec<String>>, control_pad_routes: &StableHashMap<RouteKey, Vec<String>>) -> Result<(u64, u64), Box<dyn std::error::Error>> {
    let (shortest_len, value, _) = eval_route_len_internal::<N>(code, key_pad_routes, control_pad_routes)?;
    // eprintln!("{code}: {shortest_len}");
    Ok((shortest_len, value))
}

#[allow(unused)]
fn dump_history(history: &StableHashMap<RouteKey, u64>) {
    for (key, count) in history {
        eprintln!("[{:<8?}] {:?}", key, count);
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn resolve_part_1() -> Result<(), Box<dyn std::error::Error>> {
        const DEPTH: usize = 3;

        let shortest_keys = init_keypad_shortest_path();
        let shortest_controls = init_controlpad_shortest_path();

        assert_eq!((68, 29),  eval_route_len::<DEPTH>("029A", &shortest_keys, &shortest_controls)?);
        assert_eq!((60, 980), eval_route_len::<DEPTH>("980A", &shortest_keys, &shortest_controls)?);
        assert_eq!((68, 179), eval_route_len::<DEPTH>("179A", &shortest_keys, &shortest_controls)?);
        assert_eq!((64, 456), eval_route_len::<DEPTH>("456A", &shortest_keys, &shortest_controls)?);
        assert_eq!((64, 379), eval_route_len::<DEPTH>("379A", &shortest_keys, &shortest_controls)?);
        Ok(())
    }

    #[test]
    fn resolve_part_2() -> Result<(), Box<dyn std::error::Error>> {
        const DEPTH: usize = 26;

        let shortest_keys = init_keypad_shortest_path();
        let shortest_controls = init_controlpad_shortest_path();

        assert_eq!((84248089342, 340), eval_route_len::<DEPTH>("340A", &shortest_keys, &shortest_controls)?);
        assert_eq!((91059074548, 149), eval_route_len::<DEPTH>("149A", &shortest_keys, &shortest_controls)?);
        assert_eq!((86475783008, 582), eval_route_len::<DEPTH>("582A", &shortest_keys, &shortest_controls)?);
        assert_eq!((80786362260, 780), eval_route_len::<DEPTH>("780A", &shortest_keys, &shortest_controls)?);
        assert_eq!((87288844796, 463), eval_route_len::<DEPTH>("463A", &shortest_keys, &shortest_controls)?);
        Ok(())
    }
}

// (x) 195969155895596
// (x) 780A [26] 80786362255

// 780A [26] ch: 7 len: 31420065369 (-3) ^^^<< <^^^<
// 780A [26] ch: 8 len: 14287938116
// 780A [26] ch: 0 len: 20790420654 (-2)
// 780A [26] ch: A len: 14287938116

// 340A: 84248089342
// 149A: 91059074548
// 582A: 86475783008
// (x) 780A [26] 80786362255
// (x) 780A [25] 32475283856
// (o)780A: 80786362260
// 463A: 87288844796

// 340: 84248089342
// 149: 91059074548
// 582: 86475783008

// 780 [26] 80786362260

// 31420065372 ^^^<<A
// 14287938116
// 20790420656 vvvA>A
// 14287938116

// 780 [25] 32475283856

// 463: 87288844796

// 195969155897936
// 195969155897936