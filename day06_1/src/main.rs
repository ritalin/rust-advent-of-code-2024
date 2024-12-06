use std::{fs::File, io::{BufRead, BufReader}, path::Path};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("total: {}", solve("./aoc_input.txt")?);
    Ok(())
}

fn solve<P>(path: P) -> Result<usize, Box<dyn std::error::Error>>
    where P: AsRef<Path>
{
    let (width, map) = read_file(path)?;
    let mut guard = find_guard(&map)?;

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

    Ok(trace.count())
}

type Map = Vec<Vec<u8>>;

#[derive(PartialEq, Eq, Debug)]
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
}

impl Trace {
    fn new(width: usize, height: usize, x: i32, y: i32) -> Self {
        let mut this = Self {
            marks: std::iter::repeat(false).take(height * width).collect::<Vec<_>>(),
            width,
        };
        this.mark(x as usize, y as usize);

        this
    }
    fn mark(&mut self, x: usize, y: usize) {
        self.marks[x + y * self.width] = true;
    }

    fn count(&self) -> usize {
        self.marks.iter().filter(|m| **m).count()
    }
}

#[derive(Debug)]
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

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn solve_example() -> Result<(), Box<dyn std::error::Error>> {
        let actual = crate::solve("./aoc_input_example.txt")?;
        assert_eq!(41, actual);
        Ok(())
    }

    #[test]
    fn read_file_example() -> Result<(), Box<dyn std::error::Error>> {
        let (width, map) = read_file("./aoc_input_example.txt")?;

        let expect_map = vec![
            vec![b'.', b'.', b'.', b'.', b'#', b'.', b'.', b'.', b'.', b'.', ],
            vec![b'.', b'.', b'.', b'.', b'.', b'.', b'.', b'.', b'.', b'#', ],
            vec![b'.', b'.', b'.', b'.', b'.', b'.', b'.', b'.', b'.', b'.', ],
            vec![b'.', b'.', b'#', b'.', b'.', b'.', b'.', b'.', b'.', b'.', ],
            vec![b'.', b'.', b'.', b'.', b'.', b'.', b'.', b'#', b'.', b'.', ],
            vec![b'.', b'.', b'.', b'.', b'.', b'.', b'.', b'.', b'.', b'.', ],
            vec![b'.', b'#', b'.', b'.', b'^', b'.', b'.', b'.', b'.', b'.', ],
            vec![b'.', b'.', b'.', b'.', b'.', b'.', b'.', b'.', b'#', b'.', ],
            vec![b'#', b'.', b'.', b'.', b'.', b'.', b'.', b'.', b'.', b'.', ],
            vec![b'.', b'.', b'.', b'.', b'.', b'.', b'#', b'.', b'.', b'.', ],
        ];

        assert_eq!(expect_map, map);
        assert_eq!(10, width);

        let g = find_guard(&map)?;
        assert_eq!(Direction::WEST, g.direction);
        assert_eq!((4, 6), (g.x, g.y));
        Ok(())
    }

    #[test]
    fn next_direction() -> Result<(), Box<dyn std::error::Error>> {
        let d = Direction::NORTH;
        
        let d = d.next();
        assert_eq!(Direction::EAST, d);
        let d = d.next();
        assert_eq!(Direction::SOUTH, d);
        let d = d.next();
        assert_eq!(Direction::WEST, d);
        let d = d.next();
        assert_eq!(Direction::NORTH, d);

        Ok(())
    }

    #[test]
    fn move_guard() -> Result<(), Box<dyn std::error::Error>> {
        let (width, map) = read_file("./aoc_input_example.txt")?;
        let mut g = find_guard(&map)?;
        let mut trace = Trace::new(width, map.len(), g.x, g.y);
        trace.mark(g.x as usize, g.y as usize);

        assert_eq!(Direction::WEST, g.direction);

        let (next_g, trace) = g.patrol(&map, &trace);
        assert!(next_g.is_some());
        let mut g = next_g.unwrap();
        assert_eq!(Direction::NORTH, g.direction);
        assert_eq!((4, 1), (g.x, g.y));

        let (next_g, trace) = g.patrol(&map, &trace);
        assert!(next_g.is_some());
        let g = next_g.unwrap();
        assert_eq!(Direction::EAST, g.direction);
        assert_eq!((8, 1), (g.x, g.y));

        assert_eq!(10, trace.count());
        Ok(())
    }
}
