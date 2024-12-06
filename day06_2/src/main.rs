use std::{collections::HashSet, fs::File, io::{BufRead, BufReader}, path::Path};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("total: {}", solve("./aoc_input.txt")?);
    Ok(())
}

fn solve<P>(path: P) -> Result<usize, Box<dyn std::error::Error>>
    where P: AsRef<Path>
{
    let (width, map) = read_file(path)?;
    let guard = find_guard(&map)?;
    let routes = record_route(guard.clone(), &map, width);

    let mut map = map;

    let total = routes.into_iter()
        .filter_map(|(x, y)| {
            map[y][x] = b'#';
            let result = replay(guard.clone(), &map, width);
            map[y][x] = b'.';
            
            match result {
                Status::Escape => None,
                Status::Stack => {
                    // eprintln!("(x, y) = ({}, {})", x, y);
                    Some(1)
                },
            }
        })
        .count()
    ;

    Ok(total)
}

type Map = Vec<Vec<u8>>;

#[derive(PartialEq, Eq, Clone, Debug)]
enum Direction {
    NORTH,
    EAST,
    SOUTH,
    WEST,
}

impl Direction {
    fn next(&self) -> Self {
        match self {
            Direction::NORTH => Direction::EAST,
            Direction::EAST => Direction::SOUTH,
            Direction::SOUTH => Direction::WEST,
            Direction::WEST => Direction::NORTH,   
        }
    }
}

#[derive(Clone, Debug)]
struct Trace {
    marks: Vec<bool>,
    width: usize,
    start_x: usize,
    start_y: usize,
}

impl Trace {
    fn new(width: usize, height: usize, x: i32, y: i32) -> Self {
        Self {
            marks: std::iter::repeat(false).take(height * width).collect::<Vec<_>>(),
            width,
            start_x: x as usize,
            start_y: y as usize,
        }
    }

    fn mark(&mut self, x: usize, y: usize) {
        if (x != self.start_x) || (y != self.start_y) {
            self.marks[x + y * self.width] = true;
        }
    }
}

#[derive(Clone, Debug)]
struct Guard {
    direction: Direction,
    x: i32,
    y: i32,
}

impl Guard {
    fn patrol(&mut self, map: &Map, trace: &Trace) -> (Option<Self>, Trace) {
        let mut next_trace = trace.clone();

        let direction = self.direction.next();
        let (diff_x, diff_y) = match direction {
            Direction::NORTH => (0, -1),
            Direction::EAST => (1, 0),
            Direction::SOUTH => (0, 1),
            Direction::WEST => (-1, 0),   
        };

        let height = map.len();

        loop { 
            let (x, y) = (self.x, self.y);
            let (x1, y1) = ((x + diff_x) as usize, (y + diff_y) as usize);

            self.x += diff_x;
            self.y += diff_y;

            if self.out_of_bound(trace.width, height) {
                return (None, next_trace);
            }

            if map[y1][x1] == b'#' {
                return (Some(Self {direction, x, y}), next_trace);
            }

            next_trace.mark(x1, y1);
        }
    }

    fn out_of_bound(&self, width: usize, height: usize) -> bool {
        let x = self.x as usize;
        let y = self.y as usize;

        (self.x < 0) || (x >= width) || (self.y < 0) || (y >= height)
    }
}

#[derive(PartialEq, Eq, Debug)]
enum Status {
    Escape,
    Stack,
}

fn read_file<P>(path: P) -> Result<(usize, Map), Box<dyn std::error::Error>> 
    where P: AsRef<Path>
{
    let reader = BufReader::new(File::open(path)?);

    let (widths, map): (Vec<usize>, Map) = reader.lines()
        .filter_map(|s| s.ok())
        .map(|s| (s.len(), s.into_bytes()))
        .unzip()
    ;

    Ok((widths.into_iter().max().unwrap(), map))
}

fn find_guard(map: &Map) -> Result<Guard, Box<dyn std::error::Error>> {
    for (y, map_row) in map.iter().enumerate() {
        for (x, ch) in map_row.iter().enumerate() {
            if *ch == b'^' {
                return Ok(Guard {
                    direction: Direction::WEST,
                    x: x as i32,
                    y: y as i32,
                });
            }
        }
    }

    Err("Guard not found".to_string().into())
}

fn record_route(mut guard: Guard, map: &Map, width: usize) -> Vec<(usize, usize)> {
    let mut trace = Trace::new(width, map.len(), guard.x, guard.y);

    loop {
        let (next_guard, next_trace) = guard.patrol(&map, &trace);
        trace = next_trace;

        if let Some(g) = next_guard {
            guard = g;
        }
        else {
            break;
        }
    }

    trace.marks.into_iter()
        .enumerate()
        .filter(|(_, m)| *m)
        .map(|(i, _)| (i % trace.width, i / trace.width))
        .collect()
}

fn replay(mut guard: Guard, map: &Map, width: usize) -> Status {
    let mut trace = Trace::new(width, map.len(), guard.x, guard.y);
    let mut check_point = HashSet::<(i32, i32)>::new();
    loop {
        let (next_guard, next_trace) = guard.clone().patrol(&map, &trace);
        trace = next_trace;

        if let Some(g) = next_guard {
            if ((g.x, g.y) != (guard.x, guard.y)) && check_point.contains(&(g.x, g.y)) {
                return Status::Stack;
            }
            guard = g;
            check_point.insert((guard.x, guard.y));
        }
        else {
            return Status::Escape;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn solve_example() -> Result<(), Box<dyn std::error::Error>> {
        let actual = solve("./aoc_input_example.txt")?;
        assert_eq!(6, actual);
        Ok(())
    }

    #[test]
    fn route_example() -> Result<(), Box<dyn std::error::Error>> {
        let (width, map) = read_file("./aoc_input_example.txt")?;
        let guard = find_guard(&map)?;
        let routes = record_route(guard.clone(), &map, width);
        
        let expect_routes = vec![
            (4, 1), (5, 1), (6, 1), (7, 1), (8, 1), 
            (4, 2), (8, 2), 
            (4, 3), (8, 3), 
            (2, 4), (3, 4), (4, 4), (5, 4), (6, 4), (8, 4), 
            (2, 5), (4, 5), (6, 5), (8, 5), 
            (2, 6), (3, 6), (5, 6), (6, 6), (7, 6), (8, 6), 
            (1, 7), (2, 7), (3, 7), (4, 7), (5, 7), (6, 7), (7, 7), 
            (1, 8), (2, 8), (3, 8), (4, 8), (5, 8), (6, 8), (7, 8), 
            (7, 9), 
        ];
        assert_eq!(expect_routes, routes);
        // ...#.....
        // ....XXXXX#
        // ....X...X.
        // ..#.X...X.
        // ..XXXXX#X.
        // ..X.X.X.X.
        // .#XX*XXXX.
        // .XXXXXXX#.
        // #XXXXXXX..
        // ......#X..

        Ok(())
    }

    #[test]
    fn replay_example() -> Result<(), Box<dyn std::error::Error>> {
        let (width, mut map) = read_file("./aoc_input_example.txt")?;
        let guard = find_guard(&map)?;

        let (x, y) = (5, 1);
        map[y][x] = b'#';
        assert_eq!(Status::Escape, replay(guard.clone(), &map, width));
        map[y][x] = b'.';

        let (x, y) = (4, 5);
        map[y][x] = b'#';
        assert_eq!(Status::Escape, replay(guard.clone(), &map, width));
        map[y][x] = b'.';

        let (x, y) = (7, 9);
        map[y][x] = b'#';
        assert_eq!(Status::Stack, replay(guard.clone(), &map, width));
        map[y][x] = b'.';

        Ok(())
    }
}